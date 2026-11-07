use std::fs;
use std::panic::{self, AssertUnwindSafe};
use std::path::{Path, PathBuf};

/// `macrotest` expects fixtures named `<case>.rs` next to `<case>.expanded.rs`.
/// Sources are tracked as `<case>.before.rs` so each one sorts before its
/// snapshot in diffs. Mirror both into `target/` under the names macrotest
/// expects, hard-linking snapshots so `MACROTEST=overwrite` still updates the
/// tracked files.
fn expand_cases(case_dir: &str) {
    let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let source_dir = manifest_dir.join("tests/cases").join(case_dir);
    let mirror_dir = manifest_dir.join("target/macrotest-cases").join(case_dir);

    if mirror_dir.exists() {
        fs::remove_dir_all(&mirror_dir).expect("remove stale macrotest mirror");
    }
    fs::create_dir_all(&mirror_dir).expect("create macrotest mirror");

    let mut fixtures = 0;
    for entry in fs::read_dir(&source_dir).expect("read fixture directory") {
        let source = entry.expect("read fixture entry").path();
        let Some(file_name) = source.file_name().and_then(|name| name.to_str()) else {
            continue;
        };
        let Some(case) = file_name.strip_suffix(".before.rs") else {
            continue;
        };

        let snapshot = source_dir.join(format!("{case}.expanded.rs"));
        link_or_copy(&source, &mirror_dir.join(format!("{case}.rs")));
        if snapshot.exists() {
            link_or_copy(&snapshot, &mirror_dir.join(format!("{case}.expanded.rs")));
        }
        fixtures += 1;
    }
    assert!(
        fixtures > 0,
        "no .before.rs fixtures in {}",
        source_dir.display()
    );

    let pattern = mirror_dir.join("*.rs");
    let expansion = panic::catch_unwind(AssertUnwindSafe(|| {
        macrotest::expand(pattern.to_str().expect("mirror path is UTF-8"));
    }));

    // New fixtures get their snapshot written into the mirror; copy it back so
    // `cargo test` keeps macrotest's refresh workflow.
    for entry in fs::read_dir(&mirror_dir).expect("read macrotest mirror") {
        let snapshot = entry.expect("read mirror entry").path();
        let Some(file_name) = snapshot.file_name().and_then(|name| name.to_str()) else {
            continue;
        };
        let Some(case) = file_name.strip_suffix(".expanded.rs") else {
            continue;
        };
        let tracked = source_dir.join(format!("{case}.expanded.rs"));
        if !tracked.exists() {
            fs::copy(&snapshot, &tracked).expect("copy refreshed snapshot back");
        }
    }

    if let Err(payload) = expansion {
        panic::resume_unwind(payload);
    }
}

fn link_or_copy(source: &Path, target: &Path) {
    fs::hard_link(source, target)
        .or_else(|_| fs::copy(source, target).map(|_| ()))
        .expect("link fixture into the macrotest mirror");
}

#[test]
pub fn expand_struct() {
    expand_cases("struct");
}

#[test]
fn expand_enum() {
    expand_cases("enum");
}

#[test]
fn expand_block() {
    expand_cases("block");
}

#[test]
fn parse_error() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/cases/parse_error/*.rs");
}
