use std::{env, fs};

use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct Manifest {
    profile: Option<ProfileTable>,
}

#[derive(Deserialize, Debug)]
struct ProfileTable {
    dev: Option<Profile>,
    release: Option<Profile>,
}

#[derive(Deserialize, Debug)]
struct Profile {
    #[serde(rename = "codegen-units")]
    codegen_units: Option<u8>,
}

fn main() {
    let vendor = env::var("CARGO_CFG_TARGET_VENDOR");

    if vendor.as_deref().eq(&Ok("apple")) {
        let manifest_path = match std::env::var("CARGO_MANIFEST_DIR") {
            Ok(manifest) => format!("{}/Cargo.toml", manifest),
            Err(_) => "./Cargo.toml".to_string(),
        };

        let manifest = fs::read_to_string(manifest_path.as_str())
            .expect(format!("unable to load manifest at `{}`", manifest_path).as_str());

        let manifest: Manifest = toml::from_str(manifest.as_str())
            .expect(format!("failed to parse manifest at `{}`", manifest_path).as_str());

        if manifest
            .profile
            .as_ref()
            .and_then(|profile| profile.dev.as_ref())
            .and_then(|profile| profile.codegen_units)
            .ne(&Some(1))
        {
            panic_with_warning("dev");
        }

        if manifest
            .profile
            .as_ref()
            .and_then(|profile| profile.release.as_ref())
            .and_then(|profile| profile.codegen_units)
            .ne(&Some(1))
        {
            panic_with_warning("release");
        }
    }
}

fn panic_with_warning(profile: &str) {
    panic!(
        "codegen-units must be set to 1 to compile inventory for Apple targets.

Please update Cargo.toml:
```
[profile.{}]
codegen-units = 1
```

See: https://doc.rust-lang.org/cargo/reference/profiles.html#codegen-units
Issue: https://github.com/dtolnay/inventory/issues/52",
        profile
    );
}
