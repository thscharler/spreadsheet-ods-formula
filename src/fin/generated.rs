#[allow(unused_imports)]
use super::*;
use crate::*;

///  Calculates the accrued interest for securities with periodic interest payments
#[inline]
pub fn accrint<A: DateTimeParam, B: DateTimeParam, C: DateTimeParam, D: Number>(
    issue: A,
    first: B,
    settlement: C,
    coupon: D,
    frequency: Frequency,
    b: Option<YearFracMethod>,
    calc_method: Option<bool>,
) -> FnNumber7<A, B, C, D, Frequency, Option<YearFracMethod>, Option<bool>> {
    FnNumber7(
        "ACCRINT",
        issue,
        first,
        settlement,
        coupon,
        frequency,
        b,
        calc_method,
    )
}

///  Calculates the accrued interest for securities that pay at maturity
#[inline]
pub fn accrintm<A: DateTimeParam, B: DateTimeParam, C: Number, D: Number>(
    issue: A,
    settlement: B,
    coupon: C,
    par: D,
    b: YearFracMethod,
) -> FnNumber5<A, B, C, D, YearFracMethod> {
    FnNumber5("ACCRINTM", issue, settlement, coupon, par, b)
}
