---
id: IT-013
title: "Compose bound FSL exports with V2 codegen"
type: IT
relationships:
  - target: ix://agent-ix/filament-spec-language/FR-307
    type: references
  - target: ix://agent-ix/filament-driver/FR-010
    type: verifies
  - target: ix://agent-ix/filament-contract-codegen/FR-039
    type: verifies
  - target: ix://agent-ix/filament-contract-codegen/FR-017
    type: verifies
---
# IT-013: Compose bound FSL exports with V2 codegen

## Objective

Check that the driver preserves the original request indexes when a FSL
binding export refuses one item, while the remaining items reach real codegen.
Record the separate V2 scalar outcomes from FSL-emitted checked-package bytes.

## Target Integration

FSL parse, check and emit → contract IR admission → driver export partition →
codegen package negotiation, routing and generation. The scalar controls use
FSL-emitted V2 bytes → contract IR admission → codegen derivation.

## Preconditions

The original partition fixture injects the binding-export result because its
FSL-compiled unit has no abstraction relation. The additional natural-path
fixture observes the production `QslCg` decision and checks that export is
skipped. The additional model fixture forces the driver's relation seam and
delegates to FSL's real `export_bindings`: FSL returns an `Unbound` refusal
for a model-field expression and `Ok` for its scalar siblings. FSL's compiler
does not emit an authored abstraction relation for this fixture, so the forced
relation test verifies real export and downstream composition at the seam,
not production relation authoring.

## Inputs

Three checked integer items A, B and C with export results `Ok`, `Err`, `Ok`
at driver indexes 0, 1 and 2, plus a FSL-emitted V2 package with integer
addition and Boolean negation operations.

## Test Procedure

1. Drive A, B and C with the injected export result through real codegen.
   IT-013-SC-01: codegen receives two bound envelope items and full positions
   at indexes 0, 1 and 2; both settlement layers and generated items name 0
   and 2, while the driver retains B's exact typed refusal and occurrence at 1.
2. Derive and generate a scalar claim for the admitted integer addition node.
   IT-013-SC-02: the claim is IR-confirmed and generated with a nonempty
   source map and checked bounds.
3. Derive a scalar claim for the admitted Boolean negation node.
   IT-013-SC-03: codegen generates the IR-confirmed Boolean negation claim
   with a nonempty source map. This control makes no Boolean proof claim.
4. Drive an unbound FSL-compiled integer addition through the production
   `QslCg` path with a Kani generation context.
   IT-013-SC-04: codegen returns one supported Kani record with a retained
   scalar harness, and FSL's checked-package call of `step(4)` completes with
   the integer 5.
5. Run the retained harness and oracle package under installed Kani, then
   invoke its staged native scalar operation with operand 4.
   IT-013-SC-05: the proof verdict is `Verified`, and the native integer
   result equals the checked FSL `step(4)` result of 5.
6. Replace exactly the generated harness's integer `Add` operation with
   `Subtract`, keeping the checked FSL source and generated identity.
   IT-013-SC-06: real Kani returns `Falsified` with a counterexample on the
   generated soundness assertion, rather than a compilation refusal.
7. Drive three scalar requirement items through the natural `QslCg` relation
   decision while observing export calls.
   IT-013-SC-07: a package without an abstraction relation skips
   `export_bindings`; every item reaches codegen and no item is refused.
8. Drive a checked package containing two scalar items and a model-field
   expression through the relation seam. Delegate the export to FSL's public
   `export_bindings` function and then to the production driver and codegen.
   IT-013-SC-08: FSL itself returns the model item's typed `Unbound` refusal;
   the driver retains that exact refusal and original index, withholds the
   refused item from codegen and generates both `Ok` siblings.

## Expected Results

An export-refused item never enters the codegen envelope or generated item
list, and the next bound item's driver index remains 2. The V2 controls
generate distinct integer and Boolean scalar claims. The routed integer scalar
proves and produces the same concrete value as FSL native evaluation, while
an arithmetic mutation falsifies that proof.

The former IT-010-SC-02 `integer-ir/v1` command-export path is retired by
design. SC-07 and SC-08 restore the V2 `export_bindings` → driver → CG
composition check. A source-authored abstraction relation remains untested;
SC-08 forces the seam because this checked fixture emits no relation.
