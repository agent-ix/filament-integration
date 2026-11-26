// SPDX-License-Identifier: AGPL-3.0-or-later
//! Original composite-equality source and public acquisition shared by host and native driver.

use super::{
    CheckedPackageEvidence, CheckedPackageReadLimits, CheckedPackageV2, CheckedPackageV2ReadResult,
};
use qsl_replay::{
    acquire_checked_closure, CheckedClosureCompilation, CompileUnit, DependencyInput, ReplayLimits,
    SourceIdentity, StageLimits,
};

/// One authored composite family: parameter type, three constructor bodies and the
/// independently authored `=` truth of the pairs (A,A), (A,B) and (A,C).
pub struct Case {
    pub name: &'static str,
    pub declaration: &'static str,
    pub parameter: &'static str,
    pub constructors: [&'static str; 3],
    pub truths: [bool; 3],
}

/// Selected composite families, in source order.
pub const CASES: [Case; 9] = [
    Case {
        name: "Mixed",
        declaration: "tuple Mixed(Int[0,9], Boolean);\n",
        parameter: "Mixed",
        constructors: ["Mixed(3,true)", "Mixed(3,false)", "Mixed(4,true)"],
        truths: [true, false, false],
    },
    Case {
        name: "Point",
        declaration: "record Point { x: Int[0,9]; y: Int[0,9]; }\n",
        parameter: "Point",
        constructors: [
            "Point { x: 1, y: 2 }",
            "Point { x: 1, y: 3 }",
            "Point { x: 2, y: 2 }",
        ],
        truths: [true, false, false],
    },
    Case {
        name: "Wrapper",
        declaration: "tuple Pair(Boolean, Boolean);\nrecord Wrapper { pair: Pair; n: Int[0,9]; }\n",
        parameter: "Wrapper",
        constructors: [
            "Wrapper { pair: Pair(true,false), n: 1 }",
            "Wrapper { pair: Pair(true,true), n: 1 }",
            "Wrapper { pair: Pair(true,false), n: 2 }",
        ],
        truths: [true, false, false],
    },
    Case {
        name: "Node",
        declaration: "record Node { next: Option<Node>; n: Int[0,9]; }\n",
        parameter: "Node",
        constructors: [
            "Node { next: absent, n: 1 }",
            "Node { next: absent, n: 2 }",
            "Node { next: absent, n: 1 }",
        ],
        truths: [true, false, true],
    },
    Case {
        name: "Choice",
        declaration: "union Choice { First, Second }\n",
        parameter: "Choice",
        constructors: ["Choice::First", "Choice::Second", "Choice::First"],
        truths: [true, false, true],
    },
    Case {
        name: "Shape",
        declaration: "union Shape { Dot(Integer), Box(Integer) }\n",
        parameter: "Shape",
        constructors: ["Shape::Dot(1)", "Shape::Dot(2)", "Shape::Box(1)"],
        truths: [true, false, false],
    },
    Case {
        name: "Seq",
        declaration: "",
        parameter: "Sequence<Int[0,9]>[0,3]",
        constructors: ["sequence[1,2]", "sequence[2,1]", "sequence[1,2,3]"],
        truths: [true, false, false],
    },
    Case {
        name: "Bag",
        declaration: "",
        parameter: "Set<Int[0,9]>[0,3]",
        constructors: ["set[1,2]", "set[1,3]", "set[2,1]"],
        truths: [true, false, true],
    },
    Case {
        name: "Opt",
        declaration: "",
        parameter: "Option<Int[0,9]>",
        constructors: ["absent", "absent", "absent"],
        truths: [true, true, true],
    },
];

/// Original source: declarations, `=` and `!=` functions and closed constructors.
pub fn source() -> String {
    let mut text = String::from(
        "language \"ix:native\" edition \"1-draft\";\nprofile v = \"quire.value.complete/v1\";\n",
    );
    for case in &CASES {
        if !text.contains(case.declaration) {
            text.push_str(case.declaration);
        }
    }
    for case in &CASES {
        for (prefix, operator) in [("eq", "="), ("ne", "!=")] {
            text.push_str(&format!(
                "function {prefix}{name} using v(left: {parameter}, right: {parameter}): Boolean pure {{ left {operator} right }}\n",
                name = case.name,
                parameter = case.parameter,
            ));
        }
        for (index, constructor) in case.constructors.iter().enumerate() {
            text.push_str(&format!(
                "function mk{name}{letter} using v(): {parameter} pure {{ {constructor} }}\n",
                name = case.name,
                letter = ["A", "B", "C"][index],
                parameter = case.parameter,
            ));
        }
    }
    text
}

/// Independently check, emit and strictly admit the provided original source.
pub fn acquire(bytes: &[u8]) -> (CheckedClosureCompilation, CheckedPackageV2) {
    let acquisition = acquire_checked_closure(
        CompileUnit {
            source: SourceIdentity::new("agent-ix", "composite-equality"),
            package_name: "composite-equality".to_owned(),
            path: "composite-equality.native",
            bytes,
        },
        [],
        &DependencyInput::default(),
        StageLimits::default(),
        ReplayLimits::default(),
    )
    .expect("original composite source checks and emits");
    let mut evidence = CheckedPackageEvidence::new();
    evidence.support_feature("quire.value.complete/v1");
    let CheckedPackageV2ReadResult::Admitted(package) = CheckedPackageV2::read(
        acquisition.package().bytes(),
        CheckedPackageReadLimits::bounded(),
        &evidence,
    ) else {
        panic!("exact composite source emission admits");
    };
    (acquisition, *package)
}
