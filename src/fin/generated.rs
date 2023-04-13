use crate::*;
#[allow(unused_imports)]
use super::*;

///  Calculates the accrued interest for securities with periodic interest payments
#[inline]
pub fn accrint__<A: DateTimeParam, B: DateTimeParam, C: DateTimeParam, D: Number>(issue: A, first: B, settlement: C, coupon: D, frequency: Frequency, b: YearFracMethod, calc_method: bool) -> FnNumber7<A, B, C, D, Frequency, YearFracMethod, bool> {
    FnNumber7("ACCRINT", issue, first, settlement, coupon, frequency, b, calc_method)
}

///  Calculates the accrued interest for securities with periodic interest payments
#[inline]
pub fn accrint_<A: DateTimeParam, B: DateTimeParam, C: DateTimeParam, D: Number>(issue: A, first: B, settlement: C, coupon: D, frequency: Frequency, b: YearFracMethod) -> FnNumber6<A, B, C, D, Frequency, YearFracMethod> {
    FnNumber6("ACCRINT", issue, first, settlement, coupon, frequency, b)
}

///  Calculates the accrued interest for securities with periodic interest payments
#[inline]
pub fn accrint<A: DateTimeParam, B: DateTimeParam, C: DateTimeParam, D: Number>(issue: A, first: B, settlement: C, coupon: D, frequency: Frequency) -> FnNumber5<A, B, C, D, Frequency> {
    FnNumber5("ACCRINT", issue, first, settlement, coupon, frequency)
}

///  Calculates the accrued interest for securities that pay at maturity
#[inline]
pub fn accrintm_<A: DateTimeParam, B: DateTimeParam, C: Number, D: Number>(issue: A, settlement: B, coupon: C, par: D, b: YearFracMethod) -> FnNumber5<A, B, C, D, YearFracMethod> {
    FnNumber5("ACCRINTM", issue, settlement, coupon, par, b)
}

///  Calculates the accrued interest for securities that pay at maturity
#[inline]
pub fn accrintm<A: DateTimeParam, B: DateTimeParam, C: Number, D: Number>(issue: A, settlement: B, coupon: C, par: D) -> FnNumber4<A, B, C, D> {
    FnNumber4("ACCRINTM", issue, settlement, coupon, par)
}
