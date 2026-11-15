---
id: IT-014
title: "Compose one shared type dependency cohort"
type: IT
---
# IT-014: Compose one shared type dependency cohort

## Objective

Check that the locked integration graph resolves each shared type package once,
uses one revision of each multi-crate source repository, and excludes the
retired canonical JSON encoder.

## Target Integration

The composed Cargo lock for canonical encoding, FSL, semantic value, IR,
codegen, runtime, verification contracts, filament core data and quire-rs.

## Preconditions

Cargo metadata reads the integration manifest with `--locked`.

## Inputs

The repository's committed `Cargo.toml` and `Cargo.lock`.

The named shared type packages are `filament-semantic-value`,
`filament-contract-model`, `filament-contract-ir`, `filament-contract-codegen`,
`filament-contract-runtime`, `filament-spec-language`, `fsl-foundation`,
`fsl-replay` and `filament-verification-contracts`. The first-party Git repository
for each is the same as its package name, except `filament-contract-model` and
`filament-contract-ir` both come from `filament-contract-ir`, and `fsl-foundation`
and `fsl-replay` both come from `filament-spec-language`.
The canonical source contributes `quire-canonical` and
`quire-canonical-derive`. The normal graph selects `quire-rs`; its optional
extraction and parser packages need not be present.

## Test Procedure

1. Resolve the locked dependency graph with Cargo metadata.
   IT-014-SC-01: every named shared type package has exactly one package
   identity and a locked Git source; the canonical, FSL, IR and three filament
   core data crates and the selected `quire-rs` package come from their expected
   repositories, with one revision across every selected package from each
   repository; `serde_json_canonicalizer` is absent.

## Expected Results

Cargo resolves one coherent dependency cohort without the retired encoder.
