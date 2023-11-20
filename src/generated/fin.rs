use crate::*;
#[allow(unused_imports)]
use crate::fin::*;

#[inline]
pub fn accrint<A: DateTime, B: DateTime, C: DateTime, D: Number, E: Number>(issue: A, first: B, settlement: C, coupon: D, par: E, frequency: Frequency) -> FnNumber6<A, B, C, D, E, Frequency> {
    FnNumber6("ACCRINT", issue, first, settlement, coupon, par, frequency)
}

#[inline]
pub fn accrint_<A: DateTime, B: DateTime, C: DateTime, D: Number, E: Number>(issue: A, first: B, settlement: C, coupon: D, par: E, frequency: Frequency, b: YearFracMethod) -> FnNumber7<A, B, C, D, E, Frequency, YearFracMethod> {
    FnNumber7("ACCRINT", issue, first, settlement, coupon, par, frequency, b)
}

#[inline]
pub fn accrint__<A: DateTime, B: DateTime, C: DateTime, D: Number, E: Number, F: Logical>(issue: A, first: B, settlement: C, coupon: D, par: E, frequency: Frequency, b: YearFracMethod, calc_method: F) -> FnNumber8<A, B, C, D, E, Frequency, YearFracMethod, F> {
    FnNumber8("ACCRINT", issue, first, settlement, coupon, par, frequency, b, calc_method)
}

#[inline]
pub fn accrintm<A: DateTime, B: DateTime, C: Number, D: Number>(issue: A, settlement: B, coupon: C, par: D) -> FnNumber4<A, B, C, D> {
    FnNumber4("ACCRINTM", issue, settlement, coupon, par)
}

#[inline]
pub fn accrintm_<A: DateTime, B: DateTime, C: Number, D: Number>(issue: A, settlement: B, coupon: C, par: D, b: YearFracMethod) -> FnNumber5<A, B, C, D, YearFracMethod> {
    FnNumber5("ACCRINTM", issue, settlement, coupon, par, b)
}

#[inline]
pub fn amorlinc<A: Number, B: DateTime, C: DateTime, D: Number, E: Number, F: Number>(cost: A, purchase_date: B, first_period_end_date: C, salvage: D, period: E, rate: F) -> FnNumber6<A, B, C, D, E, F> {
    FnNumber6("AMORLINC", cost, purchase_date, first_period_end_date, salvage, period, rate)
}

#[inline]
pub fn amorlinc_<A: Number, B: DateTime, C: DateTime, D: Number, E: Number, F: Number>(cost: A, purchase_date: B, first_period_end_date: C, salvage: D, period: E, rate: F, b: YearFracMethod) -> FnNumber7<A, B, C, D, E, F, YearFracMethod> {
    FnNumber7("AMORLINC", cost, purchase_date, first_period_end_date, salvage, period, rate, b)
}

#[inline]
pub fn coupdaybs<A: DateTime, B: DateTime>(settlement: A, maturity: B, frequency: Frequency) -> FnNumber3<A, B, Frequency> {
    FnNumber3("COUPDAYBS", settlement, maturity, frequency)
}

#[inline]
pub fn coupdaybs_<A: DateTime, B: DateTime>(settlement: A, maturity: B, frequency: Frequency, b: YearFracMethod) -> FnNumber4<A, B, Frequency, YearFracMethod> {
    FnNumber4("COUPDAYBS", settlement, maturity, frequency, b)
}

#[inline]
pub fn coupdays<A: DateTime, B: DateTime>(settlement: A, maturity: B, frequency: Frequency) -> FnNumber3<A, B, Frequency> {
    FnNumber3("COUPDAYS", settlement, maturity, frequency)
}

#[inline]
pub fn coupdays_<A: DateTime, B: DateTime>(settlement: A, maturity: B, frequency: Frequency, b: YearFracMethod) -> FnNumber4<A, B, Frequency, YearFracMethod> {
    FnNumber4("COUPDAYS", settlement, maturity, frequency, b)
}

#[inline]
pub fn coupdaync<A: DateTime, B: DateTime>(settlement: A, maturity: B, frequency: Frequency) -> FnNumber3<A, B, Frequency> {
    FnNumber3("COUPDAYNC", settlement, maturity, frequency)
}

#[inline]
pub fn coupdaync_<A: DateTime, B: DateTime>(settlement: A, maturity: B, frequency: Frequency, b: YearFracMethod) -> FnNumber4<A, B, Frequency, YearFracMethod> {
    FnNumber4("COUPDAYNC", settlement, maturity, frequency, b)
}

#[inline]
pub fn coupncd<A: DateTime, B: DateTime>(settlement: A, maturity: B, frequency: Frequency) -> FnNumber3<A, B, Frequency> {
    FnNumber3("COUPNCD", settlement, maturity, frequency)
}

#[inline]
pub fn coupncd_<A: DateTime, B: DateTime>(settlement: A, maturity: B, frequency: Frequency, b: YearFracMethod) -> FnNumber4<A, B, Frequency, YearFracMethod> {
    FnNumber4("COUPNCD", settlement, maturity, frequency, b)
}

#[inline]
pub fn coupnum<A: DateTime, B: DateTime>(settlement: A, maturity: B, frequency: Frequency) -> FnNumber3<A, B, Frequency> {
    FnNumber3("COUPNUM", settlement, maturity, frequency)
}

#[inline]
pub fn coupnum_<A: DateTime, B: DateTime>(settlement: A, maturity: B, frequency: Frequency, b: YearFracMethod) -> FnNumber4<A, B, Frequency, YearFracMethod> {
    FnNumber4("COUPNUM", settlement, maturity, frequency, b)
}

#[inline]
pub fn couppcd<A: DateTime, B: DateTime>(settlement: A, maturity: B, frequency: Frequency) -> FnNumber3<A, B, Frequency> {
    FnNumber3("COUPPCD", settlement, maturity, frequency)
}

#[inline]
pub fn couppcd_<A: DateTime, B: DateTime>(settlement: A, maturity: B, frequency: Frequency, b: YearFracMethod) -> FnNumber4<A, B, Frequency, YearFracMethod> {
    FnNumber4("COUPPCD", settlement, maturity, frequency, b)
}

#[inline]
pub fn cumipmt<A: Number, B: Number, C: Number, D: Number, E: Number>(rate: A, periods: B, value: C, start: D, end: E, type_: MaturityDate) -> FnNumber6<A, B, C, D, E, MaturityDate> {
    FnNumber6("CUMIPMT", rate, periods, value, start, end, type_)
}

#[inline]
pub fn cumprinc<A: Number, B: Number, C: Number, D: Number, E: Number>(rate: A, periods: B, value: C, start: D, end: E, type_: MaturityDate) -> FnNumber6<A, B, C, D, E, MaturityDate> {
    FnNumber6("CUMPRINC", rate, periods, value, start, end, type_)
}

#[inline]
pub fn db<A: Number, B: Number, C: Number, D: Number>(cost: A, salvage: B, life_time: C, period: D) -> FnNumber4<A, B, C, D> {
    FnNumber4("DB", cost, salvage, life_time, period)
}

#[inline]
pub fn db_<A: Number, B: Number, C: Number, D: Number, E: Number>(cost: A, salvage: B, life_time: C, period: D, month: E) -> FnNumber5<A, B, C, D, E> {
    FnNumber5("DB", cost, salvage, life_time, period, month)
}

#[inline]
pub fn ddb<A: Number, B: Number, C: Number, D: Number>(cost: A, salvage: B, life_time: C, period: D) -> FnNumber4<A, B, C, D> {
    FnNumber4("DDB", cost, salvage, life_time, period)
}

#[inline]
pub fn ddb_<A: Number, B: Number, C: Number, D: Number, E: Number>(cost: A, salvage: B, life_time: C, period: D, declination_factor: E) -> FnNumber5<A, B, C, D, E> {
    FnNumber5("DDB", cost, salvage, life_time, period, declination_factor)
}

#[inline]
pub fn disc<A: DateTime, B: DateTime, C: Number, D: Number>(settlement: A, maturity: B, price: C, redemption: D) -> FnNumber4<A, B, C, D> {
    FnNumber4("DISC", settlement, maturity, price, redemption)
}

#[inline]
pub fn disc_<A: DateTime, B: DateTime, C: Number, D: Number>(settlement: A, maturity: B, price: C, redemption: D, b: YearFracMethod) -> FnNumber5<A, B, C, D, YearFracMethod> {
    FnNumber5("DISC", settlement, maturity, price, redemption, b)
}

#[inline]
pub fn dollarde<A: Number, B: Number>(fractional: A, denominator: B) -> FnNumber2<A, B> {
    FnNumber2("DOLLARDE", fractional, denominator)
}

#[inline]
pub fn dollarfr<A: Number, B: Number>(decimal: A, denominator: B) -> FnNumber2<A, B> {
    FnNumber2("DOLLARFR", decimal, denominator)
}

#[inline]
pub fn duration<A: DateTime, B: DateTime, C: Number, D: Number>(settlement: A, maturity: B, coupon: C, yield_: D, frequency: Frequency) -> FnNumber5<A, B, C, D, Frequency> {
    FnNumber5("DURATION", settlement, maturity, coupon, yield_, frequency)
}

#[inline]
pub fn duration_<A: DateTime, B: DateTime, C: Number, D: Number>(settlement: A, maturity: B, coupon: C, yield_: D, frequency: Frequency, b: YearFracMethod) -> FnNumber6<A, B, C, D, Frequency, YearFracMethod> {
    FnNumber6("DURATION", settlement, maturity, coupon, yield_, frequency, b)
}

#[inline]
pub fn effect<A: Number, B: Number>(rate: A, payments: B) -> FnNumber2<A, B> {
    FnNumber2("EFFECT", rate, payments)
}

#[inline]
pub fn fv<A: Number, B: Number, C: Number>(rate: A, nper: B, payment: C) -> FnNumber3<A, B, C> {
    FnNumber3("FV", rate, nper, payment)
}

#[inline]
pub fn fv_<A: Number, B: Number, C: Number, D: Number>(rate: A, nper: B, payment: C, pv: D) -> FnNumber4<A, B, C, D> {
    FnNumber4("FV", rate, nper, payment, pv)
}

#[inline]
pub fn fv__<A: Number, B: Number, C: Number, D: Number, E: Number>(rate: A, nper: B, payment: C, pv: D, pay_type: E) -> FnNumber5<A, B, C, D, E> {
    FnNumber5("FV", rate, nper, payment, pv, pay_type)
}

#[inline]
pub fn fvschedule<A: Number, B: Sequence>(principal: A, schedule: B) -> FnNumber2<A, B> {
    FnNumber2("FVSCHEDULE", principal, schedule)
}

#[inline]
pub fn intrate<A: DateTime, B: DateTime, C: Number, D: Number>(settlement: A, maturity: B, investment: C, redemption: D) -> FnNumber4<A, B, C, D> {
    FnNumber4("INTRATE", settlement, maturity, investment, redemption)
}

#[inline]
pub fn intrate_<A: DateTime, B: DateTime, C: Number, D: Number>(settlement: A, maturity: B, investment: C, redemption: D, basis: YearFracMethod) -> FnNumber5<A, B, C, D, YearFracMethod> {
    FnNumber5("INTRATE", settlement, maturity, investment, redemption, basis)
}

#[inline]
pub fn ipmt<A: Number, B: Number, C: Number, D: Number>(rate: A, period: B, nper: C, p_v: D) -> FnNumber4<A, B, C, D> {
    FnNumber4("IPMT", rate, period, nper, p_v)
}

#[inline]
pub fn ipmt_<A: Number, B: Number, C: Number, D: Number, E: Number>(rate: A, period: B, nper: C, p_v: D, f_v: E) -> FnNumber5<A, B, C, D, E> {
    FnNumber5("IPMT", rate, period, nper, p_v, f_v)
}

#[inline]
pub fn ipmt__<A: Number, B: Number, C: Number, D: Number, E: Number, F: Number>(rate: A, period: B, nper: C, p_v: D, f_v: E, type_: F) -> FnNumber6<A, B, C, D, E, F> {
    FnNumber6("IPMT", rate, period, nper, p_v, f_v, type_)
}

#[inline]
pub fn irr<A: Sequence>(values: A) -> FnNumber1<A> {
    FnNumber1("IRR", values)
}

#[inline]
pub fn irr_<A: Sequence, B: Number>(values: A, guess: B) -> FnNumber2<A, B> {
    FnNumber2("IRR", values, guess)
}

#[inline]
pub fn ispmt<A: Number, B: Number, C: Number, D: Number>(rate: A, period: B, nper: C, pv: D) -> FnNumber4<A, B, C, D> {
    FnNumber4("ISPMT", rate, period, nper, pv)
}

#[inline]
pub fn mduration<A: DateTime, B: DateTime, C: Number, D: Number>(settlement: A, maturity: B, coupon: C, yield_: D, frequency: Frequency) -> FnNumber5<A, B, C, D, Frequency> {
    FnNumber5("MDURATION", settlement, maturity, coupon, yield_, frequency)
}

#[inline]
pub fn mduration_<A: DateTime, B: DateTime, C: Number, D: Number>(settlement: A, maturity: B, coupon: C, yield_: D, frequency: Frequency, b: YearFracMethod) -> FnNumber6<A, B, C, D, Frequency, YearFracMethod> {
    FnNumber6("MDURATION", settlement, maturity, coupon, yield_, frequency, b)
}

#[inline]
pub fn mirr<A: Array, B: Number, C: Number>(values: A, investment: B, reinvest_rate: C) -> FnNumber3<A, B, C> {
    FnNumber3("MIRR", values, investment, reinvest_rate)
}

#[inline]
pub fn nominal<A: Number, B: Number>(effective_rate: A, compounding_periods: B) -> FnNumber2<A, B> {
    FnNumber2("NOMINAL", effective_rate, compounding_periods)
}

#[inline]
pub fn nper<A: Number, B: Number, C: Number>(rate: A, payment: B, pv: C) -> FnNumber3<A, B, C> {
    FnNumber3("NPER", rate, payment, pv)
}

#[inline]
pub fn nper_<A: Number, B: Number, C: Number, D: Number>(rate: A, payment: B, pv: C, fv: D) -> FnNumber4<A, B, C, D> {
    FnNumber4("NPER", rate, payment, pv, fv)
}

#[inline]
pub fn nper__<A: Number, B: Number, C: Number, D: Number>(rate: A, payment: B, pv: C, fv: D, pay_type: PayType) -> FnNumber5<A, B, C, D, PayType> {
    FnNumber5("NPER", rate, payment, pv, fv, pay_type)
}

#[inline]
pub fn npv<A: Number, B: Sequence>(rate: A, values: B) -> FnNumber2<A, B> {
    FnNumber2("NPV", rate, values)
}

#[inline]
pub fn oddfprice<A: DateTime, B: DateTime, C: DateTime, D: DateTime, E: Number, F: Number, G: Number>(settlement: A, maturity: B, issue: C, first: D, rate: E, yield_: F, redemption: G, frequency: Frequency) -> FnNumber8<A, B, C, D, E, F, G, Frequency> {
    FnNumber8("ODDFPRICE", settlement, maturity, issue, first, rate, yield_, redemption, frequency)
}

#[inline]
pub fn oddfprice_<A: DateTime, B: DateTime, C: DateTime, D: DateTime, E: Number, F: Number, G: Number>(settlement: A, maturity: B, issue: C, first: D, rate: E, yield_: F, redemption: G, frequency: Frequency, b: YearFracMethod) -> FnNumber9<A, B, C, D, E, F, G, Frequency, YearFracMethod> {
    FnNumber9("ODDFPRICE", settlement, maturity, issue, first, rate, yield_, redemption, frequency, b)
}

#[inline]
pub fn oddfyield<A: DateTime, B: DateTime, C: DateTime, D: DateTime, E: Number, F: Number, G: Number>(settlement: A, maturity: B, issue: C, first: D, rate: E, price: F, redemption: G, frequency: Frequency) -> FnNumber8<A, B, C, D, E, F, G, Frequency> {
    FnNumber8("ODDFYIELD", settlement, maturity, issue, first, rate, price, redemption, frequency)
}

#[inline]
pub fn oddfyield_<A: DateTime, B: DateTime, C: DateTime, D: DateTime, E: Number, F: Number, G: Number>(settlement: A, maturity: B, issue: C, first: D, rate: E, price: F, redemption: G, frequency: Frequency, b: YearFracMethod) -> FnNumber9<A, B, C, D, E, F, G, Frequency, YearFracMethod> {
    FnNumber9("ODDFYIELD", settlement, maturity, issue, first, rate, price, redemption, frequency, b)
}

#[inline]
pub fn oddlprice<A: DateTime, B: DateTime, C: DateTime, D: Number, E: Number, F: Number>(settlement: A, maturity: B, last: C, rate: D, annual_yield: E, redemption: F, frequency: Frequency) -> FnNumber7<A, B, C, D, E, F, Frequency> {
    FnNumber7("ODDLPRICE", settlement, maturity, last, rate, annual_yield, redemption, frequency)
}

#[inline]
pub fn oddlprice_<A: DateTime, B: DateTime, C: DateTime, D: Number, E: Number, F: Number>(settlement: A, maturity: B, last: C, rate: D, annual_yield: E, redemption: F, frequency: Frequency, b: YearFracMethod) -> FnNumber8<A, B, C, D, E, F, Frequency, YearFracMethod> {
    FnNumber8("ODDLPRICE", settlement, maturity, last, rate, annual_yield, redemption, frequency, b)
}

#[inline]
pub fn oddlyield<A: DateTime, B: DateTime, C: DateTime, D: Number, E: Number, F: Number>(settlement: A, maturity: B, last: C, rate: D, price: E, redemption: F, frequency: Frequency) -> FnNumber7<A, B, C, D, E, F, Frequency> {
    FnNumber7("ODDLYIELD", settlement, maturity, last, rate, price, redemption, frequency)
}

#[inline]
pub fn oddlyield_<A: DateTime, B: DateTime, C: DateTime, D: Number, E: Number, F: Number>(settlement: A, maturity: B, last: C, rate: D, price: E, redemption: F, frequency: Frequency, b: YearFracMethod) -> FnNumber8<A, B, C, D, E, F, Frequency, YearFracMethod> {
    FnNumber8("ODDLYIELD", settlement, maturity, last, rate, price, redemption, frequency, b)
}

#[inline]
pub fn pduration<A: Number, B: Number, C: Number>(rate: A, current_value: B, specified_value: C) -> FnNumber3<A, B, C> {
    FnNumber3("PDURATION", rate, current_value, specified_value)
}

#[inline]
pub fn pmt<A: Number, B: Number, C: Number>(rate: A, nper: B, pv: C) -> FnNumber3<A, B, C> {
    FnNumber3("PMT", rate, nper, pv)
}

#[inline]
pub fn pmt_<A: Number, B: Number, C: Number, D: Number>(rate: A, nper: B, pv: C, fv: D) -> FnNumber4<A, B, C, D> {
    FnNumber4("PMT", rate, nper, pv, fv)
}

#[inline]
pub fn pmt__<A: Number, B: Number, C: Number, D: Number, E: Number>(rate: A, nper: B, pv: C, fv: D, pay_type: E) -> FnNumber5<A, B, C, D, E> {
    FnNumber5("PMT", rate, nper, pv, fv, pay_type)
}

#[inline]
pub fn ppmt<A: Number, B: Number, C: Number, D: Number>(rate: A, period: B, nper: C, present: D) -> FnNumber4<A, B, C, D> {
    FnNumber4("PPMT", rate, period, nper, present)
}

#[inline]
pub fn ppmt_<A: Number, B: Number, C: Number, D: Number, E: Number>(rate: A, period: B, nper: C, present: D, future: E) -> FnNumber5<A, B, C, D, E> {
    FnNumber5("PPMT", rate, period, nper, present, future)
}

#[inline]
pub fn ppmt__<A: Number, B: Number, C: Number, D: Number, E: Number, F: Number>(rate: A, period: B, nper: C, present: D, future: E, type_: F) -> FnNumber6<A, B, C, D, E, F> {
    FnNumber6("PPMT", rate, period, nper, present, future, type_)
}

#[inline]
pub fn price<A: DateTime, B: DateTime, C: Number, D: Number, E: Number>(settlement: A, maturity: B, rate: C, annual_yield: D, redemption: E, frequency: Frequency) -> FnNumber6<A, B, C, D, E, Frequency> {
    FnNumber6("PRICE", settlement, maturity, rate, annual_yield, redemption, frequency)
}

#[inline]
pub fn price_<A: DateTime, B: DateTime, C: Number, D: Number, E: Number>(settlement: A, maturity: B, rate: C, annual_yield: D, redemption: E, frequency: Frequency, bas: YearFracMethod) -> FnNumber7<A, B, C, D, E, Frequency, YearFracMethod> {
    FnNumber7("PRICE", settlement, maturity, rate, annual_yield, redemption, frequency, bas)
}

#[inline]
pub fn pricedisc<A: DateTime, B: DateTime, C: Number, D: Number>(settlement: A, maturity: B, discount: C, redemption: D) -> FnNumber4<A, B, C, D> {
    FnNumber4("PRICEDISC", settlement, maturity, discount, redemption)
}

#[inline]
pub fn pricedisc_<A: DateTime, B: DateTime, C: Number, D: Number>(settlement: A, maturity: B, discount: C, redemption: D, b: YearFracMethod) -> FnNumber5<A, B, C, D, YearFracMethod> {
    FnNumber5("PRICEDISC", settlement, maturity, discount, redemption, b)
}

#[inline]
pub fn pricemat<A: DateTime, B: DateTime, C: DateTime, D: Number, E: Number>(settlement: A, maturity: B, issue: C, rate: D, annual_yield: E) -> FnNumber5<A, B, C, D, E> {
    FnNumber5("PRICEMAT", settlement, maturity, issue, rate, annual_yield)
}

#[inline]
pub fn pricemat_<A: DateTime, B: DateTime, C: DateTime, D: Number, E: Number>(settlement: A, maturity: B, issue: C, rate: D, annual_yield: E, b: YearFracMethod) -> FnNumber6<A, B, C, D, E, YearFracMethod> {
    FnNumber6("PRICEMAT", settlement, maturity, issue, rate, annual_yield, b)
}

#[inline]
pub fn pv<A: Number, B: Number, C: Number>(rate: A, nper: B, payment: C) -> FnNumber3<A, B, C> {
    FnNumber3("PV", rate, nper, payment)
}

#[inline]
pub fn pv_<A: Number, B: Number, C: Number, D: Number>(rate: A, nper: B, payment: C, fv: D) -> FnNumber4<A, B, C, D> {
    FnNumber4("PV", rate, nper, payment, fv)
}

#[inline]
pub fn pv__<A: Number, B: Number, C: Number, D: Number, E: Number>(rate: A, nper: B, payment: C, fv: D, pay_type: E) -> FnNumber5<A, B, C, D, E> {
    FnNumber5("PV", rate, nper, payment, fv, pay_type)
}

#[inline]
pub fn rate<A: Number, B: Number, C: Number>(nper: A, payment: B, pv: C) -> FnNumber3<A, B, C> {
    FnNumber3("RATE", nper, payment, pv)
}

#[inline]
pub fn rate_<A: Number, B: Number, C: Number, D: Number>(nper: A, payment: B, pv: C, fv: D) -> FnNumber4<A, B, C, D> {
    FnNumber4("RATE", nper, payment, pv, fv)
}

#[inline]
pub fn rate__<A: Number, B: Number, C: Number, D: Number, E: Number>(nper: A, payment: B, pv: C, fv: D, pay_type: E) -> FnNumber5<A, B, C, D, E> {
    FnNumber5("RATE", nper, payment, pv, fv, pay_type)
}

#[inline]
pub fn rate___<A: Number, B: Number, C: Number, D: Number, E: Number, F: Number>(nper: A, payment: B, pv: C, fv: D, pay_type: E, guess: F) -> FnNumber6<A, B, C, D, E, F> {
    FnNumber6("RATE", nper, payment, pv, fv, pay_type, guess)
}

#[inline]
pub fn received<A: DateTime, B: DateTime, C: Number, D: Number>(settlement: A, maturity: B, investment: C, discount: D) -> FnNumber4<A, B, C, D> {
    FnNumber4("RECEIVED", settlement, maturity, investment, discount)
}

#[inline]
pub fn received_<A: DateTime, B: DateTime, C: Number, D: Number>(settlement: A, maturity: B, investment: C, discount: D, b: YearFracMethod) -> FnNumber5<A, B, C, D, YearFracMethod> {
    FnNumber5("RECEIVED", settlement, maturity, investment, discount, b)
}

#[inline]
pub fn rri<A: Number, B: Number, C: Number>(nper: A, pv: B, fv: C) -> FnNumber3<A, B, C> {
    FnNumber3("RRI", nper, pv, fv)
}

#[inline]
pub fn sln<A: Number, B: Number, C: Number>(cost: A, salvage: B, life_time: C) -> FnNumber3<A, B, C> {
    FnNumber3("SLN", cost, salvage, life_time)
}

#[inline]
pub fn syd<A: Number, B: Number, C: Number, D: Number>(cost: A, salvage: B, life_time: C, period: D) -> FnNumber4<A, B, C, D> {
    FnNumber4("SYD", cost, salvage, life_time, period)
}

#[inline]
pub fn tbilleq<A: DateTime, B: DateTime, C: Number>(settlement: A, maturity: B, discount: C) -> FnNumber3<A, B, C> {
    FnNumber3("TBILLEQ", settlement, maturity, discount)
}

#[inline]
pub fn tbillprice<A: DateTime, B: DateTime, C: Number>(settlement: A, maturity: B, discount: C) -> FnNumber3<A, B, C> {
    FnNumber3("TBILLPRICE", settlement, maturity, discount)
}

#[inline]
pub fn tbillyield<A: DateTime, B: DateTime, C: Number>(settlement: A, maturity: B, price: C) -> FnNumber3<A, B, C> {
    FnNumber3("TBILLYIELD", settlement, maturity, price)
}

#[inline]
pub fn vdb<A: Number, B: Number, C: Number, D: Number, E: Number>(cost: A, salvage: B, life_time: C, start_period: D, end_period: E) -> FnNumber5<A, B, C, D, E> {
    FnNumber5("VDB", cost, salvage, life_time, start_period, end_period)
}

#[inline]
pub fn vdb_<A: Number, B: Number, C: Number, D: Number, E: Number, F: Number>(cost: A, salvage: B, life_time: C, start_period: D, end_period: E, depreciation_factor: F) -> FnNumber6<A, B, C, D, E, F> {
    FnNumber6("VDB", cost, salvage, life_time, start_period, end_period, depreciation_factor)
}

#[inline]
pub fn vdb__<A: Number, B: Number, C: Number, D: Number, E: Number, F: Number, G: Logical>(cost: A, salvage: B, life_time: C, start_period: D, end_period: E, depreciation_factor: F, no_switch: G) -> FnNumber7<A, B, C, D, E, F, G> {
    FnNumber7("VDB", cost, salvage, life_time, start_period, end_period, depreciation_factor, no_switch)
}

#[inline]
pub fn xirr<A: Sequence, B: Sequence>(values: A, dates: B) -> FnNumber2<A, B> {
    FnNumber2("XIRR", values, dates)
}

#[inline]
pub fn xirr_<A: Sequence, B: Sequence, C: Number>(values: A, dates: B, guess: C) -> FnNumber3<A, B, C> {
    FnNumber3("XIRR", values, dates, guess)
}

#[inline]
pub fn xnpv<A: Number, B: ReferenceOrArray, C: ReferenceOrArray>(rate: A, values: B, dates: C) -> FnNumber3<A, B, C> {
    FnNumber3("XNPV", rate, values, dates)
}

#[inline]
pub fn yield_<A: DateTime, B: DateTime, C: Number, D: Number, E: Number>(settlement: A, maturity: B, rate: C, price: D, redemption: E, frequency: Frequency) -> FnNumber6<A, B, C, D, E, Frequency> {
    FnNumber6("YIELD", settlement, maturity, rate, price, redemption, frequency)
}

#[inline]
pub fn yield__<A: DateTime, B: DateTime, C: Number, D: Number, E: Number>(settlement: A, maturity: B, rate: C, price: D, redemption: E, frequency: Frequency, b: YearFracMethod) -> FnNumber7<A, B, C, D, E, Frequency, YearFracMethod> {
    FnNumber7("YIELD", settlement, maturity, rate, price, redemption, frequency, b)
}

#[inline]
pub fn yielddisc<A: DateTime, B: DateTime, C: Number, D: Number>(settlement: A, maturity: B, price: C, redemption: D) -> FnNumber4<A, B, C, D> {
    FnNumber4("YIELDDISC", settlement, maturity, price, redemption)
}

#[inline]
pub fn yielddisc_<A: DateTime, B: DateTime, C: Number, D: Number>(settlement: A, maturity: B, price: C, redemption: D, b: YearFracMethod) -> FnNumber5<A, B, C, D, YearFracMethod> {
    FnNumber5("YIELDDISC", settlement, maturity, price, redemption, b)
}

#[inline]
pub fn yieldmat<A: DateTime, B: DateTime, C: DateTime, D: Number, E: Number>(settlement: A, maturity: B, issue: C, rate: D, price: E) -> FnNumber5<A, B, C, D, E> {
    FnNumber5("YIELDMAT", settlement, maturity, issue, rate, price)
}

#[inline]
pub fn yieldmat_<A: DateTime, B: DateTime, C: DateTime, D: Number, E: Number>(settlement: A, maturity: B, issue: C, rate: D, price: E, b: YearFracMethod) -> FnNumber6<A, B, C, D, E, YearFracMethod> {
    FnNumber6("YIELDMAT", settlement, maturity, issue, rate, price, b)
}
