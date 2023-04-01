use crate::result_test::{test_ok, ReportValues};
use ods_formula3 as of;
use ods_formula3::prelude::*;
use ods_formula3::{add, CriterionCmp, FCriterion, FnAny0, ValNumber};
use spreadsheet_ods::{CellRange, CellRef};
use std::arch::asm;

mod result_test;

const Q: ReportValues = ReportValues;

fn eq(v: &String, test: &str) -> bool {
    v.as_str() == test
}

#[test]
fn test_base() {
    test_ok(of::formula(1)).test(eq, "of=1").q(Q);
    test_ok(of::formula(1)).test(eq, "of=1").q(Q);
    test_ok(of::formula(1.1)).test(eq, "of=1.1").q(Q);
    test_ok(of::formula(true)).test(eq, "of=TRUE()").q(Q);
    test_ok(of::formula(false)).test(eq, "of=FALSE()").q(Q);
    test_ok(of::formula("asdf")).test(eq, "of=\"asdf\"").q(Q);
    test_ok(of::formula("jklö".to_string()))
        .test(eq, "of=\"jklö\"")
        .q(Q);
    test_ok(of::formula(CellRef::local(5, 6)))
        .test(eq, "of=[.G6]")
        .q(Q);
    test_ok(of::formula(CellRange::local(4, 5, 8, 9)))
        .test(eq, "of=[.F5:.J9]")
        .q(Q);

    test_ok(of::formula(p(1.1))).test(eq, "of=(1.1)").q(Q);

    // can't be instantiated directly, which is ok.
    // test_ok(of::formula(FAny("1234".into()))).test(eq, "of=1").q(Q);
    // test_ok(of::formula(FNumber("1234".into()))).test(eq, "of=1").q(Q);
    // test_ok(of::formula(FText("1234".into()))).test(eq, "of=1").q(Q);
    // test_ok(of::formula(FLogical("1".into()))).test(eq, "of=1").q(Q);
    // test_ok(of::formula(FMatrix("1".into()))).test(eq, "of=1").q(Q);
    // test_ok(of::formula(FReference("1".into()))).test(eq, "of=1").q(Q);
    test_ok(of::formula(FCriterion::new(CriterionCmp::Gt, 1)))
        .test(eq, "of=\">\"&1")
        .q(Q);
    test_ok(of::formula((CriterionCmp::Eq, 5)))
        .test(eq, "of=\"=\"&5")
        .q(Q);
    test_ok(of::formula(None::<FnAny0>)).test(eq, "of=").q(Q);

    test_ok(of::formula([[1, 2, 3], [4, 5, 6]]))
        .test(eq, "of={1;2;3|4;5;6}")
        .q(Q);
    test_ok(of::formula(())).test(eq, "of=").q(Q);
    // test_ok(of::formula((1, 2, 3, 4)))
    //     .test(eq, "of=1;2;3;4")
    //     .q(Q);
}

#[test]
pub fn test_op() {
    test_ok(of::formula(ValNumber(1) + 5))
        .test(eq, "of=1+5")
        .q(Q);
    test_ok(of::formula(ValNumber(1) - 5))
        .test(eq, "of=1-5")
        .q(Q);
    test_ok(of::formula(ValNumber(1) * 5))
        .test(eq, "of=1*5")
        .q(Q);
    test_ok(of::formula(ValNumber(1) / 5))
        .test(eq, "of=1/5")
        .q(Q);
    test_ok(of::formula(ValNumber(1) ^ 5))
        .test(eq, "of=1^5")
        .q(Q);
    test_ok(of::formula(-1)).test(eq, "of=-1").q(Q);

    test_ok(of::formula(true.and(false)))
        .test(eq, "of=AND(TRUE();FALSE())")
        .q(Q);
    test_ok(of::formula(true.or(false)))
        .test(eq, "of=OR(TRUE();FALSE())")
        .q(Q);
    test_ok(of::formula(true.xor(false)))
        .test(eq, "of=XOR(TRUE();FALSE())")
        .q(Q);

    test_ok(of::formula(of::eq(5, "A")))
        .test(eq, "of=5=\"A\"")
        .q(Q);
    test_ok(of::formula(of::ne(5, "A")))
        .test(eq, "of=5<>\"A\"")
        .q(Q);
    test_ok(of::formula(of::lt(5, "A")))
        .test(eq, "of=5<\"A\"")
        .q(Q);
    test_ok(of::formula(of::le(5, "A")))
        .test(eq, "of=5<=\"A\"")
        .q(Q);
    test_ok(of::formula(of::gt(5, "A")))
        .test(eq, "of=5>\"A\"")
        .q(Q);
    test_ok(of::formula(of::ge(5, "A")))
        .test(eq, "of=5>=\"A\"")
        .q(Q);

    test_ok(of::formula(5.percent())).test(eq, "of=5%").q(Q);

    test_ok(of::formula("asdf".concat("jklö")))
        .test(eq, "of=\"asdf\"&\"jklö\"")
        .q(Q);
    test_ok(of::formula("asdf".concat(CellRef::local(5, 5))))
        .test(eq, "of=\"asdf\"&[.F6]")
        .q(Q);

    test_ok(of::formula(
        CellRef::local(5, 5).refcat(CellRef::local(6, 6)),
    ))
    .test(eq, "of=[.F6]~[.G7]")
    .q(Q);
    test_ok(of::formula(
        CellRef::local(6, 6).intersect(CellRef::local(7, 7)),
    ))
    .test(eq, "of=[.G7]![.H8]")
    .q(Q);
}

#[test]
fn test_imaginary() {
    test_ok(of::formula(of::imsec(1)))
        .test(eq, "of=IMSEC(1)")
        .q(Q);
    test_ok(of::formula(of::imsum((5, 7, 9, 99))))
        .test(eq, "of=IMSUM(5;7;9;99)")
        .q(Q);
}

#[test]
fn test_database() {
    test_ok(of::formula(of::daverage(
        CellRange::local(5, 5, 20, 9),
        "wango",
        CellRange::local(22, 5, 22, 9),
    )))
    .test(eq, "of=DAVERAGE([.F6:.J21];\"wango\";[.F23:.J23])")
    .q(Q);
}
