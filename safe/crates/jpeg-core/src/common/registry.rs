use std::{
    collections::HashMap,
    sync::{Mutex, OnceLock},
};

use ffi_types::{boolean, int, j_decompress_ptr, FALSE, TRUE};

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DecompressPolicy {
    pub max_scans: Option<int>,
    pub warnings_fatal: bool,
}

fn decode_policies() -> &'static Mutex<HashMap<usize, DecompressPolicy>> {
    static POLICIES: OnceLock<Mutex<HashMap<usize, DecompressPolicy>>> = OnceLock::new();
    POLICIES.get_or_init(|| Mutex::new(HashMap::new()))
}

#[inline]
fn decode_key(cinfo: j_decompress_ptr) -> usize {
    cinfo as usize
}

unsafe fn update_policy(cinfo: j_decompress_ptr, update: impl FnOnce(&mut DecompressPolicy)) {
    if cinfo.is_null() {
        return;
    }

    let key = decode_key(cinfo);
    let mut policies = decode_policies()
        .lock()
        .expect("decode policy mutex poisoned");
    let policy = policies.entry(key).or_default();
    update(policy);
    if *policy == DecompressPolicy::default() {
        policies.remove(&key);
    }
}

pub unsafe fn clear_decompress_policy(cinfo: j_decompress_ptr) {
    if cinfo.is_null() {
        return;
    }
    decode_policies()
        .lock()
        .expect("decode policy mutex poisoned")
        .remove(&decode_key(cinfo));
}

pub unsafe fn get_decompress_policy(cinfo: j_decompress_ptr) -> Option<DecompressPolicy> {
    if cinfo.is_null() {
        return None;
    }
    decode_policies()
        .lock()
        .expect("decode policy mutex poisoned")
        .get(&decode_key(cinfo))
        .copied()
}

pub unsafe fn set_decompress_scan_limit(cinfo: j_decompress_ptr, max_scans: int) {
    update_policy(cinfo, |policy| {
        policy.max_scans = (max_scans > 0).then_some(max_scans);
    });
}

pub unsafe fn set_decompress_warnings_fatal(cinfo: j_decompress_ptr, fatal: boolean) {
    update_policy(cinfo, |policy| {
        policy.warnings_fatal = fatal != FALSE;
    });
}

pub unsafe fn configure_decompress_policy(
    cinfo: j_decompress_ptr,
    max_scans: int,
    warnings_fatal: boolean,
) {
    update_policy(cinfo, |policy| {
        policy.max_scans = (max_scans > 0).then_some(max_scans);
        policy.warnings_fatal = warnings_fatal != FALSE;
    });
}

pub unsafe fn decompress_warnings_fatal(cinfo: j_decompress_ptr) -> bool {
    get_decompress_policy(cinfo)
        .map(|policy| policy.warnings_fatal)
        .unwrap_or(false)
}

pub unsafe fn decompress_scan_limit(cinfo: j_decompress_ptr) -> Option<int> {
    get_decompress_policy(cinfo).and_then(|policy| policy.max_scans)
}

pub unsafe fn decompress_scan_limit_exceeded(cinfo: j_decompress_ptr) -> Option<int> {
    if cinfo.is_null() {
        return None;
    }

    let max_scans = decompress_scan_limit(cinfo)?;
    ((*cinfo).input_scan_number > max_scans).then_some(max_scans)
}

pub unsafe fn warnings_fatal_flag(value: bool) -> boolean {
    if value {
        TRUE
    } else {
        FALSE
    }
}

pub unsafe fn decompress_warnings_fatal_flag(cinfo: j_decompress_ptr) -> boolean {
    warnings_fatal_flag(decompress_warnings_fatal(cinfo))
}
