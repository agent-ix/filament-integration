---
id: TC-097
title: "Compare FSpec scalar cases on FSL and the runtime"
type: TC
relationships:
  - target: ix://agent-ix/filament-integration/IT-017
    type: references
  - target: ix://agent-ix/filament-contract-runtime/FR-006
    type: verifies
  - target: ix://agent-ix/filament-contract-runtime/FR-007
    type: verifies
---

# TC-097: Compare FSpec scalar cases on FSL and the runtime

## Description

Run IT-017 against FSpec's public scalar-vector row API for its integer
division, decimal, IEEE, text and enum, and quantity and unit cases. Source
admission remains distinct from evaluated agreement.

## Test Procedure

Execute IT-017-SC-01 through IT-017-SC-05. Dispatch each lock-selected FSpec
row ID to executable cases or an explicit PENDING case with an open upstream
owner ticket and the exact missing path. For every runnable evaluation case,
run FSL and RT under matched inputs and limits and check the specified result,
charges, counters and denied-charge behavior. For every runnable admission
case, check the FSL admission decision without calling a runtime scalar
operation. Run the generated sweeps required by RT TC-018 through TC-022.

## Expected Results

Every runnable FSpec evaluation case agrees on both sides; every runnable
admission case retains its own decision. Missing FSpec API support, an unrun
runnable operation or required sweep, a ticketless PENDING case, or a mismatch
fails closure. Ticket-backed PENDING cases are reported as open work and do
not count as agreement for their affected RT procedure.
