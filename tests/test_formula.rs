use spreadsheet_ods::{CellRange, CellRef};

use ods_formula3::date::{Days360Method, WeekdayMethod};
use ods_formula3::of;
use ods_formula3::prelude::*;
use ods_formula3::{
    cell, formula, p, range, CriterionCmp, FArray, FCriterion, FMatrix, FnAny0, ValNumber,
};

use crate::result_test::{test_ok, ReportValues};

mod result_test;

const Q: ReportValues = ReportValues;

fn eq(v: &String, test: &str) -> bool {
    v.as_str() == test
}

#[test]
fn test_base() {
    test_ok(formula(1)).test(eq, "of=1").q(Q);
    test_ok(formula(1)).test(eq, "of=1").q(Q);
    test_ok(formula(1.1)).test(eq, "of=1.1").q(Q);
    test_ok(formula(true)).test(eq, "of=TRUE()").q(Q);
    test_ok(formula(false)).test(eq, "of=FALSE()").q(Q);
    test_ok(formula("asdf")).test(eq, "of=\"asdf\"").q(Q);
    test_ok(formula("jklö".to_string()))
        .test(eq, "of=\"jklö\"")
        .q(Q);
    test_ok(formula(CellRef::local(5, 6)))
        .test(eq, "of=[.G6]")
        .q(Q);
    test_ok(formula(CellRange::local(4, 5, 8, 9)))
        .test(eq, "of=[.F5:.J9]")
        .q(Q);

    test_ok(formula(p(1.1))).test(eq, "of=(1.1)").q(Q);

    // can't be instantiated directly, which is ok.
    // test_ok(formula(FAny("1234".into()))).test(eq, "of=1").q(Q);
    // test_ok(formula(FNumber("1234".into()))).test(eq, "of=1").q(Q);
    // test_ok(formula(FText("1234".into()))).test(eq, "of=1").q(Q);
    // test_ok(formula(FLogical("1".into()))).test(eq, "of=1").q(Q);
    // test_ok(formula(FMatrix("1".into()))).test(eq, "of=1").q(Q);
    // test_ok(formula(FReference("1".into()))).test(eq, "of=1").q(Q);
    test_ok(formula(FCriterion::new(CriterionCmp::Gt, 1)))
        .test(eq, "of=\">\"&1")
        .q(Q);
    test_ok(formula((CriterionCmp::Eq, 5)))
        .test(eq, "of=\"=\"&5")
        .q(Q);
    test_ok(formula(None::<FnAny0>)).test(eq, "of=").q(Q);

    test_ok(formula(FMatrix([[1, 2, 3], [4, 5, 6]])))
        .test(eq, "of={1;2;3|4;5;6}")
        .q(Q);
    test_ok(formula(())).test(eq, "of=").q(Q);
    // test_ok(formula((1, 2, 3, 4)))
    //     .test(eq, "of=1;2;3;4")
    //     .q(Q);
}

#[test]
pub fn test_op() {
    test_ok(formula(ValNumber(1) + 5)).test(eq, "of=1+5").q(Q);
    test_ok(formula(ValNumber(1) - 5)).test(eq, "of=1-5").q(Q);
    test_ok(formula(ValNumber(1) * 5)).test(eq, "of=1*5").q(Q);
    test_ok(formula(ValNumber(1) / 5)).test(eq, "of=1/5").q(Q);
    test_ok(formula(ValNumber(1) ^ 5)).test(eq, "of=1^5").q(Q);
    test_ok(formula(-1)).test(eq, "of=-1").q(Q);

    test_ok(formula(true.and(false)))
        .test(eq, "of=AND(TRUE();FALSE())")
        .q(Q);
    test_ok(formula(true.or(false)))
        .test(eq, "of=OR(TRUE();FALSE())")
        .q(Q);
    test_ok(formula(true.xor(false)))
        .test(eq, "of=XOR(TRUE();FALSE())")
        .q(Q);

    test_ok(formula(of::eq(5, "A"))).test(eq, "of=5=\"A\"").q(Q);
    test_ok(formula(of::ne(5, "A")))
        .test(eq, "of=5<>\"A\"")
        .q(Q);
    test_ok(formula(of::lt(5, "A"))).test(eq, "of=5<\"A\"").q(Q);
    test_ok(formula(of::le(5, "A")))
        .test(eq, "of=5<=\"A\"")
        .q(Q);
    test_ok(formula(of::gt(5, "A"))).test(eq, "of=5>\"A\"").q(Q);
    test_ok(formula(of::ge(5, "A")))
        .test(eq, "of=5>=\"A\"")
        .q(Q);

    test_ok(formula(5.percent())).test(eq, "of=5%").q(Q);

    test_ok(formula("asdf".concat("jklö")))
        .test(eq, "of=\"asdf\"&\"jklö\"")
        .q(Q);
    test_ok(formula("asdf".concat(CellRef::local(5, 5))))
        .test(eq, "of=\"asdf\"&[.F6]")
        .q(Q);

    test_ok(formula(CellRef::local(5, 5).refcat(CellRef::local(6, 6))))
        .test(eq, "of=[.F6]~[.G7]")
        .q(Q);
    test_ok(formula(
        CellRef::local(6, 6).intersect(CellRef::local(7, 7)),
    ))
    .test(eq, "of=[.G7]![.H8]")
    .q(Q);
}

#[test]
fn test_bitop() {
    test_ok(formula(of::bitand(ValNumber(1) + CellRef::local(5, 5), 5)))
        .test(eq, "of=BITAND(1+[.F6];5)")
        .q(Q);
}

#[test]
fn test_imaginary() {
    test_ok(formula(of::imsec(1))).test(eq, "of=IMSEC(1)").q(Q);
    test_ok(formula(of::imsum((5, 7, 9, 99))))
        .test(eq, "of=IMSUM(5;7;9;99)")
        .q(Q);
}

#[test]
fn test_database() {
    test_ok(formula(of::daverage(
        CellRange::local(5, 5, 20, 9),
        "wango",
        CellRange::local(22, 5, 22, 9),
    )))
    .test(eq, "of=DAVERAGE([.F6:.J21];\"wango\";[.F23:.J23])")
    .q(Q);
}

#[test]
fn test_date() {
    test_ok(formula(of::date(1, 1, 1)))
        .test(eq, "of=DATE(1;1;1)")
        .q(Q);
    test_ok(formula(of::datevalue(cell!(6, 7)))).q(Q);
    test_ok(formula(of::day(cell!("FUGU" => 9,9)))).q(Q);
    test_ok(formula(of::days(range!(8,8; + 4, 4), range!(9,9;+4,4)))).q(Q);
    test_ok(formula(of::days360(
        range!("MANGO"=> 9,9,12,12),
        range!("YARO"=> 8,8,12,12),
        Days360Method::Europe,
    )))
    .q(Q);
    test_ok(formula(of::edate(range!("DARO"=> 8,8; +5,5), 7))).q(Q);
    test_ok(formula(of::eomonth(cell!("LOGA"=>4,4), 5))).q(Q);
    test_ok(formula(of::hour(5))).q(Q);
    test_ok(formula(of::isoweeknum(99))).q(Q);
    test_ok(formula(of::minute(cell!(9, 9)))).q(Q);
    test_ok(formula(of::month(cell!(0, 0)))).q(Q);
    test_ok(formula(of::networkdays(
        cell!(5, 5),
        cell!(9, 9),
        Some(FArray([9, 9, 9])),
        Some(FArray([0, 0, 0, 0, 0, 1, 0])),
    )))
    .q(Q);
    test_ok(formula(of::now())).q(Q);
    test_ok(formula(of::second(cell!(5, 5)))).q(Q);
    test_ok(formula(of::time(5, 5, 5))).q(Q);
    test_ok(formula(of::timevalue(cell!(9, 9)))).q(Q);
    test_ok(formula(of::today())).q(Q);
    test_ok(formula(of::weekday(cell!(5, 5), WeekdayMethod::Monday0))).q(Q);
}

#[test]
fn test_compose() {
    test_ok(formula(of::date(1, of::month(of::today()), 2200))).q(Q);
}

#[test]
fn test_lookup() {
    test_ok(formula(of::getpivotdata_fields(
        "bonk",
        range!(1, 1, 10, 10),
        [("wang", "rang")],
    )))
    .q(Q);
    test_ok(formula(of::lookup(
        range!(1, 2, 3, 4),
        FArray([9, 9, 9, 9]),
        Some(FArray([4, 4, 4, 4])),
    )))
    .q(Q);
    test_ok(formula(of::lookup(
        range!(1, 2, 3, 4),
        FArray([9, 9, 9, 9]),
        None::<FArray<(), 0>>,
    )))
    .q(Q);
}

#[test]
fn test_math() {
    test_ok(formula(of::sumproduct((
        FMatrix([[1, 2, 3], [4, 5, 6]]),
        range!(5, 5, 9, 9),
        FMatrix([[9, 9, 9], [10, 10, 10]]),
    ))))
    .q(Q);

    test_ok(formula(of::sumif(
        range!(0, 0, 99, 99),
        (CriterionCmp::Eq, 1001),
        Some(range!(101, 0, 199, 99)),
    )))
    .q(Q);

    test_ok(formula(of::sumifs(
        range!(0, 0, 99, 99),
        [(range!(101, 0, 199, 99), (CriterionCmp::Eq, 1001))],
    )))
    .q(Q);
}
