// SPDX-License-Identifier: MIT OR Apache-2.0

#![deny(clippy::all)]
#![deny(clippy::pedantic)]

use std::path::Path;
use std::process::Command;

const TARGETS: &str = env!("CARGO_MANIFEST_DIR");

fn validate_target(target_path: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let path = target_path.to_str().unwrap();
    println!("Validating {path}");

    let output = Command::new("rustc")
        .args(&["-Z", "unstable-options", "--target", path, "--version"])
        .output()?;

    return if output.status.success() {
        Ok(())
    } else {
        Err(String::from_utf8_lossy(&output.stderr).into())
    };
}

#[test]
fn validate_targets() -> Result<(), Box<dyn std::error::Error>> {
    let target_dir = Path::new(TARGETS);
    for entry in std::fs::read_dir(target_dir)? {
        let entry = entry?;
        let file_name = entry.file_name();
        let path = target_dir.join(file_name);

        if path.is_file() && path.extension().and_then(|s| s.to_str()) == Some("json") {
            validate_target(&path)?;
        }
    }

    Ok(())
}
