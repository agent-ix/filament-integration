---
id: TC-099
title: "Execute generated oracle artifacts against FSL value authority"
type: TC
relationships:
  - target: ix://agent-ix/filament-integration/IT-019
    type: references
---
# TC-099: Execute generated oracle artifacts against FSL value authority

## Description

Run IT-019's real FSL-source to emitted V2 to strict IR admission to CG-generated
native artifact path for scalar, composite and function families, including the
authoritative TC-187 observations. Compare those executions with FSL evaluation
of the same admitted claims rather than substituting direct runtime operations.

## Test Procedure

Execute IT-019-SC-01 through SC-06. Obtain the published TC-187 cases by reference
through FSpec's public vector API and dispatch every case explicitly. Keep
admission-only observations separate. Compile the actual CG artifacts, invoke
their selected generated functions, and compare complete typed outcomes,
values, defined accounting, limit denials and selection provenance against the
real FSL authority path. Exercise the specified owner/occurrence mismatch and
sibling controls. Compile the outer test before running generated child Cargo
through its own serialized lane; do not invoke Kani or recursively lock Cargo.

## Expected Results

Every runnable observation and required family agrees on both sides. Unknown
IDs, unexecuted runnable cases, copied fixtures, direct RT substitutions or
collapsed dispositions fail. A measured upstream seam remains Planned with its
exact existing owner ticket and receives no agreement credit; IR-842 remains
open until the complete assigned composition is backed.
