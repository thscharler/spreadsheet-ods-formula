use crate::*;
#[allow(unused_imports)]
use crate::fin::*;

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

///  Calculates the amortization value for the French accounting system using linear depreciation (l'amortissement linéaire comptable) .
#[inline]
pub fn amorlinc_<A: Number, B: DateTimeParam, C: DateTimeParam, D: Number, E: Number, F: Number>(cost: A, purchase_date: B, first_period_end_date: C, salvage: D, period: E, rate: F, b: YearFracMethod) -> FnNumber7<A, B, C, D, E, F, YearFracMethod> {
    FnNumber7("AMORLINC", cost, purchase_date, first_period_end_date, salvage, period, rate, b)
}

///  Calculates the amortization value for the French accounting system using linear depreciation (l'amortissement linéaire comptable) .
#[inline]
pub fn amorlinc<A: Number, B: DateTimeParam, C: DateTimeParam, D: Number, E: Number, F: Number>(cost: A, purchase_date: B, first_period_end_date: C, salvage: D, period: E, rate: F) -> FnNumber6<A, B, C, D, E, F> {
    FnNumber6("AMORLINC", cost, purchase_date, first_period_end_date, salvage, period, rate)
}

///  Calculates the number of days between the beginning of the coupon period that contains the settlement date and the settlement date.
#[inline]
pub fn coupdaybs_<A: DateTimeParam, B: DateTimeParam, C: Number>(settlement: A, maturity: B, frequency: C, b: YearFracMethod) -> FnNumber4<A, B, C, YearFracMethod> {
    FnNumber4("COUPDAYBS", settlement, maturity, frequency, b)
}

///  Calculates the number of days between the beginning of the coupon period that contains the settlement date and the settlement date.
#[inline]
pub fn coupdaybs<A: DateTimeParam, B: DateTimeParam, C: Number>(settlement: A, maturity: B, frequency: C) -> FnNumber3<A, B, C> {
    FnNumber3("COUPDAYBS", settlement, maturity, frequency)
}

///  Calculates the number of days in a coupon period that contains the settlement date
#[inline]
pub fn coupdays_<A: DateTimeParam, B: DateTimeParam, C: Number>(settlement: A, maturity: B, frequency: C, b: YearFracMethod) -> FnNumber4<A, B, C, YearFracMethod> {
    FnNumber4("COUPDAYS", settlement, maturity, frequency, b)
}

///  Calculates the number of days in a coupon period that contains the settlement date
#[inline]
pub fn coupdays<A: DateTimeParam, B: DateTimeParam, C: Number>(settlement: A, maturity: B, frequency: C) -> FnNumber3<A, B, C> {
    FnNumber3("COUPDAYS", settlement, maturity, frequency)
}

///  Calculates the number of days between a settlement date and the next coupon date.
#[inline]
pub fn coupdaysnc_<A: DateTimeParam, B: DateTimeParam, C: Number>(settlement: A, maturity: B, frequency: C, b: YearFracMethod) -> FnNumber4<A, B, C, YearFracMethod> {
    FnNumber4("COUPDAYSNC", settlement, maturity, frequency, b)
}

///  Calculates the number of days between a settlement date and the next coupon date.
#[inline]
pub fn coupdaysnc<A: DateTimeParam, B: DateTimeParam, C: Number>(settlement: A, maturity: B, frequency: C) -> FnNumber3<A, B, C> {
    FnNumber3("COUPDAYSNC", settlement, maturity, frequency)
}

///  Calculates the next coupon date following a settlement.
#[inline]
pub fn coupncd_<A: DateTimeParam, B: DateTimeParam, C: Number>(settlement: A, maturity: B, frequency: C, b: YearFracMethod) -> FnNumber4<A, B, C, YearFracMethod> {
    FnNumber4("COUPNCD", settlement, maturity, frequency, b)
}

///  Calculates the next coupon date following a settlement.
#[inline]
pub fn coupncd<A: DateTimeParam, B: DateTimeParam, C: Number>(settlement: A, maturity: B, frequency: C) -> FnNumber3<A, B, C> {
    FnNumber3("COUPNCD", settlement, maturity, frequency)
}
