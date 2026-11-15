// SPDX-License-Identifier: AGPL-3.0-or-later
//! Live FSL checked-package producer and IR reader for IT-016 tests.

use std::{fs, path::Path};

use quire_contract_ir::{
    read_checked_package, CheckedPackageDispatchResult, CheckedPackageEvidence,
    CheckedPackageReadLimits, CheckedPackageV2,
};
use serde_json::Value;

use crate::support::{cargo_command::command as cargo_child_command, fsl_root};

/// A FSL-owned model document selected by the producer and supplied as the
/// same bytes to the IR reader. The document stays in FSL's checkout.
pub struct ModelInput<'a> {
    pub identity: &'a str,
    pub path: &'a Path,
}

/// Inputs to one live FSL source → checked-package V2 → IR readback.
pub struct CheckedProducerRequest<'a> {
    pub source_path: &'a str,
    pub source: &'a str,
    pub package_name: &'a str,
    pub models: &'a [ModelInput<'a>],
    pub execution_points: Option<&'a Path>,
}

/// Run the lock-selected producer and read exactly its emitted bytes. A
/// missing executable, refused source or refused reader fails the test.
pub fn emit_and_read_checked_v2(request: CheckedProducerRequest<'_>) -> CheckedPackageV2 {
    let directory = tempfile::tempdir().expect("test source directory");
    fs::write(directory.path().join(request.source_path), request.source)
        .expect("write authored FSL source");
    let mut command = cargo_child_command();
    command
        .current_dir(directory.path())
        .env(
            "CARGO_TARGET_DIR",
            Path::new(env!("CARGO_MANIFEST_DIR")).join("target"),
        )
        .args(["run", "--locked", "--quiet", "--manifest-path"])
        .arg(fsl_root().join("Cargo.toml"))
        .args(["--example", "emit_checked_v2", "--"]);
    for model in request.models {
        command.arg("--model").arg(model.path);
    }
    if let Some(points) = request.execution_points {
        command.arg("--execution-points").arg(points);
    }
    let output = command
        .args(["--package-name", request.package_name, request.source_path])
        .output()
        .expect("lock-selected FSL emitter starts");
    assert!(
        output.status.success(),
        "FSL emitter refused: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let wire: Value = serde_json::from_slice(&output.stdout).expect("FSL emits JSON");
    let mut evidence = CheckedPackageEvidence::new();
    for feature in wire["lock"]["required_features"]
        .as_array()
        .expect("producer lock feature list")
    {
        evidence.support_feature(feature.as_str().expect("feature name"));
    }
    for model in request.models {
        let bytes = fs::read(model.path).expect("FSL-owned model document is present");
        evidence
            .insert_domain_package_document(model.identity, bytes.as_slice())
            .expect("FSL model document is valid reader evidence");
    }
    let package = match read_checked_package(
        &output.stdout,
        CheckedPackageReadLimits::bounded(),
        &evidence,
    ) {
        CheckedPackageDispatchResult::AdmittedV2(package) => *package,
        other => panic!("IR refused FSL's checked package: {other:?}"),
    };
    eprintln!(
        "producer=cargo run --locked --example emit_checked_v2; application={}; \
         source-revision=Cargo.lock; outcome=AdmittedV2({})",
        env!("CARGO_PKG_VERSION"),
        package.name()
    );
    package
}
