#!/usr/bin/env python3
from __future__ import annotations

import argparse
import os
import re
import subprocess
import sys
from collections import OrderedDict
from pathlib import Path


def host_arch() -> str:
    if "DEB_HOST_ARCH" in os.environ and os.environ["DEB_HOST_ARCH"]:
        return os.environ["DEB_HOST_ARCH"]

    for command in (["dpkg", "--print-architecture"], ["uname", "-m"]):
        try:
            result = subprocess.run(
                command,
                check=True,
                capture_output=True,
                text=True,
            )
            value = result.stdout.strip()
            if value:
                return value
        except (FileNotFoundError, subprocess.CalledProcessError):
            continue

    raise SystemExit("error: unable to determine host architecture")


def qualifier_matches(qualifier: str, arch: str) -> bool:
    qualifier = qualifier.strip()
    if not qualifier:
        return True
    if qualifier.startswith("arch="):
        return arch in qualifier[len("arch=") :].split()
    return True


def iter_manifest_tokens(
    symbols_file: Path, skip_regex: re.Pattern[str] | None, arch: str
):
    with symbols_file.open(encoding="utf-8") as handle:
        for raw_line in handle:
            line = raw_line.rstrip("\r\n")
            stripped = line.lstrip()

            if not stripped or stripped.startswith("#") or stripped.startswith("*"):
                continue
            if not raw_line[:1].isspace():
                continue

            qualifier = ""
            if stripped.startswith("("):
                close = stripped.find(")")
                if close == -1:
                    continue
                qualifier = stripped[1:close]
                stripped = stripped[close + 1 :].lstrip()

            if not qualifier_matches(qualifier, arch):
                continue

            token = stripped.split(maxsplit=1)[0]
            if "@" not in token:
                continue

            symbol_name = token.split("@", 1)[0]
            if skip_regex and skip_regex.search(symbol_name):
                continue

            yield token


def exported_tokens(library: Path, skip_regex: re.Pattern[str] | None) -> set[str]:
    result = subprocess.run(
        ["objdump", "-T", str(library)],
        check=True,
        capture_output=True,
        text=True,
    )
    tokens: set[str] = set()
    line_re = re.compile(
        r"^(?P<address>[0-9A-Fa-f]+)\s+"
        r"(?:(?P<bind>\w)\s+)?"
        r"(?P<kind>\w+)\s+"
        r"(?P<section>\S+)\s+"
        r"(?P<size>[0-9A-Fa-f]+)\s+"
        r"(?P<version>\S+)\s+"
        r"(?P<symbol>\S+)$"
    )

    for line in result.stdout.splitlines():
        match = line_re.match(line)
        if not match:
            continue
        section = match.group("section")
        version = match.group("version")
        symbol = match.group("symbol")

        if section == "*UND*" or version == "Base":
            continue

        if version.startswith("(") and version.endswith(")"):
            version = version[1:-1]

        if skip_regex and skip_regex.search(symbol):
            continue

        tokens.add(f"{symbol}@{version}")

    return tokens


def cmd_check(args: argparse.Namespace) -> int:
    arch = host_arch()
    skip_regex = re.compile(args.skip_regex) if args.skip_regex else None
    symbols_file = Path(args.symbols_file)
    library = Path(args.shared_library)

    if not symbols_file.is_file():
        raise SystemExit(f"error: missing symbols file: {symbols_file}")
    if not library.exists():
        raise SystemExit(f"error: missing library: {library}")

    manifest = set(iter_manifest_tokens(symbols_file, skip_regex, arch))
    exports = exported_tokens(library, skip_regex)

    missing = sorted(manifest - exports)
    unexpected = sorted(exports - manifest)

    if missing:
        print(
            f"missing {len(missing)} manifest-declared export(s) from {library}:",
            file=sys.stderr,
        )
        for token in missing:
            print(f"  {token}", file=sys.stderr)

    if unexpected:
        print(
            f"found {len(unexpected)} exported symbol(s) in {library} that are not declared in {symbols_file}:",
            file=sys.stderr,
        )
        for token in unexpected:
            print(f"  {token}", file=sys.stderr)

    if missing or unexpected:
        return 1

    if not manifest:
        raise SystemExit(f"error: no manifest symbols selected from {symbols_file}")

    print(
        f"validated {len(manifest)} manifest symbol(s) in {library} against {symbols_file}"
    )
    return 0


def cmd_render_version_script(args: argparse.Namespace) -> int:
    arch = host_arch()
    skip_regex = re.compile(args.skip_regex) if args.skip_regex else None
    symbols_file = Path(args.symbols_file)
    output = Path(args.output)

    if not symbols_file.is_file():
        raise SystemExit(f"error: missing symbols file: {symbols_file}")

    nodes: "OrderedDict[str, list[str]]" = OrderedDict()
    seen_symbols: dict[str, set[str]] = {}

    for token in iter_manifest_tokens(symbols_file, skip_regex, arch):
        symbol, version = token.split("@", 1)
        nodes.setdefault(version, [])
        seen_symbols.setdefault(version, set())
        if symbol == version or symbol in seen_symbols[version]:
            continue
        nodes[version].append(symbol)
        seen_symbols[version].add(symbol)

    if not nodes:
        raise SystemExit(f"error: no version nodes selected from {symbols_file}")

    lines: list[str] = []
    versions = list(nodes.keys())
    last_index = len(versions) - 1

    for index, version in enumerate(versions):
        parent = versions[index - 1] if index else None
        symbols = nodes[version]

        lines.append(f"{version}")
        lines.append("{")
        if symbols:
            lines.append("  global:")
            for symbol in symbols:
                lines.append(f"    {symbol};")
        if index == last_index:
            lines.append("  local:")
            lines.append("    *;")
        lines.append("}" + (f" {parent};" if parent else ";"))
        lines.append("")

    output.parent.mkdir(parents=True, exist_ok=True)
    output.write_text("\n".join(lines), encoding="utf-8")
    return 0


def cmd_rewrite_min_version(args: argparse.Namespace) -> int:
    only_file = Path(args.only_symbols_file)
    symbols_file = Path(args.generated_symbols_file)

    if not only_file.is_file():
        raise SystemExit(f"error: missing symbols subset file: {only_file}")
    if not symbols_file.is_file():
        raise SystemExit(f"error: missing generated symbols file: {symbols_file}")

    tokens: list[str] = []
    for raw_line in only_file.read_text(encoding="utf-8").splitlines():
        line = raw_line.strip()
        if not line or line.startswith("#"):
            continue
        tokens.append(line)

    lines = symbols_file.read_text(encoding="utf-8").splitlines()
    rewritten: list[str] = []
    old_suffix = f" {args.match_version}"
    new_suffix = f" {args.replacement_version}"

    for line in lines:
        updated = line
        if line.rstrip().endswith(old_suffix):
            for token in tokens:
                if token in line:
                    updated = f"{line[:-len(old_suffix)]}{new_suffix}"
                    break
        rewritten.append(updated)

    symbols_file.write_text("\n".join(rewritten) + "\n", encoding="utf-8")
    return 0


def build_parser() -> argparse.ArgumentParser:
    parser = argparse.ArgumentParser(
        description="Parse Debian .symbols files for bootstrap symbol checks and linker scripts."
    )
    subparsers = parser.add_subparsers(dest="command", required=True)

    check = subparsers.add_parser("check")
    check.add_argument("--skip-regex")
    check.add_argument("symbols_file")
    check.add_argument("shared_library")
    check.set_defaults(func=cmd_check)

    render = subparsers.add_parser("render-version-script")
    render.add_argument("--skip-regex")
    render.add_argument("symbols_file")
    render.add_argument("output")
    render.set_defaults(func=cmd_render_version_script)

    rewrite = subparsers.add_parser("rewrite-min-version")
    rewrite.add_argument("--match-version", required=True)
    rewrite.add_argument("--replacement-version", required=True)
    rewrite.add_argument("only_symbols_file")
    rewrite.add_argument("generated_symbols_file")
    rewrite.set_defaults(func=cmd_rewrite_min_version)

    return parser


def main() -> int:
    parser = build_parser()
    args = parser.parse_args()
    return args.func(args)


if __name__ == "__main__":
    raise SystemExit(main())
