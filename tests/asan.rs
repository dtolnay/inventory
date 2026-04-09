use std::env;
use std::ffi::OsString;
use std::path::Path;
use std::process::Command;

#[rustversion::attr(not(nightly), ignore = "requires nightly")]
#[cfg_attr(miri, ignore = "incompatible with miri")]
#[cfg_attr(
    not(any(
        all(target_os = "macos",  any(target_arch = "x86_64", target_arch = "aarch64")),
        all(target_os = "linux", any(target_arch = "x86_64", target_arch = "aarch64")),
        all(target_os = "freebsd", target_arch = "x86_64"),
        all(target_os = "fuchsia", any(target_arch = "x86_64", target_arch = "aarch64")),
    )),
    ignore = "Architecture does not support ASAN"
)]
#[test]
fn asan_build() {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let fixture_dir = manifest_dir.join("tests/fixtures/asan");
    let target_dir = manifest_dir.join("target/tests/asan");

    let mut rustflags = OsString::from("-Zsanitizer=address -Cforce-frame-pointers=yes --cfg=asan");
    if let Some(existing) = env::var_os("RUSTFLAGS") {
        rustflags.push(" ");
        rustflags.push(existing);
    }

    let build_target = detect_target_triple();

    let cargo = env::var_os("CARGO").unwrap_or_else(|| OsString::from("cargo"));
    let output = Command::new(cargo)
        .arg("build")
        .arg("-Zbuild-std")
        .arg("--target")
        .arg(build_target)
        .current_dir(&fixture_dir)
        .env("CARGO_TARGET_DIR", &target_dir)
        .env("RUSTFLAGS", rustflags)
        .output()
        .expect("failed to run cargo build for ASan reproducer");

    if !output.status.success() {
        let stdout = String::from_utf8_lossy(&output.stdout);
        let stderr = String::from_utf8_lossy(&output.stderr);
        panic!(
            "ASan reproducer build failed\nstatus: {}\nstdout:\n{}\nstderr:\n{}",
            output.status, stdout, stderr,
        );
    }
}

fn detect_target_triple() -> String {
    // Without build scripts we can't easily detect the target triple.
    // Since the documentation says that ASAN is only available on a few targets,
    // we do this ugly "detection" below, which avoids using a build script.
    // https://doc.rust-lang.org/beta/unstable-book/compiler-flags/sanitizer.html#addresssanitizer
    // aarch64-apple-darwin
    // aarch64-unknown-fuchsia
    // aarch64-unknown-linux-gnu
    // x86_64-apple-darwin
    // x86_64-unknown-fuchsia
    // x86_64-unknown-freebsd
    // x86_64-unknown-linux-gnu


    if cfg!(target_os = "macos") {
        if cfg!(target_arch = "aarch64") {
            "aarch64-apple-darwin"
        } else if cfg!(target_arch = "x86_64") {
            "x86_64-apple-darwin"
        } else {
            panic!("unsupported macos architecture");
        }
    } else if cfg!(target_os = "linux") {
        if cfg!(target_arch = "x86_64") && cfg!(target_env = "gnu") {
            "x86_64-unknown-linux-gnu"
        } else if cfg!(target_arch = "aarch64") && cfg!(target_env = "gnu") {
            "aarch64-unknown-linux-gnu"
        } else {
            panic!("unsupported linux architecture");
        }
    } else if cfg!(target_os = "freebsd") && cfg!(target_arch = "x86_64") {
        "x86_64-unknown-freebsd"
    } else if cfg!(target_os = "fuchsia") && cfg!(target_arch = "aarch64") {
        "aarch64-unknown-fuchsia"
    } else {
        panic!("unsupported target");
    }.to_string()
}