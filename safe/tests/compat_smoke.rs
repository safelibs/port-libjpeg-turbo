use std::process::Command;

#[test]
fn bootstrap_scripts_are_present_and_helpful() {
    let root = safe::safe_root();
    let stage_install = root.join("scripts/stage-install.sh");
    let relink = root.join("scripts/relink-original-objects.sh");
    let dependents = root.join("scripts/run-dependent-subset.sh");

    for path in [&stage_install, &relink, &dependents] {
        assert!(path.exists(), "missing {:?}", path);
    }

    for script in [stage_install, relink, dependents] {
        let status = Command::new("bash")
            .arg(script)
            .arg("--help")
            .status()
            .expect("failed to run bootstrap script");
        assert!(status.success());
    }
}

