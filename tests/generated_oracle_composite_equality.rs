// SPDX-License-Identifier: AGPL-3.0-or-later
//! FR-018 composite equality operation cells: generated oracle against independent QSL.

#[path = "support/family_checked_source.rs"]
mod family_checked_source;
#[path = "support/family_function_selection.rs"]
mod family_function_selection;
#[path = "support/family_scalar_limits.rs"]
mod family_scalar_limits;
#[path = "support/generated_oracle.rs"]
mod generated_oracle;
#[path = "support/oracle_observation.rs"]
mod oracle_observation;

#[path = "support/composite_equality_source.rs"]
mod composite_equality_source;

use family_function_selection::body;
use ix_trace_rs::trace;
use quire_contract_codegen::{
    generate_composite_equality_oracles, routed_contract_lowering_profile, ClaimDisposition,
    CompositeEqualityItem, CompositeEqualityRefusal, ContractDriverResultSelection,
    ContractRequestPosition, EqualityOperandDescriptor, EqualityOperatorKind,
};
use quire_contract_ir::{
    CheckedNodeId, CheckedPackageEvidence, CheckedPackageReadLimits, CheckedPackageV2,
    CheckedPackageV2ReadResult, CheckedTerm, ContractDependencies, ContractPackageReadResult,
    ContractPackageWireV1,
};
use quire_eval::FamilyOutcome;
use quire_exact::{ChargePoint, LimitKind, Meter, Outcome, Value};

use composite_equality_source::CASES;

const WORKS: [u64; 5] = [1024, 0, 1, 2, 3];

fn operand(package: &CheckedPackageV2, application: &CheckedTerm, index: usize) -> EqualityOperandDescriptor {
    let CheckedTerm::Application { arguments, .. } = application else {
        panic!("original equality application");
    };
    let CheckedTerm::Reference { target } = &arguments[index] else {
        panic!("original equality operand is a parameter reference");
    };
    let selected = package
        .graph()
        .nodes
        .iter()
        .filter(|node| &node.node_id == target)
        .collect::<Vec<_>>();
    let [node] = selected.as_slice() else {
        panic!("one original parameter node");
    };
    EqualityOperandDescriptor::typed(node.semantic_type.clone())
}

#[trace("TC-114", "IT-019-SC-05")]
#[test]
fn composite_equality_matches_independent_qsl_per_charge() {
    let source = composite_equality_source::source();
    let (acquisition, package) = composite_equality_source::acquire(source.as_bytes());
    let mut nodes = Vec::new();
    let mut items = Vec::new();
    for case in &CASES {
        for (prefix, operator) in [
            ("eq", EqualityOperatorKind::Equal),
            ("ne", EqualityOperatorKind::NotEqual),
        ] {
            let node_id = body(&package, &format!("{prefix}{}", case.name));
            let term = package.term(&node_id).expect("original equality body");
            items.push(CompositeEqualityItem {
                node_id: node_id.clone(),
                operator,
                left: operand(&package, term, 0),
                right: operand(&package, term, 1),
            });
            nodes.push(node_id);
        }
    }
    let lowered = package.lower(&nodes, &routed_contract_lowering_profile());
    let contract = match ContractPackageWireV1::read_with_source(
        lowered.package.canonical_bytes().expect("original lowered contract"),
        CheckedPackageReadLimits::bounded(),
        &ContractDependencies::new(),
        &package,
    )
    .expect("the source authenticates its contract")
    {
        ContractPackageReadResult::Admitted(contract) => contract,
        other => panic!("the actual lowered contract admits: {other:?}"),
    };
    let positions = nodes
        .iter()
        .cloned()
        .map(|node_id| ContractRequestPosition::DriverItem {
            node_id,
            result_selection: ContractDriverResultSelection::NotApplicable,
        })
        .collect::<Vec<_>>();
    let indices = (0..nodes.len()).collect::<Vec<_>>();
    let output = generate_composite_equality_oracles(
        &contract,
        Some(&package),
        Some(acquisition.context()),
        &positions,
        &indices,
        &items,
        Default::default(),
    )
    .expect("composite equality generation");
    let claim_for = |output: &quire_contract_codegen::CompositeEqualityOracles, index: usize| {
        let claims = output
            .claim_map
            .items
            .iter()
            .filter(|claim| claim.node_id == nodes[index])
            .cloned()
            .collect::<Vec<_>>();
        let [claim] = claims.as_slice() else {
            panic!("one original equality claim at {index}");
        };
        claim.clone()
    };
    // Typed generation dispositions the owner contract defines today.
    for (case_index, case) in CASES.iter().enumerate() {
        for op in 0..2 {
            let claim = claim_for(&output, case_index * 2 + op);
            match case.name {
                "Shape" => assert!(
                    matches!(
                        &claim.result,
                        ClaimDisposition::Refused {
                            refusal: CompositeEqualityRefusal::CompositeContextMismatch { .. }
                        }
                    ),
                    "payload union: {:?}",
                    claim.result
                ),
                "Bag" => assert!(
                    matches!(
                        &claim.result,
                        ClaimDisposition::Refused {
                            refusal: CompositeEqualityRefusal::Unsupported { .. }
                        }
                    ),
                    "bounded set: {:?}",
                    claim.result
                ),
                other => assert!(
                    matches!(&claim.result, ClaimDisposition::Generated(_)),
                    "{other} must generate: {:?}",
                    claim.result
                ),
            }
        }
    }
    let authority = checked(&source);
    let evaluate = |name: &str, arguments: Vec<Value>, work: u64| {
        let function = authority
            .graph()
            .function_by_identity(
                authority
                    .graph()
                    .function_identity(name)
                    .expect("original checked function identity"),
            )
            .expect("original checked body");
        let mut meter = Meter::new(family_scalar_limits::limits(work));
        meter.enable_charge_capture(64, 256);
        let evaluation = qsl_eval::value::evaluate_closed_with_frame(
            &authority,
            function.body,
            function.slots,
            arguments,
            function.formed_units,
            &mut meter,
        )
        .expect("independent public checked-body evaluation");
        assert!(evaluation.losses.is_empty());
        let FamilyOutcome::Evaluated(outcome) = evaluation.outcome else {
            panic!("composite equality value outcome");
        };
        (outcome, meter)
    };
    let mut expected = Vec::new();
    let mut driver = include_str!("support/composite_equality_observation.rs").to_owned();
    let mut calls = String::new();
    for (case_index, case) in CASES.iter().enumerate() {
        if matches!(case.name, "Shape" | "Bag") {
            continue;
        }
        let values = ["A", "B", "C"].map(|letter| {
            let (outcome, _) = evaluate(&format!("mk{}{letter}", case.name), Vec::new(), 1024);
            let Outcome::Completed(value) = outcome else {
                panic!("original constructor completes: {outcome:?}");
            };
            value
        });
        expected.push(format!(
            "input {}: {:?} {:?} {:?}",
            case.name, values[0], values[1], values[2]
        ));
        calls.push_str(&format!(
            "{{ let a=construct(\"mk{n}A\"); let b=construct(\"mk{n}B\"); let c=construct(\"mk{n}C\");\n println!(\"IT019_OBSERVATION input {n}: {{:?}} {{:?}} {{:?}}\",a,b,c);\n",
            n = case.name
        ));
        for (op, prefix) in ["eq", "ne"].into_iter().enumerate() {
            let claim = claim_for(&output, case_index * 2 + op);
            let ClaimDisposition::Generated(generated) = &claim.result else {
                unreachable!("asserted generated above");
            };
            for (sample, right) in [0_usize, 1, 2].into_iter().enumerate() {
                let truth = case.truths[sample] ^ (op == 1);
                for work in WORKS {
                    let (outcome, mut meter) = evaluate(
                        &format!("{prefix}{}", case.name),
                        vec![values[0].clone(), values[right].clone()],
                        work,
                    );
                    if work == 1024 {
                        assert!(
                            matches!(&outcome, Outcome::Completed(Value::Boolean(actual)) if *actual == truth),
                            "{prefix}{} sample {sample}: independent authored truth {truth}: {outcome:?}",
                            case.name
                        );
                    } else if work == 0 {
                        assert!(
                            matches!(&outcome, Outcome::Incomplete(stop) if stop.limit_kind == LimitKind::WorkUnits && stop.limit == 0 && stop.consumed == 0 && stop.charge_point == ChargePoint::EqualityPlanForm),
                            "{prefix}{} work 0 stop: {outcome:?}",
                            case.name
                        );
                    } else {
                        assert!(
                            matches!(&outcome, Outcome::Incomplete(stop) if stop.limit_kind == LimitKind::WorkUnits && stop.limit == work),
                            "{prefix}{} work {work}: {outcome:?}",
                            case.name
                        );
                    }
                    expected.push(oracle_observation::captured_snapshot(&outcome, &mut meter));
                    calls.push_str(&format!(
                        "{{ let prepared=quire_composite_equality_oracles::{env}(context).expect(\"prepared environment\"); let mut meter=Meter::new(family_scalar_limits::limits({work})); meter.enable_charge_capture(64,256); let actual=lift(quire_composite_equality_oracles::{oracle}(&prepared,&{left},&{right},&mut meter)); println!(\"IT019_OBSERVATION {{}}\",oracle_observation::captured_snapshot(&actual,&mut meter)); }}\n",
                        env = generated.environment_symbol,
                        oracle = generated.oracle_symbol,
                        left = "a",
                        right = ["a", "b", "c"][right],
                    ));
                }
            }
        }
        calls.push_str("}\n");
    }
    driver = driver.replace("__CASES__", &calls);
    driver = driver.replace(
        "include!(\"family_scalar_limits.rs\");",
        include_str!("support/family_scalar_limits.rs"),
    );
    driver = driver.replace(
        "include!(\"composite_equality_source.rs\");",
        include_str!("support/composite_equality_source.rs"),
    );
    assert_eq!(
        generated_oracle::execute(&output.artifacts, &driver),
        expected
    );
}

fn checked(source: &str) -> qsl_package::CheckedPackage {
    let parsed = qsl_cst::parse(
        qsl_foundation::SourceIdentity::new("agent-ix", "composite-equality"),
        "composite-equality.native",
        source.as_bytes(),
        qsl_cst::Limits::default(),
    )
    .expect("original composite source parses");
    assert!(parsed.is_admissible(), "{:?}", parsed.diagnostics());
    let unit = qsl_forms::build_unit(&parsed).expect("original composite forms");
    qsl_package::CheckedPackage::link(
        qsl_semantics::check::PackageDeclarations::assemble(
            parsed.source().reference().clone(),
            "composite-equality".to_owned(),
            unit,
            qsl_semantics::model::intake::SelectedModels::default(),
            Vec::new(),
        )
        .expect("original composite assembly")
        .check(quire_semantic_value::checking::CheckingLimits::default())
        .expect("original composite checks"),
    )
}
