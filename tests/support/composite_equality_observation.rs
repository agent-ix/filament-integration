// SPDX-License-Identifier: AGPL-3.0-or-later
//! Native caller of the generated composite equality oracles. Inputs come from
//! the original source constructors through the public checked-call interface.

mod family_scalar_limits {
    include!("family_scalar_limits.rs");
}
mod composite_equality_source {
    include!("composite_equality_source.rs");
}
mod oracle_observation;

use qsl_replay::{ApplicationEnvironment, LibraryName};
use quire_contract_model::{
    CheckedPackageEvidence, CheckedPackageReadLimits, CheckedPackageV2, CheckedPackageV2ReadResult,
};
use quire_eval::FamilyOutcome;
use quire_exact::{Meter, Outcome, Value};
use quire_semantic_value::object_closure::ObjectClosure;

fn lift(outcome: Outcome<bool>) -> Outcome<Value> {
    match outcome {
        Outcome::Completed(v) => Outcome::Completed(Value::Boolean(v)),
        Outcome::Undefined(v) => Outcome::Undefined(v),
        Outcome::Refused(v) => Outcome::Refused(v),
        Outcome::Incomplete(v) => Outcome::Incomplete(v),
    }
}

#[test]
fn observe() {
    let source = composite_equality_source::source();
    let (acquired, package) = composite_equality_source::acquire(source.as_bytes());
    let handle = acquired.handle();
    let owner = LibraryName::new(package.name().to_owned()).unwrap();
    let construct = |name: &str| -> Value {
        let declarations = package
            .graph()
            .nodes
            .iter()
            .filter(|node| {
                node.declaration
                    .as_ref()
                    .is_some_and(|d| d.qualified_name.iter().map(AsRef::as_ref).eq([name]))
            })
            .collect::<Vec<_>>();
        let [declaration] = declarations.as_slice() else {
            panic!("one original constructor declaration {name}");
        };
        let key = acquired
            .context()
            .wire_graph()
            .key(&declaration.node_id)
            .expect("actual source constructor key");
        let selected = handle
            .lookup_callable(&owner, &key)
            .expect("original constructor selection");
        let environment = ApplicationEnvironment::new(ObjectClosure::default());
        let mut registry = handle.registry();
        let mut meter = Meter::new(family_scalar_limits::limits(1024));
        let evaluation = registry
            .begin_call(&selected, Vec::new(), &environment, &mut meter)
            .expect("source constructor admission")
            .run()
            .expect("source constructor evaluation");
        let FamilyOutcome::Evaluated(Outcome::Completed(value)) = evaluation.outcome else {
            panic!("source constructor completes: {evaluation:?}");
        };
        value
    };
    let context = Some(acquired.context());
    __CASES__
}
