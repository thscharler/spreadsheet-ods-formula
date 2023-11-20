//! 
//! The financial functions are defined for use in financial calculations.
//! 
//! An annuity is a recurring series of payments. A "simple annuity" is one 
//! where equal payments are made at equal intervals, and the compounding of 
//! interest occurs at those same intervals. The time between payments is 
//! called the "payment interval". Where payments are made at the end of the 
//! payment interval, it is called an "ordinary annuity". Where payments are 
//! made at the beginning of the payment interval, it is called an "annuity 
//! due". Periods are numbered starting at 1.
//! 
//! Financial functions defined in this standard use a cash flow sign 
//! convention where outgoing cash flows are negative and incoming cash flows 
//! are positive.

use crate::*;
#[allow(unused_imports)]
use crate::fin::*;

/// Calculates the accrued interest for securities with periodic interest 
/// payments.
///
/// __Syntax__: 
/// ```ods
///     ACCRINT( Issue DateParam; First DateParam; Settlement DateParam; Coupon Number; Par Number; Frequency Integer )
/// ```
///
/// __Constraints__:
/// Issue < First < Settlement ; Coupon > 0; Par > 0
/// 
/// Frequency is one of the following values:
///
/// __Semantics__:
/// Calculates the accrued interest for securities with periodic interest 
/// payments. ACCRINT supports short, standard, and long Coupon periods.
/// 
/// If CalcMethod is TRUE (the default) then ACCRINT returns the sum of the 
/// accrued interest in each coupon period from issue date until settlement 
/// date. If CalcMethod is FALSE then ACCRINT returns the sum of the accrued 
/// interest in each coupon period from first interest date until settlement 
/// date. For each coupon period, the interest is Par * Coupon * 
/// YEARFRAC(start-of-period;end-of-period; B)
/// 
/// •Issue: The security's issue or dated date.
/// 
/// •First: The security's first interest date.
/// 
/// •Settlement: The security's settlement date.
/// 
/// •Coupon: The security's annual coupon rate.
/// 
/// •Par: The security's par value, that is, the principal to be paid at 
/// maturity.
/// 
/// •Frequency: The number of coupon payments per year.
/// 
/// •B: Indicates the day-count convention to use in the calculation. 4.11.7
/// 
/// •CalcMethod: A logical value that specifies how to treat the case where 
/// Settlement > First.
///
/// __See also__: "ACCRINTM", "YEARFRAC", 
#[inline]
pub fn accrint<A: DateTime, B: DateTime, C: DateTime, D: Number, E: Number>(issue: A, first: B, settlement: C, coupon: D, par: E, frequency: Frequency) -> FnNumber6<A, B, C, D, E, Frequency> {
    FnNumber6("ACCRINT", issue, first, settlement, coupon, par, frequency)
}

/// Calculates the accrued interest for securities with periodic interest 
/// payments.
///
/// __Syntax__: 
/// ```ods
///     ACCRINT( Issue DateParam; First DateParam; Settlement DateParam; Coupon Number; Par Number; Frequency Integer; B Basis )
/// ```
///
/// __Constraints__:
/// Issue < First < Settlement ; Coupon > 0; Par > 0
/// 
/// Frequency is one of the following values:
///
/// __Semantics__:
/// Calculates the accrued interest for securities with periodic interest 
/// payments. ACCRINT supports short, standard, and long Coupon periods.
/// 
/// If CalcMethod is TRUE (the default) then ACCRINT returns the sum of the 
/// accrued interest in each coupon period from issue date until settlement 
/// date. If CalcMethod is FALSE then ACCRINT returns the sum of the accrued 
/// interest in each coupon period from first interest date until settlement 
/// date. For each coupon period, the interest is Par * Coupon * 
/// YEARFRAC(start-of-period;end-of-period; B)
/// 
/// •Issue: The security's issue or dated date.
/// 
/// •First: The security's first interest date.
/// 
/// •Settlement: The security's settlement date.
/// 
/// •Coupon: The security's annual coupon rate.
/// 
/// •Par: The security's par value, that is, the principal to be paid at 
/// maturity.
/// 
/// •Frequency: The number of coupon payments per year.
/// 
/// •B: Indicates the day-count convention to use in the calculation. 4.11.7
/// 
/// •CalcMethod: A logical value that specifies how to treat the case where 
/// Settlement > First.
///
/// __See also__: "ACCRINTM", "YEARFRAC", 
#[inline]
pub fn accrint_<A: DateTime, B: DateTime, C: DateTime, D: Number, E: Number>(issue: A, first: B, settlement: C, coupon: D, par: E, frequency: Frequency, b: YearFracMethod) -> FnNumber7<A, B, C, D, E, Frequency, YearFracMethod> {
    FnNumber7("ACCRINT", issue, first, settlement, coupon, par, frequency, b)
}

/// Calculates the accrued interest for securities with periodic interest 
/// payments.
///
/// __Syntax__: 
/// ```ods
///     ACCRINT( Issue DateParam; First DateParam; Settlement DateParam; Coupon Number; Par Number; Frequency Integer; B Basis; CalcMethod Logical )
/// ```
///
/// __Constraints__:
/// Issue < First < Settlement ; Coupon > 0; Par > 0
/// 
/// Frequency is one of the following values:
///
/// __Semantics__:
/// Calculates the accrued interest for securities with periodic interest 
/// payments. ACCRINT supports short, standard, and long Coupon periods.
/// 
/// If CalcMethod is TRUE (the default) then ACCRINT returns the sum of the 
/// accrued interest in each coupon period from issue date until settlement 
/// date. If CalcMethod is FALSE then ACCRINT returns the sum of the accrued 
/// interest in each coupon period from first interest date until settlement 
/// date. For each coupon period, the interest is Par * Coupon * 
/// YEARFRAC(start-of-period;end-of-period; B)
/// 
/// •Issue: The security's issue or dated date.
/// 
/// •First: The security's first interest date.
/// 
/// •Settlement: The security's settlement date.
/// 
/// •Coupon: The security's annual coupon rate.
/// 
/// •Par: The security's par value, that is, the principal to be paid at 
/// maturity.
/// 
/// •Frequency: The number of coupon payments per year.
/// 
/// •B: Indicates the day-count convention to use in the calculation. 4.11.7
/// 
/// •CalcMethod: A logical value that specifies how to treat the case where 
/// Settlement > First.
///
/// __See also__: "ACCRINTM", "YEARFRAC", 
#[inline]
pub fn accrint__<A: DateTime, B: DateTime, C: DateTime, D: Number, E: Number, F: Logical>(issue: A, first: B, settlement: C, coupon: D, par: E, frequency: Frequency, b: YearFracMethod, calc_method: F) -> FnNumber8<A, B, C, D, E, Frequency, YearFracMethod, F> {
    FnNumber8("ACCRINT", issue, first, settlement, coupon, par, frequency, b, calc_method)
}

/// Calculates the accrued interest for securities that pay at maturity.
///
/// __Syntax__: 
/// ```ods
///     ACCRINTM( Issue DateParam; Settlement DateParam; Coupon Number; Par Number )
/// ```
///
/// __Constraints__:
/// Coupon > 0; Par > 0
///
/// __Semantics__:
/// Calculates the accrued interest for securities that pay at maturity.
/// 
/// •Issue: The security's issue or dated date.
/// 
/// •Settlement: The security's maturity date.
/// 
/// •Coupon: The security's annual coupon rate.
/// 
/// •Par: The security's par value, that is, the principal to be paid at 
/// maturity.
/// 
/// •B: Indicates the day-count convention to use in the calculation. 4.11.7
///
/// __See also__: "ACCRINT", 
#[inline]
pub fn accrintm<A: DateTime, B: DateTime, C: Number, D: Number>(issue: A, settlement: B, coupon: C, par: D) -> FnNumber4<A, B, C, D> {
    FnNumber4("ACCRINTM", issue, settlement, coupon, par)
}

/// Calculates the accrued interest for securities that pay at maturity.
///
/// __Syntax__: 
/// ```ods
///     ACCRINTM( Issue DateParam; Settlement DateParam; Coupon Number; Par Number; B Basis )
/// ```
///
/// __Constraints__:
/// Coupon > 0; Par > 0
///
/// __Semantics__:
/// Calculates the accrued interest for securities that pay at maturity.
/// 
/// •Issue: The security's issue or dated date.
/// 
/// •Settlement: The security's maturity date.
/// 
/// •Coupon: The security's annual coupon rate.
/// 
/// •Par: The security's par value, that is, the principal to be paid at 
/// maturity.
/// 
/// •B: Indicates the day-count convention to use in the calculation. 4.11.7
///
/// __See also__: "ACCRINT", 
#[inline]
pub fn accrintm_<A: DateTime, B: DateTime, C: Number, D: Number>(issue: A, settlement: B, coupon: C, par: D, b: YearFracMethod) -> FnNumber5<A, B, C, D, YearFracMethod> {
    FnNumber5("ACCRINTM", issue, settlement, coupon, par, b)
}

/// Calculates the amortization value for the French accounting system using 
/// linear depreciation (l'amortissement linéaire comptable) .
///
/// __Syntax__: 
/// ```ods
///     AMORLINC( Cost Number; PurchaseDate DateParam; FirstPeriodEndDate DateParam; Salvage Number; Period Integer; Rate Number )
/// ```
///
/// __Constraints__:
/// Cost > 0; PurchaseDate ≤ FirstPeriodEndDate; Salvage ≥ 0; Period ≥ 0; 
/// Rate > 0
///
/// __Semantics__:
/// Calculates the amortization value for the French accounting system using 
/// linear depreciation.
/// 
/// •Cost: The value of the asset at the date of aquisition.
/// 
/// •PurchaseDate: The date of aquisition.
/// 
/// •FirstPeriodEndDate: The end date of the first depreciation period.
/// 
/// •Salvage: The value of the asset at the end of the depreciation life 
/// time.
/// 
/// •Period: Which period the depreciation should be calculated for.
/// 
/// •Rate: The rate of depreciation.
/// 
/// •B: Indicates the day-count convention to use in the calculation. 4.11.7
/// 
/// When Period = 0:
/// 
/// For full periods, where Period > 0, the depreciation is Cost * Rate
/// 
/// For the last period, possibly a partial period: the depreciation = Cost - 
/// Salvage - accumulated-depreciation, where accumulated-depreciation is the 
/// sum of the depreciation in period 0 plus any full period depreciations.
/// 
/// When Period > depreciated life of the asset, i.e., when Period > (Cost - 
/// Salvage) / (Cost * Rate) then the depreciation is 0.
///
/// __Note__:
/// The behavior of this function is implementation-defined in cases where 
/// PurchaseDate = FirstPeriodEndDate.
///
/// __See also__: "DB", "DDB", "YEARFRAC", 
#[inline]
pub fn amorlinc<A: Number, B: DateTime, C: DateTime, D: Number, E: Number, F: Number>(cost: A, purchase_date: B, first_period_end_date: C, salvage: D, period: E, rate: F) -> FnNumber6<A, B, C, D, E, F> {
    FnNumber6("AMORLINC", cost, purchase_date, first_period_end_date, salvage, period, rate)
}

/// Calculates the amortization value for the French accounting system using 
/// linear depreciation (l'amortissement linéaire comptable) .
///
/// __Syntax__: 
/// ```ods
///     AMORLINC( Cost Number; PurchaseDate DateParam; FirstPeriodEndDate DateParam; Salvage Number; Period Integer; Rate Number; B Basis )
/// ```
///
/// __Constraints__:
/// Cost > 0; PurchaseDate ≤ FirstPeriodEndDate; Salvage ≥ 0; Period ≥ 0; 
/// Rate > 0
///
/// __Semantics__:
/// Calculates the amortization value for the French accounting system using 
/// linear depreciation.
/// 
/// •Cost: The value of the asset at the date of aquisition.
/// 
/// •PurchaseDate: The date of aquisition.
/// 
/// •FirstPeriodEndDate: The end date of the first depreciation period.
/// 
/// •Salvage: The value of the asset at the end of the depreciation life 
/// time.
/// 
/// •Period: Which period the depreciation should be calculated for.
/// 
/// •Rate: The rate of depreciation.
/// 
/// •B: Indicates the day-count convention to use in the calculation. 4.11.7
/// 
/// When Period = 0:
/// 
/// For full periods, where Period > 0, the depreciation is Cost * Rate
/// 
/// For the last period, possibly a partial period: the depreciation = Cost - 
/// Salvage - accumulated-depreciation, where accumulated-depreciation is the 
/// sum of the depreciation in period 0 plus any full period depreciations.
/// 
/// When Period > depreciated life of the asset, i.e., when Period > (Cost - 
/// Salvage) / (Cost * Rate) then the depreciation is 0.
///
/// __Note__:
/// The behavior of this function is implementation-defined in cases where 
/// PurchaseDate = FirstPeriodEndDate.
///
/// __See also__: "DB", "DDB", "YEARFRAC", 
#[inline]
pub fn amorlinc_<A: Number, B: DateTime, C: DateTime, D: Number, E: Number, F: Number>(cost: A, purchase_date: B, first_period_end_date: C, salvage: D, period: E, rate: F, b: YearFracMethod) -> FnNumber7<A, B, C, D, E, F, YearFracMethod> {
    FnNumber7("AMORLINC", cost, purchase_date, first_period_end_date, salvage, period, rate, b)
}

/// Calculates the number of days between the beginning of the coupon period 
/// that contains the settlement date and the settlement date.
///
/// __Syntax__: 
/// ```ods
///     COUPDAYBS( Settlement DateParam; Maturity DateParam; Frequency Integer )
/// ```
///
/// __Constraints__:
/// Settlement < Maturity
/// 
/// Frequency is one of the following values:
///
/// __Semantics__:
/// Calculate the number of days from the beginning of the coupon period to the 
/// settlement date.
/// 
/// •Settlement: The settlement date.
/// 
/// •Maturity: The maturity date.
/// 
/// •Frequency: The number of coupon payments per year.
/// 
/// •B: Indicates the day-count convention to use in the calculation. 4.11.7
///
/// __See also__: "COUPDAYS", "COUPDAYSNC", "COUPNCD", "COUPNUM", "COUPPCD", 
#[inline]
pub fn coupdaybs<A: DateTime, B: DateTime>(settlement: A, maturity: B, frequency: Frequency) -> FnNumber3<A, B, Frequency> {
    FnNumber3("COUPDAYBS", settlement, maturity, frequency)
}

/// Calculates the number of days between the beginning of the coupon period 
/// that contains the settlement date and the settlement date.
///
/// __Syntax__: 
/// ```ods
///     COUPDAYBS( Settlement DateParam; Maturity DateParam; Frequency Integer; B Basis )
/// ```
///
/// __Constraints__:
/// Settlement < Maturity
/// 
/// Frequency is one of the following values:
///
/// __Semantics__:
/// Calculate the number of days from the beginning of the coupon period to the 
/// settlement date.
/// 
/// •Settlement: The settlement date.
/// 
/// •Maturity: The maturity date.
/// 
/// •Frequency: The number of coupon payments per year.
/// 
/// •B: Indicates the day-count convention to use in the calculation. 4.11.7
///
/// __See also__: "COUPDAYS", "COUPDAYSNC", "COUPNCD", "COUPNUM", "COUPPCD", 
#[inline]
pub fn coupdaybs_<A: DateTime, B: DateTime>(settlement: A, maturity: B, frequency: Frequency, b: YearFracMethod) -> FnNumber4<A, B, Frequency, YearFracMethod> {
    FnNumber4("COUPDAYBS", settlement, maturity, frequency, b)
}

/// Calculates the number of days in a coupon period that contains the 
/// settlement date.
///
/// __Syntax__: 
/// ```ods
///     COUPDAYS( Settlement DateParam; Maturity DateParam; Frequency Integer )
/// ```
///
/// __Constraints__:
/// Settlement < Maturity
/// 
/// Frequency is one of the following values:
///
/// __Semantics__:
/// Calculates the number of days in the coupon period containing the 
/// settlement date.
/// 
/// •Settlement: The settlement date.
/// 
/// •Maturity: The maturity date.
/// 
/// •Frequency: The number of coupon payments per year.
/// 
/// •B: Indicates the day-count convention to use in the calculation. 4.11.7
///
/// __See also__: "COUPDAYBS", "COUPDAYSNC", "COUPNCD", "COUPNUM", "COUPPCD", 
#[inline]
pub fn coupdays<A: DateTime, B: DateTime>(settlement: A, maturity: B, frequency: Frequency) -> FnNumber3<A, B, Frequency> {
    FnNumber3("COUPDAYS", settlement, maturity, frequency)
}

/// Calculates the number of days in a coupon period that contains the 
/// settlement date.
///
/// __Syntax__: 
/// ```ods
///     COUPDAYS( Settlement DateParam; Maturity DateParam; Frequency Integer; B Basis )
/// ```
///
/// __Constraints__:
/// Settlement < Maturity
/// 
/// Frequency is one of the following values:
///
/// __Semantics__:
/// Calculates the number of days in the coupon period containing the 
/// settlement date.
/// 
/// •Settlement: The settlement date.
/// 
/// •Maturity: The maturity date.
/// 
/// •Frequency: The number of coupon payments per year.
/// 
/// •B: Indicates the day-count convention to use in the calculation. 4.11.7
///
/// __See also__: "COUPDAYBS", "COUPDAYSNC", "COUPNCD", "COUPNUM", "COUPPCD", 
#[inline]
pub fn coupdays_<A: DateTime, B: DateTime>(settlement: A, maturity: B, frequency: Frequency, b: YearFracMethod) -> FnNumber4<A, B, Frequency, YearFracMethod> {
    FnNumber4("COUPDAYS", settlement, maturity, frequency, b)
}

/// Calculates the number of days between a settlement date and the next coupon 
/// date.
///
/// __Syntax__: 
/// ```ods
///     COUPDAYNC( Settlement DateParam; Maturity DateParam; Frequency Integer )
/// ```
///
/// __Constraints__:
/// Settlement < Maturity
/// 
/// Frequency is one of the following values:
///
/// __Semantics__:
/// Calculates the number of days between the settlement date and the next 
/// coupon date.
/// 
/// •Settlement: The settlement date.
/// 
/// •Maturity: The maturity date.
/// 
/// •Frequency: The number of coupon payments per year.
/// 
/// •B: Indicates the day-count convention to use in the calculation. 4.11.7
///
/// __See also__: "COUPDAYBS", "COUPDAYS", "COUPNCD", "COUPNUM", "COUPPCD", 
#[inline]
pub fn coupdaync<A: DateTime, B: DateTime>(settlement: A, maturity: B, frequency: Frequency) -> FnNumber3<A, B, Frequency> {
    FnNumber3("COUPDAYNC", settlement, maturity, frequency)
}

/// Calculates the number of days between a settlement date and the next coupon 
/// date.
///
/// __Syntax__: 
/// ```ods
///     COUPDAYNC( Settlement DateParam; Maturity DateParam; Frequency Integer; B Basis )
/// ```
///
/// __Constraints__:
/// Settlement < Maturity
/// 
/// Frequency is one of the following values:
///
/// __Semantics__:
/// Calculates the number of days between the settlement date and the next 
/// coupon date.
/// 
/// •Settlement: The settlement date.
/// 
/// •Maturity: The maturity date.
/// 
/// •Frequency: The number of coupon payments per year.
/// 
/// •B: Indicates the day-count convention to use in the calculation. 4.11.7
///
/// __See also__: "COUPDAYBS", "COUPDAYS", "COUPNCD", "COUPNUM", "COUPPCD", 
#[inline]
pub fn coupdaync_<A: DateTime, B: DateTime>(settlement: A, maturity: B, frequency: Frequency, b: YearFracMethod) -> FnNumber4<A, B, Frequency, YearFracMethod> {
    FnNumber4("COUPDAYNC", settlement, maturity, frequency, b)
}

/// Calculates the next coupon date following a settlement.
///
/// __Syntax__: 
/// ```ods
///     COUPNCD( Settlement DateParam; Maturity DateParam; Frequency Integer )
/// ```
///
/// __Constraints__:
/// Settlement < Maturity
/// 
/// Frequency is the number of coupon payments per year. Frequency is one of 
/// the following values:
///
/// __Semantics__:
/// Calculates the next coupon date after the Settlement date based on the 
/// Maturity (expiration) date of the asset, the Frequency of coupon payments 
/// and the day-count B.
/// 
/// B indicates the day-count convention to use in the calculation. 4.11.7
///
/// __See also__: "COUPDAYSNC", 
#[inline]
pub fn coupncd<A: DateTime, B: DateTime>(settlement: A, maturity: B, frequency: Frequency) -> FnNumber3<A, B, Frequency> {
    FnNumber3("COUPNCD", settlement, maturity, frequency)
}

/// Calculates the next coupon date following a settlement.
///
/// __Syntax__: 
/// ```ods
///     COUPNCD( Settlement DateParam; Maturity DateParam; Frequency Integer; B Basis )
/// ```
///
/// __Constraints__:
/// Settlement < Maturity
/// 
/// Frequency is the number of coupon payments per year. Frequency is one of 
/// the following values:
///
/// __Semantics__:
/// Calculates the next coupon date after the Settlement date based on the 
/// Maturity (expiration) date of the asset, the Frequency of coupon payments 
/// and the day-count B.
/// 
/// B indicates the day-count convention to use in the calculation. 4.11.7
///
/// __See also__: "COUPDAYSNC", 
#[inline]
pub fn coupncd_<A: DateTime, B: DateTime>(settlement: A, maturity: B, frequency: Frequency, b: YearFracMethod) -> FnNumber4<A, B, Frequency, YearFracMethod> {
    FnNumber4("COUPNCD", settlement, maturity, frequency, b)
}

/// Calculates the number of outstanding coupons between settlement and 
/// maturity dates.
///
/// __Syntax__: 
/// ```ods
///     COUPNUM( Settlement DateParam; Maturity DateParam; Frequency Integer )
/// ```
///
/// __Constraints__:
/// Frequency is the number of coupon payments per year. Frequency is one of 
/// the following values:
///
/// __Semantics__:
/// Calculates the number of coupons in the interval between the Settlement and 
/// the Maturity (expiration) date of the asset, the Frequency of coupon 
/// payments and the day-count B.
/// 
/// B indicates the day-count convention to use in the calculation. 4.11.7
///
/// __See also__: "COUPDAYBS", "COUPDAYS", "COUPDAYSNC", "COUPNCD", "COUPPCD", 
#[inline]
pub fn coupnum<A: DateTime, B: DateTime>(settlement: A, maturity: B, frequency: Frequency) -> FnNumber3<A, B, Frequency> {
    FnNumber3("COUPNUM", settlement, maturity, frequency)
}

/// Calculates the number of outstanding coupons between settlement and 
/// maturity dates.
///
/// __Syntax__: 
/// ```ods
///     COUPNUM( Settlement DateParam; Maturity DateParam; Frequency Integer; B Basis )
/// ```
///
/// __Constraints__:
/// Frequency is the number of coupon payments per year. Frequency is one of 
/// the following values:
///
/// __Semantics__:
/// Calculates the number of coupons in the interval between the Settlement and 
/// the Maturity (expiration) date of the asset, the Frequency of coupon 
/// payments and the day-count B.
/// 
/// B indicates the day-count convention to use in the calculation. 4.11.7
///
/// __See also__: "COUPDAYBS", "COUPDAYS", "COUPDAYSNC", "COUPNCD", "COUPPCD", 
#[inline]
pub fn coupnum_<A: DateTime, B: DateTime>(settlement: A, maturity: B, frequency: Frequency, b: YearFracMethod) -> FnNumber4<A, B, Frequency, YearFracMethod> {
    FnNumber4("COUPNUM", settlement, maturity, frequency, b)
}

/// Calculates the next coupon date prior a settlement.
///
/// __Syntax__: 
/// ```ods
///     COUPPCD( Settlement DateParam; Maturity DateParam; Frequency Integer )
/// ```
///
/// __Constraints__:
/// Settlement < Maturity
/// 
/// Frequency is the number of coupon payments per year. Frequency is one of 
/// the following values:
///
/// __Semantics__:
/// Calculates the next coupon date prior to the Settlement date based on the 
/// Maturity (expiration) date of the asset, the Frequency of coupon payments 
/// and the day-count B.
/// 
/// B indicates the day-count convention to use in the calculation. 4.11.7
///
/// __See also__: "COUPDAYBS", "COUPDAYS", "COUPDAYSNC", "COUPNCD", "COUPNUM", 
#[inline]
pub fn couppcd<A: DateTime, B: DateTime>(settlement: A, maturity: B, frequency: Frequency) -> FnNumber3<A, B, Frequency> {
    FnNumber3("COUPPCD", settlement, maturity, frequency)
}

/// Calculates the next coupon date prior a settlement.
///
/// __Syntax__: 
/// ```ods
///     COUPPCD( Settlement DateParam; Maturity DateParam; Frequency Integer; B Basis )
/// ```
///
/// __Constraints__:
/// Settlement < Maturity
/// 
/// Frequency is the number of coupon payments per year. Frequency is one of 
/// the following values:
///
/// __Semantics__:
/// Calculates the next coupon date prior to the Settlement date based on the 
/// Maturity (expiration) date of the asset, the Frequency of coupon payments 
/// and the day-count B.
/// 
/// B indicates the day-count convention to use in the calculation. 4.11.7
///
/// __See also__: "COUPDAYBS", "COUPDAYS", "COUPDAYSNC", "COUPNCD", "COUPNUM", 
#[inline]
pub fn couppcd_<A: DateTime, B: DateTime>(settlement: A, maturity: B, frequency: Frequency, b: YearFracMethod) -> FnNumber4<A, B, Frequency, YearFracMethod> {
    FnNumber4("COUPPCD", settlement, maturity, frequency, b)
}

/// Calculates a cumulative interest payment.
///
/// __Syntax__: 
/// ```ods
///     CUMIPMT( Rate Number; Periods Number; Value Number; Start Integer; End Integer; Type Integer )
/// ```
///
/// __Constraints__:
/// Rate > 0; Value > 0; 1 ≤ Start ≤ End ≤ Periods
/// 
/// Type is one of the following values:
/// 
/// due at the beginning
///
/// __Semantics__:
/// Calculates the cumulative interest payment.
/// 
/// •Rate: The interest rate per period.
/// 
/// •Periods: The number of periods.
/// 
/// •Value: The current value of the loan.
/// 
/// •Start: The starting period.
/// 
/// •End: The end period.
/// 
/// •Type: The maturity date, the beginning or the end of a period.
///
/// __See also__: "IPMT", "CUMPRINC", 
#[inline]
pub fn cumipmt<A: Number, B: Number, C: Number, D: Number, E: Number>(rate: A, periods: B, value: C, start: D, end: E, type_: MaturityDate) -> FnNumber6<A, B, C, D, E, MaturityDate> {
    FnNumber6("CUMIPMT", rate, periods, value, start, end, type_)
}

/// Calculates a cumulative principal payment.
///
/// __Syntax__: 
/// ```ods
///     CUMPRINC( Rate Number; Periods Number; Value Number; Start Integer; End Integer; Type Integer )
/// ```
///
/// __Constraints__:
/// Type is one of the following values:
///
/// __Semantics__:
/// Calculates the cumulative principal payment.
/// 
/// •Rate: The interest rate per period.
/// 
/// •Periods: The number of periods.
/// 
/// •Value: The current value of the loan.
/// 
/// •Start: The starting period.
/// 
/// •End: The end period.
/// 
/// •Type: The maturity date, the beginning or the end of a period.
///
/// __See also__: "PPMT", "CUMIPMT", 
#[inline]
pub fn cumprinc<A: Number, B: Number, C: Number, D: Number, E: Number>(rate: A, periods: B, value: C, start: D, end: E, type_: MaturityDate) -> FnNumber6<A, B, C, D, E, MaturityDate> {
    FnNumber6("CUMPRINC", rate, periods, value, start, end, type_)
}

/// Compute the depreciation allowance of an asset.
///
/// __Syntax__: 
/// ```ods
///     DB( Cost Number; Salvage Number; LifeTime Integer; Period Number )
/// ```
///
/// __Constraints__:
/// Cost > 0, Salvage ≥ 0, LifeTime > 0; Period > 0; 0 < Month < 13
///
/// __Semantics__:
/// Calculate the depreciation allowance of an asset with an initial value of 
/// Cost, an expected useful LifeTime, and a final Salvage value at a specified 
/// Period of time, using the fixed-declining balance method. The parameters 
/// are:
/// 
/// •Cost: the total amount paid for the asset.
/// 
/// •Salvage: the salvage value at the end of the LifeTime.
/// 
/// •LifeTime: the number of periods that the depreciation will occur over. A 
/// positive integer.
/// 
/// •Period: the time period for which you want to find the depreciation 
/// allowance, in the same units as LifeTime.
/// 
/// •Month: (optional) the number of months in the first year of 
/// depreciation, assumed to be 12, if not specified. If a value is specified 
/// for Month, LifeTime and Period are assumed to be measured in years.
/// 
/// The rate is calculated as follows:
/// 
/// and is rounded to 3 decimals.
/// 
/// For the first period the residual value is
/// 
/// For all periods, where Period ≤ LifeTime, the residual value is 
/// calculated by
/// 
/// If Month was specified, the residual value for the period after LifeTime 
/// becomes
/// 
/// The depreciation allowance for the first period is
/// 
/// For all other periods the allowance is calculated by
/// 
/// For all periods, where Period > LifeTime + 1 – INT(Month / 12), the 
/// depreciation allowance is zero.
///
/// __See also__: "DDB", "SLN", "INT", 
#[inline]
pub fn db<A: Number, B: Number, C: Number, D: Number>(cost: A, salvage: B, life_time: C, period: D) -> FnNumber4<A, B, C, D> {
    FnNumber4("DB", cost, salvage, life_time, period)
}

/// Compute the depreciation allowance of an asset.
///
/// __Syntax__: 
/// ```ods
///     DB( Cost Number; Salvage Number; LifeTime Integer; Period Number; Month Number )
/// ```
///
/// __Constraints__:
/// Cost > 0, Salvage ≥ 0, LifeTime > 0; Period > 0; 0 < Month < 13
///
/// __Semantics__:
/// Calculate the depreciation allowance of an asset with an initial value of 
/// Cost, an expected useful LifeTime, and a final Salvage value at a specified 
/// Period of time, using the fixed-declining balance method. The parameters 
/// are:
/// 
/// •Cost: the total amount paid for the asset.
/// 
/// •Salvage: the salvage value at the end of the LifeTime.
/// 
/// •LifeTime: the number of periods that the depreciation will occur over. A 
/// positive integer.
/// 
/// •Period: the time period for which you want to find the depreciation 
/// allowance, in the same units as LifeTime.
/// 
/// •Month: (optional) the number of months in the first year of 
/// depreciation, assumed to be 12, if not specified. If a value is specified 
/// for Month, LifeTime and Period are assumed to be measured in years.
/// 
/// The rate is calculated as follows:
/// 
/// and is rounded to 3 decimals.
/// 
/// For the first period the residual value is
/// 
/// For all periods, where Period ≤ LifeTime, the residual value is 
/// calculated by
/// 
/// If Month was specified, the residual value for the period after LifeTime 
/// becomes
/// 
/// The depreciation allowance for the first period is
/// 
/// For all other periods the allowance is calculated by
/// 
/// For all periods, where Period > LifeTime + 1 – INT(Month / 12), the 
/// depreciation allowance is zero.
///
/// __See also__: "DDB", "SLN", "INT", 
#[inline]
pub fn db_<A: Number, B: Number, C: Number, D: Number, E: Number>(cost: A, salvage: B, life_time: C, period: D, month: E) -> FnNumber5<A, B, C, D, E> {
    FnNumber5("DB", cost, salvage, life_time, period, month)
}

/// Compute the amount of depreciation at a given period of time.
///
/// __Syntax__: 
/// ```ods
///     DDB( Cost Number; Salvage Number; LifeTime Number; Period Number )
/// ```
///
/// __Constraints__:
/// Cost ≥ 0, Salvage ≥ 0, Salvage ≤ Cost, 1 ≤ Period ≤ LifeTime, 
/// DeclinationFactor > 0
///
/// __Semantics__:
/// Compute the amount of depreciation of an asset at a given period of time. 
/// The parameters are:
/// 
/// •Cost: the total amount paid for the asset.
/// 
/// •Salvage: the salvage value at the end of the LifeTime
/// 
/// •LifeTime: the number of periods that the depreciation will occur over.
/// 
/// •Period: the period for which a depreciation value is specified.
/// 
/// •DeclinationFactor: the method of calculating depreciation, the rate at 
/// which the balance declines. Defaults to 2. If 2, double-declining balance 
/// is used.
/// 
/// To calculate depreciation, DDB uses a fixed rate. When DeclinationFactor = 
/// 2 this is the double-declining-balance method (because it is double the 
/// straight-line rate that would depreciate the asset to zero). The rate is 
/// given by:
/// 
/// The depreciation each period is calculated as
/// 
/// depreciation_of_period = MIN( book_value_at_start_of_ period * rate; 
/// book_value_at_start_of_ period - Salvage )
/// 
/// Thus the asset depreciates at rate until the book value is Salvage value.
/// 
/// To allow also non-integer Period values this algorithm may be used:
/// 
/// If Period is an Integer number, the relation between DDB and VDB is:
/// DDB( Cost ; Salvage ; LifeTime ; Period ; DeclinationFactor )
/// equals
/// VDB( Cost ; Salvage ; LifeTime ; Period - 1 ; Period ; DeclinationFactor ; 
/// TRUE )
///
/// __See also__: "SLN", "VDB", "MIN", 
#[inline]
pub fn ddb<A: Number, B: Number, C: Number, D: Number>(cost: A, salvage: B, life_time: C, period: D) -> FnNumber4<A, B, C, D> {
    FnNumber4("DDB", cost, salvage, life_time, period)
}

/// Compute the amount of depreciation at a given period of time.
///
/// __Syntax__: 
/// ```ods
///     DDB( Cost Number; Salvage Number; LifeTime Number; Period Number; DeclinationFactor Number )
/// ```
///
/// __Constraints__:
/// Cost ≥ 0, Salvage ≥ 0, Salvage ≤ Cost, 1 ≤ Period ≤ LifeTime, 
/// DeclinationFactor > 0
///
/// __Semantics__:
/// Compute the amount of depreciation of an asset at a given period of time. 
/// The parameters are:
/// 
/// •Cost: the total amount paid for the asset.
/// 
/// •Salvage: the salvage value at the end of the LifeTime
/// 
/// •LifeTime: the number of periods that the depreciation will occur over.
/// 
/// •Period: the period for which a depreciation value is specified.
/// 
/// •DeclinationFactor: the method of calculating depreciation, the rate at 
/// which the balance declines. Defaults to 2. If 2, double-declining balance 
/// is used.
/// 
/// To calculate depreciation, DDB uses a fixed rate. When DeclinationFactor = 
/// 2 this is the double-declining-balance method (because it is double the 
/// straight-line rate that would depreciate the asset to zero). The rate is 
/// given by:
/// 
/// The depreciation each period is calculated as
/// 
/// depreciation_of_period = MIN( book_value_at_start_of_ period * rate; 
/// book_value_at_start_of_ period - Salvage )
/// 
/// Thus the asset depreciates at rate until the book value is Salvage value.
/// 
/// To allow also non-integer Period values this algorithm may be used:
/// 
/// If Period is an Integer number, the relation between DDB and VDB is:
/// DDB( Cost ; Salvage ; LifeTime ; Period ; DeclinationFactor )
/// equals
/// VDB( Cost ; Salvage ; LifeTime ; Period - 1 ; Period ; DeclinationFactor ; 
/// TRUE )
///
/// __See also__: "SLN", "VDB", "MIN", 
#[inline]
pub fn ddb_<A: Number, B: Number, C: Number, D: Number, E: Number>(cost: A, salvage: B, life_time: C, period: D, declination_factor: E) -> FnNumber5<A, B, C, D, E> {
    FnNumber5("DDB", cost, salvage, life_time, period, declination_factor)
}

/// Returns the discount rate of a security.
///
/// __Syntax__: 
/// ```ods
///     DISC( Settlement DateParam; Maturity DateParam; Price Number; Redemption Number )
/// ```
///
/// __Constraints__:
/// Settlement < Maturity
///
/// __Semantics__:
/// Calculates the discount rate of a security.
/// 
/// •Settlement: The settlement date of the security.
/// 
/// •Maturity: The maturity date.
/// 
/// •Price: The price of the security.
/// 
/// •Redemption: The redemption value of the security.
/// 
/// •B: Indicates the day-count convention to use in the calculation. 4.11.7
///
/// __See also__: "YEARFRAC", 
#[inline]
pub fn disc<A: DateTime, B: DateTime, C: Number, D: Number>(settlement: A, maturity: B, price: C, redemption: D) -> FnNumber4<A, B, C, D> {
    FnNumber4("DISC", settlement, maturity, price, redemption)
}

/// Returns the discount rate of a security.
///
/// __Syntax__: 
/// ```ods
///     DISC( Settlement DateParam; Maturity DateParam; Price Number; Redemption Number; B Basis )
/// ```
///
/// __Constraints__:
/// Settlement < Maturity
///
/// __Semantics__:
/// Calculates the discount rate of a security.
/// 
/// •Settlement: The settlement date of the security.
/// 
/// •Maturity: The maturity date.
/// 
/// •Price: The price of the security.
/// 
/// •Redemption: The redemption value of the security.
/// 
/// •B: Indicates the day-count convention to use in the calculation. 4.11.7
///
/// __See also__: "YEARFRAC", 
#[inline]
pub fn disc_<A: DateTime, B: DateTime, C: Number, D: Number>(settlement: A, maturity: B, price: C, redemption: D, b: YearFracMethod) -> FnNumber5<A, B, C, D, YearFracMethod> {
    FnNumber5("DISC", settlement, maturity, price, redemption, b)
}

/// Converts a fractional dollar representation into a decimal representation.
///
/// __Syntax__: 
/// ```ods
///     DOLLARDE( Fractional Number; Denominator Integer )
/// ```
///
/// __Constraints__:
/// Denominator > 0
///
/// __Semantics__:
/// Converts a fractional dollar representation into a decimal representation.
/// 
/// •Fractional: Decimal fraction.
/// 
/// •Denominator: The denominator of the fraction.
///
/// __See also__: "DOLLARFR", "TRUNC", 
#[inline]
pub fn dollarde<A: Number, B: Number>(fractional: A, denominator: B) -> FnNumber2<A, B> {
    FnNumber2("DOLLARDE", fractional, denominator)
}

/// Converts a decimal dollar representation into a fractional representation.
///
/// __Syntax__: 
/// ```ods
///     DOLLARFR( Decimal Number; Denominator Integer )
/// ```
///
/// __Constraints__:
/// Denominator > 0
///
/// __Semantics__:
/// Converts a decimal dollar representation into a fractional representation.
/// 
/// •Decimal: A decimal number.
/// 
/// •Denominator: The denominator of the fraction.
///
/// __See also__: "DOLLARDE", "TRUNC", 
#[inline]
pub fn dollarfr<A: Number, B: Number>(decimal: A, denominator: B) -> FnNumber2<A, B> {
    FnNumber2("DOLLARFR", decimal, denominator)
}

/// Returns the Macaulay duration of a fixed interest security in years
///
/// __Syntax__: 
/// ```ods
///     DURATION( Settlement Date; Maturity Date; Coupon Number; Yield Number; Frequency Number )
/// ```
///
/// __Constraints__:
/// Yield ≥0, Coupon ≥ 0, Settlement ≤ Maturity; Frequency = 1, 2, 4
///
/// __Semantics__:
/// Computes the Macaulay duration, given:
/// 
/// •Settlement: the date of purchase of the security
/// 
/// •Maturity: the date when the security matures
/// 
/// •Coupon: the annual nominal rate of interest
/// 
/// •Yield: the annual yield of the security
/// 
/// •Frequency: number of interest payments per year
/// 
/// •B: Indicates the day-count convention to use in the calculation. 4.11.7
///
/// __See also__: "MDURATION", 
#[inline]
pub fn duration<A: DateTime, B: DateTime, C: Number, D: Number>(settlement: A, maturity: B, coupon: C, yield_: D, frequency: Frequency) -> FnNumber5<A, B, C, D, Frequency> {
    FnNumber5("DURATION", settlement, maturity, coupon, yield_, frequency)
}

/// Returns the Macaulay duration of a fixed interest security in years
///
/// __Syntax__: 
/// ```ods
///     DURATION( Settlement Date; Maturity Date; Coupon Number; Yield Number; Frequency Number; B Basis )
/// ```
///
/// __Constraints__:
/// Yield ≥0, Coupon ≥ 0, Settlement ≤ Maturity; Frequency = 1, 2, 4
///
/// __Semantics__:
/// Computes the Macaulay duration, given:
/// 
/// •Settlement: the date of purchase of the security
/// 
/// •Maturity: the date when the security matures
/// 
/// •Coupon: the annual nominal rate of interest
/// 
/// •Yield: the annual yield of the security
/// 
/// •Frequency: number of interest payments per year
/// 
/// •B: Indicates the day-count convention to use in the calculation. 4.11.7
///
/// __See also__: "MDURATION", 
#[inline]
pub fn duration_<A: DateTime, B: DateTime, C: Number, D: Number>(settlement: A, maturity: B, coupon: C, yield_: D, frequency: Frequency, b: YearFracMethod) -> FnNumber6<A, B, C, D, Frequency, YearFracMethod> {
    FnNumber6("DURATION", settlement, maturity, coupon, yield_, frequency, b)
}

/// Returns the net annual interest rate for a nominal interest rate.
///
/// __Syntax__: 
/// ```ods
///     EFFECT( Rate Number; Payments Integer )
/// ```
///
/// __Constraints__:
/// Rate ≥ 0; Payments > 0
///
/// __Semantics__:
/// Nominal interest refers to the amount of interest due at the end of a 
/// calculation period. Effective interest increases with the number of 
/// payments made. In other words, interest is often paid in installments (for 
/// example, monthly or quarterly) before the end of the calculation period.
/// 
/// •Rate: The interest rate per period.
/// 
/// •Payments: The number of payments per period.
///
/// __See also__: "NOMINAL", 
#[inline]
pub fn effect<A: Number, B: Number>(rate: A, payments: B) -> FnNumber2<A, B> {
    FnNumber2("EFFECT", rate, payments)
}

/// Compute the future value (FV) of an investment.
///
/// __Syntax__: 
/// ```ods
///     FV( Rate Number; Nper Number; Payment Number )
/// ```
///
/// __Constraints__:
/// None.
///
/// __Semantics__:
/// Computes the future value of an investment. The parameters are:
/// 
/// •Rate: the interest rate per period.
/// 
/// •Nper: the total number of payment periods.
/// 
/// •Payment: the payment made in each period.
/// 
/// •Pv: the present value; default is 0.
/// 
/// •PayType: the type of payment, defaults to 0. It is 0 if payments are due 
/// at the end of the period; 1 if they are due at the beginning of the period.
/// 
/// See PV 6.12.41 for the equation this solves.
///
/// __See also__: "PV", "NPER", "PMT", "RATE", 
#[inline]
pub fn fv<A: Number, B: Number, C: Number>(rate: A, nper: B, payment: C) -> FnNumber3<A, B, C> {
    FnNumber3("FV", rate, nper, payment)
}

/// Compute the future value (FV) of an investment.
///
/// __Syntax__: 
/// ```ods
///     FV( Rate Number; Nper Number; Payment Number; Pv Number )
/// ```
///
/// __Constraints__:
/// None.
///
/// __Semantics__:
/// Computes the future value of an investment. The parameters are:
/// 
/// •Rate: the interest rate per period.
/// 
/// •Nper: the total number of payment periods.
/// 
/// •Payment: the payment made in each period.
/// 
/// •Pv: the present value; default is 0.
/// 
/// •PayType: the type of payment, defaults to 0. It is 0 if payments are due 
/// at the end of the period; 1 if they are due at the beginning of the period.
/// 
/// See PV 6.12.41 for the equation this solves.
///
/// __See also__: "PV", "NPER", "PMT", "RATE", 
#[inline]
pub fn fv_<A: Number, B: Number, C: Number, D: Number>(rate: A, nper: B, payment: C, pv: D) -> FnNumber4<A, B, C, D> {
    FnNumber4("FV", rate, nper, payment, pv)
}

/// Compute the future value (FV) of an investment.
///
/// __Syntax__: 
/// ```ods
///     FV( Rate Number; Nper Number; Payment Number; Pv Number; PayType Number )
/// ```
///
/// __Constraints__:
/// None.
///
/// __Semantics__:
/// Computes the future value of an investment. The parameters are:
/// 
/// •Rate: the interest rate per period.
/// 
/// •Nper: the total number of payment periods.
/// 
/// •Payment: the payment made in each period.
/// 
/// •Pv: the present value; default is 0.
/// 
/// •PayType: the type of payment, defaults to 0. It is 0 if payments are due 
/// at the end of the period; 1 if they are due at the beginning of the period.
/// 
/// See PV 6.12.41 for the equation this solves.
///
/// __See also__: "PV", "NPER", "PMT", "RATE", 
#[inline]
pub fn fv__<A: Number, B: Number, C: Number, D: Number, E: Number>(rate: A, nper: B, payment: C, pv: D, pay_type: E) -> FnNumber5<A, B, C, D, E> {
    FnNumber5("FV", rate, nper, payment, pv, pay_type)
}

/// Returns the accumulated value given starting capital and a series of 
/// interest rates.
///
/// __Syntax__: 
/// ```ods
///     FVSCHEDULE( Principal Number; Schedule NumberSequence )
/// ```
///
/// __Constraints__:
/// None.
///
/// __Semantics__:
/// Returns the accumulated value given starting capital and a series of 
/// interest rates, as follows:
///
/// __See also__: "PV", "NPER", "PMT", "RATE", 
#[inline]
pub fn fvschedule<A: Number, B: Sequence>(principal: A, schedule: B) -> FnNumber2<A, B> {
    FnNumber2("FVSCHEDULE", principal, schedule)
}

/// Computes the interest rate of a fully vested security.
///
/// __Syntax__: 
/// ```ods
///     INTRATE( Settlement Date; Maturity Date; Investment Number; Redemption Number )
/// ```
///
/// __Constraints__:
/// Settlement < Maturity
///
/// __Semantics__:
/// Calculates the annual interest rate that results when an item is purchased 
/// at the investment price and sold at the redemption price. No interest is 
/// paid on the investment. The parameters are:
/// 
/// •Settlement: the date of purchase of the security.
/// 
/// •Maturity: the date on which the security is sold.
/// 
/// •Investment: the purchase price.
/// 
/// •Redemption: the selling price.
/// 
/// •Basis: indicates the day-count convention to use in the calculation. 
/// 4.11.7
/// 
/// The return value for this function is:
///
/// __See also__: "RECEIVED", "YEARFRAC", 
#[inline]
pub fn intrate<A: DateTime, B: DateTime, C: Number, D: Number>(settlement: A, maturity: B, investment: C, redemption: D) -> FnNumber4<A, B, C, D> {
    FnNumber4("INTRATE", settlement, maturity, investment, redemption)
}

/// Computes the interest rate of a fully vested security.
///
/// __Syntax__: 
/// ```ods
///     INTRATE( Settlement Date; Maturity Date; Investment Number; Redemption Number; Basis Basis )
/// ```
///
/// __Constraints__:
/// Settlement < Maturity
///
/// __Semantics__:
/// Calculates the annual interest rate that results when an item is purchased 
/// at the investment price and sold at the redemption price. No interest is 
/// paid on the investment. The parameters are:
/// 
/// •Settlement: the date of purchase of the security.
/// 
/// •Maturity: the date on which the security is sold.
/// 
/// •Investment: the purchase price.
/// 
/// •Redemption: the selling price.
/// 
/// •Basis: indicates the day-count convention to use in the calculation. 
/// 4.11.7
/// 
/// The return value for this function is:
///
/// __See also__: "RECEIVED", "YEARFRAC", 
#[inline]
pub fn intrate_<A: DateTime, B: DateTime, C: Number, D: Number>(settlement: A, maturity: B, investment: C, redemption: D, basis: YearFracMethod) -> FnNumber5<A, B, C, D, YearFracMethod> {
    FnNumber5("INTRATE", settlement, maturity, investment, redemption, basis)
}

/// Returns the amount of an annuity payment going towards interest.
///
/// __Syntax__: 
/// ```ods
///     IPMT( Rate Number; Period Number; Nper Number; PV Number )
/// ```
///
/// __Constraints__:
/// None.
///
/// __Semantics__:
/// Computes the interest portion of an amortized payment for a constant 
/// interest rate and regular payments. The interest payment is the interest 
/// rate multiplied by the balance at the beginning of the period. The 
/// parameters are:
/// 
/// •Rate: The periodic interest rate.
/// 
/// •Period: The period for which the interest payment is computed.
/// 
/// •Nper: The total number of periods for which the payments are made
/// 
/// •PV: The present value (e.g. The initial loan amount).
/// 
/// •FV: The future value (optional) at the end of the periods. Zero if 
/// omitted.
/// 
/// •Type: the due date for the payments (optional). Zero if omitted. If Type 
/// is 1, then payments are made at the beginning of each period. If Type is 0, 
/// then payments are made at the end of each period.
///
/// __See also__: "PPMT", "PMT", 
#[inline]
pub fn ipmt<A: Number, B: Number, C: Number, D: Number>(rate: A, period: B, nper: C, p_v: D) -> FnNumber4<A, B, C, D> {
    FnNumber4("IPMT", rate, period, nper, p_v)
}

/// Returns the amount of an annuity payment going towards interest.
///
/// __Syntax__: 
/// ```ods
///     IPMT( Rate Number; Period Number; Nper Number; PV Number; FV Number )
/// ```
///
/// __Constraints__:
/// None.
///
/// __Semantics__:
/// Computes the interest portion of an amortized payment for a constant 
/// interest rate and regular payments. The interest payment is the interest 
/// rate multiplied by the balance at the beginning of the period. The 
/// parameters are:
/// 
/// •Rate: The periodic interest rate.
/// 
/// •Period: The period for which the interest payment is computed.
/// 
/// •Nper: The total number of periods for which the payments are made
/// 
/// •PV: The present value (e.g. The initial loan amount).
/// 
/// •FV: The future value (optional) at the end of the periods. Zero if 
/// omitted.
/// 
/// •Type: the due date for the payments (optional). Zero if omitted. If Type 
/// is 1, then payments are made at the beginning of each period. If Type is 0, 
/// then payments are made at the end of each period.
///
/// __See also__: "PPMT", "PMT", 
#[inline]
pub fn ipmt_<A: Number, B: Number, C: Number, D: Number, E: Number>(rate: A, period: B, nper: C, p_v: D, f_v: E) -> FnNumber5<A, B, C, D, E> {
    FnNumber5("IPMT", rate, period, nper, p_v, f_v)
}

/// Returns the amount of an annuity payment going towards interest.
///
/// __Syntax__: 
/// ```ods
///     IPMT( Rate Number; Period Number; Nper Number; PV Number; FV Number; Type Number )
/// ```
///
/// __Constraints__:
/// None.
///
/// __Semantics__:
/// Computes the interest portion of an amortized payment for a constant 
/// interest rate and regular payments. The interest payment is the interest 
/// rate multiplied by the balance at the beginning of the period. The 
/// parameters are:
/// 
/// •Rate: The periodic interest rate.
/// 
/// •Period: The period for which the interest payment is computed.
/// 
/// •Nper: The total number of periods for which the payments are made
/// 
/// •PV: The present value (e.g. The initial loan amount).
/// 
/// •FV: The future value (optional) at the end of the periods. Zero if 
/// omitted.
/// 
/// •Type: the due date for the payments (optional). Zero if omitted. If Type 
/// is 1, then payments are made at the beginning of each period. If Type is 0, 
/// then payments are made at the end of each period.
///
/// __See also__: "PPMT", "PMT", 
#[inline]
pub fn ipmt__<A: Number, B: Number, C: Number, D: Number, E: Number, F: Number>(rate: A, period: B, nper: C, p_v: D, f_v: E, type_: F) -> FnNumber6<A, B, C, D, E, F> {
    FnNumber6("IPMT", rate, period, nper, p_v, f_v, type_)
}

/// Compute the internal rate of return for a series of cash flows.
///
/// __Syntax__: 
/// ```ods
///     IRR( Values NumberSequence )
/// ```
///
/// __Constraints__:
/// None.
///
/// __Semantics__:
/// Compute the internal rate of return for a series of cash flows.
/// 
/// If provided, Guess is an estimate of the interest rate to start the 
/// iterative computation. If omitted, the value 0.1 (10%) is assumed.
/// 
/// The result of IRR is the rate at which the NPV() function will return zero 
/// with the given values.
/// 
/// There is no closed form for IRR. Evaluators may return an approximate 
/// solution using an iterative method, in which case the Guess parameter may 
/// be used to initialize the iteration. If the evaluator is unable to converge 
/// on a solution given a particular Guess, it may return an Error.
///
/// __See also__: "NPV", "RATE", 
#[inline]
pub fn irr<A: Sequence>(values: A) -> FnNumber1<A> {
    FnNumber1("IRR", values)
}

/// Compute the internal rate of return for a series of cash flows.
///
/// __Syntax__: 
/// ```ods
///     IRR( Values NumberSequence; Guess Number )
/// ```
///
/// __Constraints__:
/// None.
///
/// __Semantics__:
/// Compute the internal rate of return for a series of cash flows.
/// 
/// If provided, Guess is an estimate of the interest rate to start the 
/// iterative computation. If omitted, the value 0.1 (10%) is assumed.
/// 
/// The result of IRR is the rate at which the NPV() function will return zero 
/// with the given values.
/// 
/// There is no closed form for IRR. Evaluators may return an approximate 
/// solution using an iterative method, in which case the Guess parameter may 
/// be used to initialize the iteration. If the evaluator is unable to converge 
/// on a solution given a particular Guess, it may return an Error.
///
/// __See also__: "NPV", "RATE", 
#[inline]
pub fn irr_<A: Sequence, B: Number>(values: A, guess: B) -> FnNumber2<A, B> {
    FnNumber2("IRR", values, guess)
}

/// Compute the interest payment of an amortized loan for a given period.
///
/// __Syntax__: 
/// ```ods
///     ISPMT( Rate Number; Period Number; Nper Number; Pv Number )
/// ```
///
/// __Constraints__:
/// None.
///
/// __Semantics__:
/// Computes the interest payment of an amortized loan for a given period. The 
/// parameters are:
/// 
/// •Rate: the interest rate per period.
/// 
/// •Period: the period for which the interest is computed
/// 
/// •Nper: the total number of payment periods.
/// 
/// •Pv: the amount of the investment
///
/// __See also__: "PV", "FV", "NPER", "PMT", "RATE", 
#[inline]
pub fn ispmt<A: Number, B: Number, C: Number, D: Number>(rate: A, period: B, nper: C, pv: D) -> FnNumber4<A, B, C, D> {
    FnNumber4("ISPMT", rate, period, nper, pv)
}

/// Returns the modified Macaulay duration of a fixed interest security in 
/// years.
///
/// __Syntax__: 
/// ```ods
///     MDURATION( Settlement Date; Maturity Date; Coupon Number; Yield Number; Frequency Number )
/// ```
///
/// __Constraints__:
/// Yield ≥ 0, Coupon ≥ 0, Settlement ≤ Maturity; Frequency = 1, 2, 4
///
/// __Semantics__:
/// Computes the modified Macaulay duration, given:
/// 
/// •Settlement: the date of purchase of the security
/// 
/// •Maturity: the date when the security matures
/// 
/// •Coupon: the annual nominal rate of interest
/// 
/// •Yield: the annual yield of the security
/// 
/// •Frequency: number of interest payments per year
/// 
/// •B: Indicates the day-count convention to use in the calculation. 4.11.7
/// 
/// The modified duration is computed as follows:
///
/// __See also__: "DURATION", 
#[inline]
pub fn mduration<A: DateTime, B: DateTime, C: Number, D: Number>(settlement: A, maturity: B, coupon: C, yield_: D, frequency: Frequency) -> FnNumber5<A, B, C, D, Frequency> {
    FnNumber5("MDURATION", settlement, maturity, coupon, yield_, frequency)
}

/// Returns the modified Macaulay duration of a fixed interest security in 
/// years.
///
/// __Syntax__: 
/// ```ods
///     MDURATION( Settlement Date; Maturity Date; Coupon Number; Yield Number; Frequency Number; B Basis )
/// ```
///
/// __Constraints__:
/// Yield ≥ 0, Coupon ≥ 0, Settlement ≤ Maturity; Frequency = 1, 2, 4
///
/// __Semantics__:
/// Computes the modified Macaulay duration, given:
/// 
/// •Settlement: the date of purchase of the security
/// 
/// •Maturity: the date when the security matures
/// 
/// •Coupon: the annual nominal rate of interest
/// 
/// •Yield: the annual yield of the security
/// 
/// •Frequency: number of interest payments per year
/// 
/// •B: Indicates the day-count convention to use in the calculation. 4.11.7
/// 
/// The modified duration is computed as follows:
///
/// __See also__: "DURATION", 
#[inline]
pub fn mduration_<A: DateTime, B: DateTime, C: Number, D: Number>(settlement: A, maturity: B, coupon: C, yield_: D, frequency: Frequency, b: YearFracMethod) -> FnNumber6<A, B, C, D, Frequency, YearFracMethod> {
    FnNumber6("MDURATION", settlement, maturity, coupon, yield_, frequency, b)
}

/// Returns the modified internal rate of return (IRR) of a series of periodic 
/// investments.
///
/// __Syntax__: 
/// ```ods
///     MIRR( Values Array; Investment Number; ReinvestRate Number )
/// ```
///
/// __Constraints__:
/// Values shall contain at least one positive value and at least one negative 
/// value.
///
/// __Semantics__:
/// Values is a series of periodic income (positive values) and payments 
/// (negative values) at regular intervals (Text and Empty cells are ignored). 
/// Investment is the rate of interest of the payments (negative values); 
/// ReinvestRate is the rate of interest of the reinvestment (positive values).
/// 
/// Computes the modified internal rate of return, which is:
/// 
/// where N is the number of incomes and payments in Values (total).
///
/// __See also__: "IRR", "NPV", 
#[inline]
pub fn mirr<A: Array, B: Number, C: Number>(values: A, investment: B, reinvest_rate: C) -> FnNumber3<A, B, C> {
    FnNumber3("MIRR", values, investment, reinvest_rate)
}

/// Compute the annual nominal interest rate.
///
/// __Syntax__: 
/// ```ods
///     NOMINAL( EffectiveRate Number; CompoundingPeriods Integer )
/// ```
///
/// __Constraints__:
/// EffectiveRate > 0 , CompoundingPeriods > 0
///
/// __Semantics__:
/// Returns the annual nominal interest rate based on the effective rate and 
/// the number of compounding periods in one year. The parameters are:
/// 
/// •EffectiveRate: effective rate
/// 
/// •CompoundingPeriods: the compounding periods per year
/// 
/// Suppose that P is the present value, m is the compounding periods per year, 
/// the future value after one year is
/// 
/// The mapping between nominal rate and effective rate is
///
/// __See also__: "EFFECT", 
#[inline]
pub fn nominal<A: Number, B: Number>(effective_rate: A, compounding_periods: B) -> FnNumber2<A, B> {
    FnNumber2("NOMINAL", effective_rate, compounding_periods)
}

/// Compute the number of payment periods for an investment.
///
/// __Syntax__: 
/// ```ods
///     NPER( Rate Number; Payment Number; Pv Number )
/// ```
///
/// __Constraints__:
/// None.
///
/// __Semantics__:
/// Computes the number of payment periods for an investment. The parameters 
/// are:
/// 
/// •Rate: the constant interest rate.
/// 
/// •Payment: the payment made in each period.
/// 
/// •Pv: the present value of the investment.
/// 
/// •Fv: the future value; default is 0.
/// 
/// •PayType: the type of payment, defaults to 0. It is 0 if payments are due 
/// at the end of the period; 1 if they are due at the beginning of the period.
/// 
/// If Rate is 0, then NPER solves this equation:
/// 
/// ** Some equitation **
/// 
/// If Rate is non-zero, then NPER solves this equation:
/// 
/// ** Some equitation **
/// 
/// Evaluators claiming to support the “Medium” or “Large” set shall 
/// support negative rates; evaluators only claiming to support the “Small” 
/// set need not.
///
/// __See also__: "FV", "RATE", "PMT", "PV", 
#[inline]
pub fn nper<A: Number, B: Number, C: Number>(rate: A, payment: B, pv: C) -> FnNumber3<A, B, C> {
    FnNumber3("NPER", rate, payment, pv)
}

/// Compute the number of payment periods for an investment.
///
/// __Syntax__: 
/// ```ods
///     NPER( Rate Number; Payment Number; Pv Number; Fv Number )
/// ```
///
/// __Constraints__:
/// None.
///
/// __Semantics__:
/// Computes the number of payment periods for an investment. The parameters 
/// are:
/// 
/// •Rate: the constant interest rate.
/// 
/// •Payment: the payment made in each period.
/// 
/// •Pv: the present value of the investment.
/// 
/// •Fv: the future value; default is 0.
/// 
/// •PayType: the type of payment, defaults to 0. It is 0 if payments are due 
/// at the end of the period; 1 if they are due at the beginning of the period.
/// 
/// If Rate is 0, then NPER solves this equation:
/// 
/// ** Some equitation **
/// 
/// If Rate is non-zero, then NPER solves this equation:
/// 
/// ** Some equitation **
/// 
/// Evaluators claiming to support the “Medium” or “Large” set shall 
/// support negative rates; evaluators only claiming to support the “Small” 
/// set need not.
///
/// __See also__: "FV", "RATE", "PMT", "PV", 
#[inline]
pub fn nper_<A: Number, B: Number, C: Number, D: Number>(rate: A, payment: B, pv: C, fv: D) -> FnNumber4<A, B, C, D> {
    FnNumber4("NPER", rate, payment, pv, fv)
}

/// Compute the number of payment periods for an investment.
///
/// __Syntax__: 
/// ```ods
///     NPER( Rate Number; Payment Number; Pv Number; Fv Number; PayType Number )
/// ```
///
/// __Constraints__:
/// None.
///
/// __Semantics__:
/// Computes the number of payment periods for an investment. The parameters 
/// are:
/// 
/// •Rate: the constant interest rate.
/// 
/// •Payment: the payment made in each period.
/// 
/// •Pv: the present value of the investment.
/// 
/// •Fv: the future value; default is 0.
/// 
/// •PayType: the type of payment, defaults to 0. It is 0 if payments are due 
/// at the end of the period; 1 if they are due at the beginning of the period.
/// 
/// If Rate is 0, then NPER solves this equation:
/// 
/// ** Some equitation **
/// 
/// If Rate is non-zero, then NPER solves this equation:
/// 
/// ** Some equitation **
/// 
/// Evaluators claiming to support the “Medium” or “Large” set shall 
/// support negative rates; evaluators only claiming to support the “Small” 
/// set need not.
///
/// __See also__: "FV", "RATE", "PMT", "PV", 
#[inline]
pub fn nper__<A: Number, B: Number, C: Number, D: Number>(rate: A, payment: B, pv: C, fv: D, pay_type: PayType) -> FnNumber5<A, B, C, D, PayType> {
    FnNumber5("NPER", rate, payment, pv, fv, pay_type)
}

/// Compute the net present value (NPV) for a series of periodic cash flows.
///
/// __Syntax__: 
/// ```ods
///     NPV( Rate Number{; Values NumberSequenceList}+ )
/// ```
///
/// __Constraints__:
/// None.
///
/// __Semantics__:
/// Computes the net present value for a series of periodic cash flows with the 
/// discount rate Rate. Values should be positive if they are received as 
/// income, and negative if the amounts are paid as outgo. Because the result 
/// is affected by the order of values, evaluators shall evaluate arguments in 
/// the order given and range reference and array arguments row-wise starting 
/// from top left.
/// 
/// If N is the number of values in Values, the formula for NPV is:
///
/// __See also__: "FV", "IRR", "NPER", "PMT", "PV", "XNPV", 
#[inline]
pub fn npv<A: Number, B: Sequence>(rate: A, values: B) -> FnNumber2<A, B> {
    FnNumber2("NPV", rate, values)
}

/// Compute the value of a security per 100 currency units of face value. The 
/// security has an irregular first interest date.
///
/// __Syntax__: 
/// ```ods
///     ODDFPRICE( Settlement DateParam; Maturity DateParam; Issue DateParam; First DateParam; Rate Number; Yield Number; Redemption Number; Frequency Number )
/// ```
///
/// __Constraints__:
/// Rate, Yield, and Redemption should be greater than 0.
///
/// __Semantics__:
/// The parameters are
/// 
/// •Settlement: the settlement/purchase date of the security
/// 
/// •Maturity: the maturity/expiry date of the security
/// 
/// •Issue: the issue date of the security
/// 
/// •First: the first coupon date of the security
/// 
/// •Rate: the interest rate of the security
/// 
/// •Yield: the annual yield of the security
/// 
/// •Redemption: the redemption value per 100 currency units face value
/// 
/// •Frequency: the number of interest payments per year. 1 = annual; 2 = 
/// semiannual; 4 = quarterly.
/// 
/// •B: indicates the day-count convention to use in the calculation. 4.11.7
///
/// __See also__: "ODDLPRICE", "ODDFYIELD", 
#[inline]
pub fn oddfprice<A: DateTime, B: DateTime, C: DateTime, D: DateTime, E: Number, F: Number, G: Number>(settlement: A, maturity: B, issue: C, first: D, rate: E, yield_: F, redemption: G, frequency: Frequency) -> FnNumber8<A, B, C, D, E, F, G, Frequency> {
    FnNumber8("ODDFPRICE", settlement, maturity, issue, first, rate, yield_, redemption, frequency)
}

/// Compute the value of a security per 100 currency units of face value. The 
/// security has an irregular first interest date.
///
/// __Syntax__: 
/// ```ods
///     ODDFPRICE( Settlement DateParam; Maturity DateParam; Issue DateParam; First DateParam; Rate Number; Yield Number; Redemption Number; Frequency Number; B Basis )
/// ```
///
/// __Constraints__:
/// Rate, Yield, and Redemption should be greater than 0.
///
/// __Semantics__:
/// The parameters are
/// 
/// •Settlement: the settlement/purchase date of the security
/// 
/// •Maturity: the maturity/expiry date of the security
/// 
/// •Issue: the issue date of the security
/// 
/// •First: the first coupon date of the security
/// 
/// •Rate: the interest rate of the security
/// 
/// •Yield: the annual yield of the security
/// 
/// •Redemption: the redemption value per 100 currency units face value
/// 
/// •Frequency: the number of interest payments per year. 1 = annual; 2 = 
/// semiannual; 4 = quarterly.
/// 
/// •B: indicates the day-count convention to use in the calculation. 4.11.7
///
/// __See also__: "ODDLPRICE", "ODDFYIELD", 
#[inline]
pub fn oddfprice_<A: DateTime, B: DateTime, C: DateTime, D: DateTime, E: Number, F: Number, G: Number>(settlement: A, maturity: B, issue: C, first: D, rate: E, yield_: F, redemption: G, frequency: Frequency, b: YearFracMethod) -> FnNumber9<A, B, C, D, E, F, G, Frequency, YearFracMethod> {
    FnNumber9("ODDFPRICE", settlement, maturity, issue, first, rate, yield_, redemption, frequency, b)
}

/// Compute the yield of a security per 100 currency units of face value. The 
/// security has an irregular first interest date.
///
/// __Syntax__: 
/// ```ods
///     ODDFYIELD( Settlement DateParam; Maturity DateParam; Issue DateParam; First DateParam; Rate Number; Price Number; Redemption Number; Frequency Number )
/// ```
///
/// __Constraints__:
/// Rate, Price, and Redemption should be greater than 0. Maturity > First > 
/// Settlement > Issue.
///
/// __Semantics__:
/// The parameters are
/// 
/// •Settlement: the settlement/purchase date of the security
/// 
/// •Maturity: the maturity/expiry date of the security
/// 
/// •Issue: the issue date of the security
/// 
/// •First: the first coupon date of the security
/// 
/// •Rate: the interest rate of the security
/// 
/// •Price: the price of the security
/// 
/// •Redemption: the redemption value per 100 currency units face value
/// 
/// •Frequency: the number of interest payments per year. 1 = annual; 2 = 
/// semiannual; 4 = quarterly.
/// 
/// •B: indicates the day-count convention to use in the calculation. 4.11.7
///
/// __See also__: "ODDLYIELD", "ODDFPRICE", 
#[inline]
pub fn oddfyield<A: DateTime, B: DateTime, C: DateTime, D: DateTime, E: Number, F: Number, G: Number>(settlement: A, maturity: B, issue: C, first: D, rate: E, price: F, redemption: G, frequency: Frequency) -> FnNumber8<A, B, C, D, E, F, G, Frequency> {
    FnNumber8("ODDFYIELD", settlement, maturity, issue, first, rate, price, redemption, frequency)
}

/// Compute the yield of a security per 100 currency units of face value. The 
/// security has an irregular first interest date.
///
/// __Syntax__: 
/// ```ods
///     ODDFYIELD( Settlement DateParam; Maturity DateParam; Issue DateParam; First DateParam; Rate Number; Price Number; Redemption Number; Frequency Number; B Basis )
/// ```
///
/// __Constraints__:
/// Rate, Price, and Redemption should be greater than 0. Maturity > First > 
/// Settlement > Issue.
///
/// __Semantics__:
/// The parameters are
/// 
/// •Settlement: the settlement/purchase date of the security
/// 
/// •Maturity: the maturity/expiry date of the security
/// 
/// •Issue: the issue date of the security
/// 
/// •First: the first coupon date of the security
/// 
/// •Rate: the interest rate of the security
/// 
/// •Price: the price of the security
/// 
/// •Redemption: the redemption value per 100 currency units face value
/// 
/// •Frequency: the number of interest payments per year. 1 = annual; 2 = 
/// semiannual; 4 = quarterly.
/// 
/// •B: indicates the day-count convention to use in the calculation. 4.11.7
///
/// __See also__: "ODDLYIELD", "ODDFPRICE", 
#[inline]
pub fn oddfyield_<A: DateTime, B: DateTime, C: DateTime, D: DateTime, E: Number, F: Number, G: Number>(settlement: A, maturity: B, issue: C, first: D, rate: E, price: F, redemption: G, frequency: Frequency, b: YearFracMethod) -> FnNumber9<A, B, C, D, E, F, G, Frequency, YearFracMethod> {
    FnNumber9("ODDFYIELD", settlement, maturity, issue, first, rate, price, redemption, frequency, b)
}

/// Compute the value of a security per 100 currency units of face value. The 
/// security has an irregular last interest date.
///
/// __Syntax__: 
/// ```ods
///     ODDLPRICE( Settlement DateParam; Maturity DateParam; Last DateParam; Rate Number; AnnualYield Number; Redemption Number; Frequency Number )
/// ```
///
/// __Constraints__:
/// Rate, AnnualYield, and Redemption should be greater than 0. The Maturity 
/// date should be greater than the Settlement date, and the Settlement should 
/// be greater than the last interest date.
///
/// __Semantics__:
/// The parameters are
/// 
/// •Settlement: the settlement/purchase date of the security
/// 
/// •Maturity: the maturity/expiry date of the security
/// 
/// •Last: the last interest date of the security
/// 
/// •Rate: the interest rate of the security
/// 
/// •AnnualYield: the annual yield of the security
/// 
/// •Redemption: the redemption value per 100 currency units face value
/// 
/// •Frequency: the number of interest payments per year. 1 = annual; 2 = 
/// semiannual; 4 = quarterly
/// 
/// •B: indicates the day-count convention to use in the calculation. 4.11.7
///
/// __See also__: "ODDFPRICE", 
#[inline]
pub fn oddlprice<A: DateTime, B: DateTime, C: DateTime, D: Number, E: Number, F: Number>(settlement: A, maturity: B, last: C, rate: D, annual_yield: E, redemption: F, frequency: Frequency) -> FnNumber7<A, B, C, D, E, F, Frequency> {
    FnNumber7("ODDLPRICE", settlement, maturity, last, rate, annual_yield, redemption, frequency)
}

/// Compute the value of a security per 100 currency units of face value. The 
/// security has an irregular last interest date.
///
/// __Syntax__: 
/// ```ods
///     ODDLPRICE( Settlement DateParam; Maturity DateParam; Last DateParam; Rate Number; AnnualYield Number; Redemption Number; Frequency Number; B Basis )
/// ```
///
/// __Constraints__:
/// Rate, AnnualYield, and Redemption should be greater than 0. The Maturity 
/// date should be greater than the Settlement date, and the Settlement should 
/// be greater than the last interest date.
///
/// __Semantics__:
/// The parameters are
/// 
/// •Settlement: the settlement/purchase date of the security
/// 
/// •Maturity: the maturity/expiry date of the security
/// 
/// •Last: the last interest date of the security
/// 
/// •Rate: the interest rate of the security
/// 
/// •AnnualYield: the annual yield of the security
/// 
/// •Redemption: the redemption value per 100 currency units face value
/// 
/// •Frequency: the number of interest payments per year. 1 = annual; 2 = 
/// semiannual; 4 = quarterly
/// 
/// •B: indicates the day-count convention to use in the calculation. 4.11.7
///
/// __See also__: "ODDFPRICE", 
#[inline]
pub fn oddlprice_<A: DateTime, B: DateTime, C: DateTime, D: Number, E: Number, F: Number>(settlement: A, maturity: B, last: C, rate: D, annual_yield: E, redemption: F, frequency: Frequency, b: YearFracMethod) -> FnNumber8<A, B, C, D, E, F, Frequency, YearFracMethod> {
    FnNumber8("ODDLPRICE", settlement, maturity, last, rate, annual_yield, redemption, frequency, b)
}

/// Compute the yield of a security which has an irregular last interest date.
///
/// __Syntax__: 
/// ```ods
///     ODDLYIELD( Settlement DateParam; Maturity DateParam; Last DateParam; Rate Number; Price Number; Redemption Number; Frequency Number )
/// ```
///
/// __Constraints__:
/// Rate, Price, and Redemption should be greater than 0.
///
/// __Semantics__:
/// The parameters are
/// 
/// •Settlement: the settlement/purchase date of the security
/// 
/// •Maturity: the maturity/expiry date of the security
/// 
/// •Last: the last interest date of the security
/// 
/// •Rate: the interest rate of the security
/// 
/// •Price: the price of the security
/// 
/// •Redemption: the redemption value per 100 currency units face value
/// 
/// •Frequency: the number of interest payments per year. 1 = annual; 2 = 
/// semiannual; 4 = quarterly.
/// 
/// •B: indicates the day-count convention to use in the calculation. 4.11.7
///
/// __See also__: "ODDLPRICE", "ODDFYIELD", 
#[inline]
pub fn oddlyield<A: DateTime, B: DateTime, C: DateTime, D: Number, E: Number, F: Number>(settlement: A, maturity: B, last: C, rate: D, price: E, redemption: F, frequency: Frequency) -> FnNumber7<A, B, C, D, E, F, Frequency> {
    FnNumber7("ODDLYIELD", settlement, maturity, last, rate, price, redemption, frequency)
}

/// Compute the yield of a security which has an irregular last interest date.
///
/// __Syntax__: 
/// ```ods
///     ODDLYIELD( Settlement DateParam; Maturity DateParam; Last DateParam; Rate Number; Price Number; Redemption Number; Frequency Number; B Basis )
/// ```
///
/// __Constraints__:
/// Rate, Price, and Redemption should be greater than 0.
///
/// __Semantics__:
/// The parameters are
/// 
/// •Settlement: the settlement/purchase date of the security
/// 
/// •Maturity: the maturity/expiry date of the security
/// 
/// •Last: the last interest date of the security
/// 
/// •Rate: the interest rate of the security
/// 
/// •Price: the price of the security
/// 
/// •Redemption: the redemption value per 100 currency units face value
/// 
/// •Frequency: the number of interest payments per year. 1 = annual; 2 = 
/// semiannual; 4 = quarterly.
/// 
/// •B: indicates the day-count convention to use in the calculation. 4.11.7
///
/// __See also__: "ODDLPRICE", "ODDFYIELD", 
#[inline]
pub fn oddlyield_<A: DateTime, B: DateTime, C: DateTime, D: Number, E: Number, F: Number>(settlement: A, maturity: B, last: C, rate: D, price: E, redemption: F, frequency: Frequency, b: YearFracMethod) -> FnNumber8<A, B, C, D, E, F, Frequency, YearFracMethod> {
    FnNumber8("ODDLYIELD", settlement, maturity, last, rate, price, redemption, frequency, b)
}

/// Returns the number of periods required by an investment to realize a 
/// specified value.
///
/// __Syntax__: 
/// ```ods
///     PDURATION( Rate Number; CurrentValue Number; SpecifiedValue Number )
/// ```
///
/// __Constraints__:
/// Rate > 0; CurrentValue > 0; SpecifiedValue > 0
///
/// __Semantics__:
/// Calculates the number of periods for attaining a certain value 
/// SpecifiedValue, starting from CurrentValue and using the interest rate 
/// Rate.
/// 
/// •Rate: The interest rate per period.
/// 
/// •CurrentValue: The current value of the investment.
/// 
/// •SpecifiedValue: The value, that should be reached.
///
/// __See also__: "DURATION", 
#[inline]
pub fn pduration<A: Number, B: Number, C: Number>(rate: A, current_value: B, specified_value: C) -> FnNumber3<A, B, C> {
    FnNumber3("PDURATION", rate, current_value, specified_value)
}

/// Compute the payment made each period for an investment.
///
/// __Syntax__: 
/// ```ods
///     PMT( Rate Number; Nper Integer; Pv Number )
/// ```
///
/// __Constraints__:
/// Nper > 0
///
/// __Semantics__:
/// Computes the payment made each period for an investment. The parameters 
/// are:
/// 
/// •Rate: the interest rate per period.
/// 
/// •Nper: the total number of payment periods.
/// 
/// •Pv: the present value of the investment.
/// 
/// •Fv: the future value of the investment; default is 0.
/// 
/// •PayType: the type of payment, defaults to 0. It is 0 if payments are due 
/// at the end of the period; 1 if they are due at the beginning of the period. 
/// With PayType = 1 the first payment is made on the same day the loan is 
/// taken out.
/// 
/// If Rate is 0, the following equation is solved:
/// 
/// If Rate is nonzero, then PMT solves this equation:
///
/// __See also__: "FV", "NPER", "PV", "RATE", 
#[inline]
pub fn pmt<A: Number, B: Number, C: Number>(rate: A, nper: B, pv: C) -> FnNumber3<A, B, C> {
    FnNumber3("PMT", rate, nper, pv)
}

/// Compute the payment made each period for an investment.
///
/// __Syntax__: 
/// ```ods
///     PMT( Rate Number; Nper Integer; Pv Number; Fv Number )
/// ```
///
/// __Constraints__:
/// Nper > 0
///
/// __Semantics__:
/// Computes the payment made each period for an investment. The parameters 
/// are:
/// 
/// •Rate: the interest rate per period.
/// 
/// •Nper: the total number of payment periods.
/// 
/// •Pv: the present value of the investment.
/// 
/// •Fv: the future value of the investment; default is 0.
/// 
/// •PayType: the type of payment, defaults to 0. It is 0 if payments are due 
/// at the end of the period; 1 if they are due at the beginning of the period. 
/// With PayType = 1 the first payment is made on the same day the loan is 
/// taken out.
/// 
/// If Rate is 0, the following equation is solved:
/// 
/// If Rate is nonzero, then PMT solves this equation:
///
/// __See also__: "FV", "NPER", "PV", "RATE", 
#[inline]
pub fn pmt_<A: Number, B: Number, C: Number, D: Number>(rate: A, nper: B, pv: C, fv: D) -> FnNumber4<A, B, C, D> {
    FnNumber4("PMT", rate, nper, pv, fv)
}

/// Compute the payment made each period for an investment.
///
/// __Syntax__: 
/// ```ods
///     PMT( Rate Number; Nper Integer; Pv Number; Fv Number; PayType Number )
/// ```
///
/// __Constraints__:
/// Nper > 0
///
/// __Semantics__:
/// Computes the payment made each period for an investment. The parameters 
/// are:
/// 
/// •Rate: the interest rate per period.
/// 
/// •Nper: the total number of payment periods.
/// 
/// •Pv: the present value of the investment.
/// 
/// •Fv: the future value of the investment; default is 0.
/// 
/// •PayType: the type of payment, defaults to 0. It is 0 if payments are due 
/// at the end of the period; 1 if they are due at the beginning of the period. 
/// With PayType = 1 the first payment is made on the same day the loan is 
/// taken out.
/// 
/// If Rate is 0, the following equation is solved:
/// 
/// If Rate is nonzero, then PMT solves this equation:
///
/// __See also__: "FV", "NPER", "PV", "RATE", 
#[inline]
pub fn pmt__<A: Number, B: Number, C: Number, D: Number, E: Number>(rate: A, nper: B, pv: C, fv: D, pay_type: E) -> FnNumber5<A, B, C, D, E> {
    FnNumber5("PMT", rate, nper, pv, fv, pay_type)
}

/// Calculate the payment for a given period on the principal for an investment 
/// at a given interest rate and constant payments.
///
/// __Syntax__: 
/// ```ods
///     PPMT( Rate Number; Period Integer; Nper Integer; Present Number )
/// ```
///
/// __Constraints__:
/// Rate and Present should be greater than 0. 0 < Period < Nper.
///
/// __Semantics__:
/// The parameters are:
/// 
/// •Rate: the interest rate.
/// 
/// •Period: the given period that the payment returned is for.
/// 
/// •Nper: the total number of periods.
/// 
/// •Present: the present value.
/// 
/// •Future: optional, the future value specified after Nper periods. The 
/// default value is 0.
/// 
/// •Type: optional, 0 or 1, respectively for payment at the end or at the 
/// beginning of a period. The default value is 0.
///
/// __See also__: "PMT", 
#[inline]
pub fn ppmt<A: Number, B: Number, C: Number, D: Number>(rate: A, period: B, nper: C, present: D) -> FnNumber4<A, B, C, D> {
    FnNumber4("PPMT", rate, period, nper, present)
}

/// Calculate the payment for a given period on the principal for an investment 
/// at a given interest rate and constant payments.
///
/// __Syntax__: 
/// ```ods
///     PPMT( Rate Number; Period Integer; Nper Integer; Present Number; Future Number )
/// ```
///
/// __Constraints__:
/// Rate and Present should be greater than 0. 0 < Period < Nper.
///
/// __Semantics__:
/// The parameters are:
/// 
/// •Rate: the interest rate.
/// 
/// •Period: the given period that the payment returned is for.
/// 
/// •Nper: the total number of periods.
/// 
/// •Present: the present value.
/// 
/// •Future: optional, the future value specified after Nper periods. The 
/// default value is 0.
/// 
/// •Type: optional, 0 or 1, respectively for payment at the end or at the 
/// beginning of a period. The default value is 0.
///
/// __See also__: "PMT", 
#[inline]
pub fn ppmt_<A: Number, B: Number, C: Number, D: Number, E: Number>(rate: A, period: B, nper: C, present: D, future: E) -> FnNumber5<A, B, C, D, E> {
    FnNumber5("PPMT", rate, period, nper, present, future)
}

/// Calculate the payment for a given period on the principal for an investment 
/// at a given interest rate and constant payments.
///
/// __Syntax__: 
/// ```ods
///     PPMT( Rate Number; Period Integer; Nper Integer; Present Number; Future Number; Type Number )
/// ```
///
/// __Constraints__:
/// Rate and Present should be greater than 0. 0 < Period < Nper.
///
/// __Semantics__:
/// The parameters are:
/// 
/// •Rate: the interest rate.
/// 
/// •Period: the given period that the payment returned is for.
/// 
/// •Nper: the total number of periods.
/// 
/// •Present: the present value.
/// 
/// •Future: optional, the future value specified after Nper periods. The 
/// default value is 0.
/// 
/// •Type: optional, 0 or 1, respectively for payment at the end or at the 
/// beginning of a period. The default value is 0.
///
/// __See also__: "PMT", 
#[inline]
pub fn ppmt__<A: Number, B: Number, C: Number, D: Number, E: Number, F: Number>(rate: A, period: B, nper: C, present: D, future: E, type_: F) -> FnNumber6<A, B, C, D, E, F> {
    FnNumber6("PPMT", rate, period, nper, present, future, type_)
}

/// Calculates a quoted price for an interest paying security, per 100 currency 
/// units of face value.
///
/// __Syntax__: 
/// ```ods
///     PRICE( Settlement DateParam; Maturity DateParam; Rate Number; AnnualYield Number; Redemption Number; Frequency Number )
/// ```
///
/// __Constraints__:
/// Rate, AnnualYield, and Redemption should be greater than 0; Frequency = 1, 
/// 2 or 4.
///
/// __Semantics__:
/// If A is the number of days from the Settlement date to next coupon date, B 
/// is the number of days of the coupon period that the Settlement is in, C is 
/// the number of coupons between Settlement date and Redemption date, D is the 
/// number of days from beginning of coupon period to Settlement date, then 
/// PRICE is calculated as
/// 
/// The parameters are:
/// 
/// •Settlement: the settlement/purchase date of the security.
/// 
/// •Maturity: the maturity/expiry date of the security.
/// 
/// •Rate: the interest rate of the security.
/// 
/// •AnnualYield: a measure of the annual yield of a security (compounded at 
/// each interest payment).
/// 
/// •Redemption: the redemption value per 100 currency units face value.
/// 
/// •Frequency: the number of interest payments per year. 1 = annual; 2 = 
/// semiannual; 4 = quarterly.
/// 
/// •Bas: indicates the day-count convention to use in the calculation. 
/// 4.11.7
///
/// __See also__: "PRICEDISC", "PRICEMAT", 
#[inline]
pub fn price<A: DateTime, B: DateTime, C: Number, D: Number, E: Number>(settlement: A, maturity: B, rate: C, annual_yield: D, redemption: E, frequency: Frequency) -> FnNumber6<A, B, C, D, E, Frequency> {
    FnNumber6("PRICE", settlement, maturity, rate, annual_yield, redemption, frequency)
}

/// Calculates a quoted price for an interest paying security, per 100 currency 
/// units of face value.
///
/// __Syntax__: 
/// ```ods
///     PRICE( Settlement DateParam; Maturity DateParam; Rate Number; AnnualYield Number; Redemption Number; Frequency Number; Bas Basis )
/// ```
///
/// __Constraints__:
/// Rate, AnnualYield, and Redemption should be greater than 0; Frequency = 1, 
/// 2 or 4.
///
/// __Semantics__:
/// If A is the number of days from the Settlement date to next coupon date, B 
/// is the number of days of the coupon period that the Settlement is in, C is 
/// the number of coupons between Settlement date and Redemption date, D is the 
/// number of days from beginning of coupon period to Settlement date, then 
/// PRICE is calculated as
/// 
/// The parameters are:
/// 
/// •Settlement: the settlement/purchase date of the security.
/// 
/// •Maturity: the maturity/expiry date of the security.
/// 
/// •Rate: the interest rate of the security.
/// 
/// •AnnualYield: a measure of the annual yield of a security (compounded at 
/// each interest payment).
/// 
/// •Redemption: the redemption value per 100 currency units face value.
/// 
/// •Frequency: the number of interest payments per year. 1 = annual; 2 = 
/// semiannual; 4 = quarterly.
/// 
/// •Bas: indicates the day-count convention to use in the calculation. 
/// 4.11.7
///
/// __See also__: "PRICEDISC", "PRICEMAT", 
#[inline]
pub fn price_<A: DateTime, B: DateTime, C: Number, D: Number, E: Number>(settlement: A, maturity: B, rate: C, annual_yield: D, redemption: E, frequency: Frequency, bas: YearFracMethod) -> FnNumber7<A, B, C, D, E, Frequency, YearFracMethod> {
    FnNumber7("PRICE", settlement, maturity, rate, annual_yield, redemption, frequency, bas)
}

/// Calculate the price of a security with a discount per 100 currency units of 
/// face value.
///
/// __Syntax__: 
/// ```ods
///     PRICEDISC( Settlement DateParam; Maturity DateParam; Discount Number; Redemption Number )
/// ```
///
/// __Constraints__:
/// Discount and Redemption should be greater than 0.
///
/// __Semantics__:
/// The parameters are:
/// 
/// •Settlement: the settlement/purchase date of the security.
/// 
/// •Maturity: the maturity/expiry date of the security.
/// 
/// •Discount: the discount rate of the security.
/// 
/// •Redemption: the redemption value per 100 currency units face value.
/// 
/// •B: indicates the day-count convention to use in the calculation. 4.11.7
///
/// __See also__: "PRICE", "PRICEMAT", "YIELDDISC", 
#[inline]
pub fn pricedisc<A: DateTime, B: DateTime, C: Number, D: Number>(settlement: A, maturity: B, discount: C, redemption: D) -> FnNumber4<A, B, C, D> {
    FnNumber4("PRICEDISC", settlement, maturity, discount, redemption)
}

/// Calculate the price of a security with a discount per 100 currency units of 
/// face value.
///
/// __Syntax__: 
/// ```ods
///     PRICEDISC( Settlement DateParam; Maturity DateParam; Discount Number; Redemption Number; B Basis )
/// ```
///
/// __Constraints__:
/// Discount and Redemption should be greater than 0.
///
/// __Semantics__:
/// The parameters are:
/// 
/// •Settlement: the settlement/purchase date of the security.
/// 
/// •Maturity: the maturity/expiry date of the security.
/// 
/// •Discount: the discount rate of the security.
/// 
/// •Redemption: the redemption value per 100 currency units face value.
/// 
/// •B: indicates the day-count convention to use in the calculation. 4.11.7
///
/// __See also__: "PRICE", "PRICEMAT", "YIELDDISC", 
#[inline]
pub fn pricedisc_<A: DateTime, B: DateTime, C: Number, D: Number>(settlement: A, maturity: B, discount: C, redemption: D, b: YearFracMethod) -> FnNumber5<A, B, C, D, YearFracMethod> {
    FnNumber5("PRICEDISC", settlement, maturity, discount, redemption, b)
}

/// Calculate the price per 100 currency units of face value of the security 
/// that pays interest on the maturity date.
///
/// __Syntax__: 
/// ```ods
///     PRICEMAT( Settlement DateParam; Maturity DateParam; Issue DateParam; Rate Number; AnnualYield Number )
/// ```
///
/// __Constraints__:
/// Settlement < Maturity, Rate ≥ 0, AnnualYield ≥ 0
///
/// __Semantics__:
/// The parameters are:
/// 
/// •Settlement: the settlement/purchase date of the security.
/// 
/// •Maturity: the maturity/expiry date of the security.
/// 
/// •Issue: the issue date of the security.
/// 
/// •Rate: the interest rate of the security.
/// 
/// •AnnualYield: the annual yield of the security.
/// 
/// •B: indicates the day-count convention to use in the calculation. 4.11.7
/// 
/// If both, Rate and AnnualYield, are 0, the return value is 100.
///
/// __See also__: "PRICEDISC", "PRICEMAT", 
#[inline]
pub fn pricemat<A: DateTime, B: DateTime, C: DateTime, D: Number, E: Number>(settlement: A, maturity: B, issue: C, rate: D, annual_yield: E) -> FnNumber5<A, B, C, D, E> {
    FnNumber5("PRICEMAT", settlement, maturity, issue, rate, annual_yield)
}

/// Calculate the price per 100 currency units of face value of the security 
/// that pays interest on the maturity date.
///
/// __Syntax__: 
/// ```ods
///     PRICEMAT( Settlement DateParam; Maturity DateParam; Issue DateParam; Rate Number; AnnualYield Number; B Basis )
/// ```
///
/// __Constraints__:
/// Settlement < Maturity, Rate ≥ 0, AnnualYield ≥ 0
///
/// __Semantics__:
/// The parameters are:
/// 
/// •Settlement: the settlement/purchase date of the security.
/// 
/// •Maturity: the maturity/expiry date of the security.
/// 
/// •Issue: the issue date of the security.
/// 
/// •Rate: the interest rate of the security.
/// 
/// •AnnualYield: the annual yield of the security.
/// 
/// •B: indicates the day-count convention to use in the calculation. 4.11.7
/// 
/// If both, Rate and AnnualYield, are 0, the return value is 100.
///
/// __See also__: "PRICEDISC", "PRICEMAT", 
#[inline]
pub fn pricemat_<A: DateTime, B: DateTime, C: DateTime, D: Number, E: Number>(settlement: A, maturity: B, issue: C, rate: D, annual_yield: E, b: YearFracMethod) -> FnNumber6<A, B, C, D, E, YearFracMethod> {
    FnNumber6("PRICEMAT", settlement, maturity, issue, rate, annual_yield, b)
}

/// Compute the present value (PV) of an investment.
///
/// __Syntax__: 
/// ```ods
///     PV( Rate Number; Nper Number; Payment Number )
/// ```
///
/// __Constraints__:
/// None.
///
/// __Semantics__:
/// Computes the present value of an investment. The parameters are:
/// 
/// •Rate: the interest rate per period.
/// 
/// •Nper: the total number of payment periods.
/// 
/// •Payment: the payment made in each period.
/// 
/// •Fv: the future value; default is 0.
/// 
/// •PayType: the type of payment, defaults to 0. It is 0 if payments are due 
/// at the end of the period; 1 if they are due at the beginning of the period.
/// 
/// If Rate is 0, then:
/// 
/// If Rate is nonzero, then PV solves this equation:
///
/// __See also__: "FV", "NPER", "PMT", "RATE", 
#[inline]
pub fn pv<A: Number, B: Number, C: Number>(rate: A, nper: B, payment: C) -> FnNumber3<A, B, C> {
    FnNumber3("PV", rate, nper, payment)
}

/// Compute the present value (PV) of an investment.
///
/// __Syntax__: 
/// ```ods
///     PV( Rate Number; Nper Number; Payment Number; Fv Number )
/// ```
///
/// __Constraints__:
/// None.
///
/// __Semantics__:
/// Computes the present value of an investment. The parameters are:
/// 
/// •Rate: the interest rate per period.
/// 
/// •Nper: the total number of payment periods.
/// 
/// •Payment: the payment made in each period.
/// 
/// •Fv: the future value; default is 0.
/// 
/// •PayType: the type of payment, defaults to 0. It is 0 if payments are due 
/// at the end of the period; 1 if they are due at the beginning of the period.
/// 
/// If Rate is 0, then:
/// 
/// If Rate is nonzero, then PV solves this equation:
///
/// __See also__: "FV", "NPER", "PMT", "RATE", 
#[inline]
pub fn pv_<A: Number, B: Number, C: Number, D: Number>(rate: A, nper: B, payment: C, fv: D) -> FnNumber4<A, B, C, D> {
    FnNumber4("PV", rate, nper, payment, fv)
}

/// Compute the present value (PV) of an investment.
///
/// __Syntax__: 
/// ```ods
///     PV( Rate Number; Nper Number; Payment Number; Fv Number; PayType Number )
/// ```
///
/// __Constraints__:
/// None.
///
/// __Semantics__:
/// Computes the present value of an investment. The parameters are:
/// 
/// •Rate: the interest rate per period.
/// 
/// •Nper: the total number of payment periods.
/// 
/// •Payment: the payment made in each period.
/// 
/// •Fv: the future value; default is 0.
/// 
/// •PayType: the type of payment, defaults to 0. It is 0 if payments are due 
/// at the end of the period; 1 if they are due at the beginning of the period.
/// 
/// If Rate is 0, then:
/// 
/// If Rate is nonzero, then PV solves this equation:
///
/// __See also__: "FV", "NPER", "PMT", "RATE", 
#[inline]
pub fn pv__<A: Number, B: Number, C: Number, D: Number, E: Number>(rate: A, nper: B, payment: C, fv: D, pay_type: E) -> FnNumber5<A, B, C, D, E> {
    FnNumber5("PV", rate, nper, payment, fv, pay_type)
}

/// Compute the interest rate per period of an investment.
///
/// __Syntax__: 
/// ```ods
///     RATE( Nper Number; Payment Number; Pv Number )
/// ```
///
/// __Constraints__:
/// If Nper is 0 or less than 0, the result is an Error.
///
/// __Semantics__:
/// Computes the interest rate of an investment. The parameters are:
/// 
/// •Nper: the total number of payment periods.
/// 
/// •Payment: the payment made in each period.
/// 
/// •Pv: the present value of the investment.
/// 
/// •Fv: the future value; default is 0.
/// 
/// •PayType: the type of payment, defaults to 0. It is 0 if payments are due 
/// at the end of the period; 1 if they are due at the beginning of the period.
/// 
/// •Guess: An estimate of the interest rate to start the iterative 
/// computation. If omitted, 0.1 (10%) is assumed.
/// 
/// RATE solves this equation:
///
/// __See also__: "FV", "NPER", "PMT", "PV", 
#[inline]
pub fn rate<A: Number, B: Number, C: Number>(nper: A, payment: B, pv: C) -> FnNumber3<A, B, C> {
    FnNumber3("RATE", nper, payment, pv)
}

/// Compute the interest rate per period of an investment.
///
/// __Syntax__: 
/// ```ods
///     RATE( Nper Number; Payment Number; Pv Number; Fv Number )
/// ```
///
/// __Constraints__:
/// If Nper is 0 or less than 0, the result is an Error.
///
/// __Semantics__:
/// Computes the interest rate of an investment. The parameters are:
/// 
/// •Nper: the total number of payment periods.
/// 
/// •Payment: the payment made in each period.
/// 
/// •Pv: the present value of the investment.
/// 
/// •Fv: the future value; default is 0.
/// 
/// •PayType: the type of payment, defaults to 0. It is 0 if payments are due 
/// at the end of the period; 1 if they are due at the beginning of the period.
/// 
/// •Guess: An estimate of the interest rate to start the iterative 
/// computation. If omitted, 0.1 (10%) is assumed.
/// 
/// RATE solves this equation:
///
/// __See also__: "FV", "NPER", "PMT", "PV", 
#[inline]
pub fn rate_<A: Number, B: Number, C: Number, D: Number>(nper: A, payment: B, pv: C, fv: D) -> FnNumber4<A, B, C, D> {
    FnNumber4("RATE", nper, payment, pv, fv)
}

/// Compute the interest rate per period of an investment.
///
/// __Syntax__: 
/// ```ods
///     RATE( Nper Number; Payment Number; Pv Number; Fv Number; PayType Number )
/// ```
///
/// __Constraints__:
/// If Nper is 0 or less than 0, the result is an Error.
///
/// __Semantics__:
/// Computes the interest rate of an investment. The parameters are:
/// 
/// •Nper: the total number of payment periods.
/// 
/// •Payment: the payment made in each period.
/// 
/// •Pv: the present value of the investment.
/// 
/// •Fv: the future value; default is 0.
/// 
/// •PayType: the type of payment, defaults to 0. It is 0 if payments are due 
/// at the end of the period; 1 if they are due at the beginning of the period.
/// 
/// •Guess: An estimate of the interest rate to start the iterative 
/// computation. If omitted, 0.1 (10%) is assumed.
/// 
/// RATE solves this equation:
///
/// __See also__: "FV", "NPER", "PMT", "PV", 
#[inline]
pub fn rate__<A: Number, B: Number, C: Number, D: Number, E: Number>(nper: A, payment: B, pv: C, fv: D, pay_type: E) -> FnNumber5<A, B, C, D, E> {
    FnNumber5("RATE", nper, payment, pv, fv, pay_type)
}

/// Compute the interest rate per period of an investment.
///
/// __Syntax__: 
/// ```ods
///     RATE( Nper Number; Payment Number; Pv Number; Fv Number; PayType Number; Guess Number )
/// ```
///
/// __Constraints__:
/// If Nper is 0 or less than 0, the result is an Error.
///
/// __Semantics__:
/// Computes the interest rate of an investment. The parameters are:
/// 
/// •Nper: the total number of payment periods.
/// 
/// •Payment: the payment made in each period.
/// 
/// •Pv: the present value of the investment.
/// 
/// •Fv: the future value; default is 0.
/// 
/// •PayType: the type of payment, defaults to 0. It is 0 if payments are due 
/// at the end of the period; 1 if they are due at the beginning of the period.
/// 
/// •Guess: An estimate of the interest rate to start the iterative 
/// computation. If omitted, 0.1 (10%) is assumed.
/// 
/// RATE solves this equation:
///
/// __See also__: "FV", "NPER", "PMT", "PV", 
#[inline]
pub fn rate___<A: Number, B: Number, C: Number, D: Number, E: Number, F: Number>(nper: A, payment: B, pv: C, fv: D, pay_type: E, guess: F) -> FnNumber6<A, B, C, D, E, F> {
    FnNumber6("RATE", nper, payment, pv, fv, pay_type, guess)
}

/// Calculates the amount received at maturity for a zero coupon bond.
///
/// __Syntax__: 
/// ```ods
///     RECEIVED( Settlement DateParam; Maturity DateParam; Investment Number; Discount Number )
/// ```
///
/// __Constraints__:
/// Investment and Discount should be greater than 0, Settlement < Maturity
///
/// __Semantics__:
/// The parameters are:
/// 
/// Settlement: the settlement/purchase date of the security
/// 
/// •Maturity: the maturity/expiry date of the security
/// 
/// •Investment: the amount of investment in the security
/// 
/// •Discount: the discount rate of the security
/// 
/// •B: indicates the day-count convention to use in the calculation. 4.11.7
/// 
/// The returned value is:
///
/// __See also__: "YEARFRAC", 
#[inline]
pub fn received<A: DateTime, B: DateTime, C: Number, D: Number>(settlement: A, maturity: B, investment: C, discount: D) -> FnNumber4<A, B, C, D> {
    FnNumber4("RECEIVED", settlement, maturity, investment, discount)
}

/// Calculates the amount received at maturity for a zero coupon bond.
///
/// __Syntax__: 
/// ```ods
///     RECEIVED( Settlement DateParam; Maturity DateParam; Investment Number; Discount Number; B Basis )
/// ```
///
/// __Constraints__:
/// Investment and Discount should be greater than 0, Settlement < Maturity
///
/// __Semantics__:
/// The parameters are:
/// 
/// Settlement: the settlement/purchase date of the security
/// 
/// •Maturity: the maturity/expiry date of the security
/// 
/// •Investment: the amount of investment in the security
/// 
/// •Discount: the discount rate of the security
/// 
/// •B: indicates the day-count convention to use in the calculation. 4.11.7
/// 
/// The returned value is:
///
/// __See also__: "YEARFRAC", 
#[inline]
pub fn received_<A: DateTime, B: DateTime, C: Number, D: Number>(settlement: A, maturity: B, investment: C, discount: D, b: YearFracMethod) -> FnNumber5<A, B, C, D, YearFracMethod> {
    FnNumber5("RECEIVED", settlement, maturity, investment, discount, b)
}

/// Returns an equivalent interest rate when an investment increases in value.
///
/// __Syntax__: 
/// ```ods
///     RRI( Nper Number; Pv Number; Fv Number )
/// ```
///
/// __Constraints__:
/// Nper > 0
///
/// __Semantics__:
/// Returns the interest rate given Nper (the number of periods), Pv (present 
/// value), and Fv (future value), calculated as follows:
///
/// __See also__: "FV", "NPER", "PMT", "PV", "RATE", 
#[inline]
pub fn rri<A: Number, B: Number, C: Number>(nper: A, pv: B, fv: C) -> FnNumber3<A, B, C> {
    FnNumber3("RRI", nper, pv, fv)
}

/// Compute the amount of depreciation at a given period of time using the 
/// straight-line depreciation method.
///
/// __Syntax__: 
/// ```ods
///     SLN( Cost Number; Salvage Number; LifeTime Number )
/// ```
///
/// __Constraints__:
/// None.
///
/// __Semantics__:
/// Compute the amount of depreciation of an asset at a given period of time 
/// using straight-line depreciation. The parameters are:
/// 
/// •Cost: the total amount paid for the asset.
/// 
/// •Salvage: the salvage value at the end of the LifeTime (often 0)
/// 
/// •LifeTime: the number of periods that the depreciation will occur over. A 
/// positive integer.
/// 
/// For alternative methods to compute depreciation, see DDB 6.12.14.
#[inline]
pub fn sln<A: Number, B: Number, C: Number>(cost: A, salvage: B, life_time: C) -> FnNumber3<A, B, C> {
    FnNumber3("SLN", cost, salvage, life_time)
}

/// Compute the amount of depreciation at a given period of time using the 
/// Sum-of-the-Years'-Digits method.
///
/// __Syntax__: 
/// ```ods
///     SYD( Cost Number; Salvage Number; LifeTime Number; Period Number )
/// ```
///
/// __Constraints__:
/// None.
///
/// __Semantics__:
/// Compute the amount of depreciation of an asset at a given period of time 
/// using the Sum-of-the-Years'-Digits method. The parameters are:
/// 
/// •Cost: the total amount paid for the asset.
/// 
/// •Salvage: the salvage value at the end of the LifeTime (often 0).
/// 
/// •LifeTime: the number of periods that the depreciation will occur over. A 
/// positive integer.
/// 
/// •Period: the period for which the depreciation value is specified.
/// 
/// For other methods of computing depreciation, see DDB 6.12.14.
///
/// __See also__: "SLN", 
#[inline]
pub fn syd<A: Number, B: Number, C: Number, D: Number>(cost: A, salvage: B, life_time: C, period: D) -> FnNumber4<A, B, C, D> {
    FnNumber4("SYD", cost, salvage, life_time, period)
}

/// Compute the bond-equivalent yield for a treasury bill.
///
/// __Syntax__: 
/// ```ods
///     TBILLEQ( Settlement DateParam; Maturity DateParam; Discount Number )
/// ```
///
/// __Constraints__:
/// The maturity date should be less than one year beyond settlement date. 
/// Discount is any positive value.
///
/// __Semantics__:
/// The parameters are defined as:
/// 
/// •Settlement: the settlement/purchase date of the treasury bill.
/// 
/// •Maturity: the maturity/expiry date of the treasury bill.
/// 
/// •Discount: the discount rate of the treasury bill.
/// 
/// TBILLEQ is calculated as
/// 
/// where DSM is the number of days between settlement and maturity computed 
/// according to the 360 days per year basis (basis 2, 4.11.7)
///
/// __See also__: "TBILLPRICE", "TBILLYIELD", 
#[inline]
pub fn tbilleq<A: DateTime, B: DateTime, C: Number>(settlement: A, maturity: B, discount: C) -> FnNumber3<A, B, C> {
    FnNumber3("TBILLEQ", settlement, maturity, discount)
}

/// Compute the price per 100 face value for a treasury bill.
///
/// __Syntax__: 
/// ```ods
///     TBILLPRICE( Settlement DateParam; Maturity DateParam; Discount Number )
/// ```
///
/// __Constraints__:
/// The maturity date should be less than one year beyond settlement. Discount 
/// is any positive value.
///
/// __Semantics__:
/// The parameters are:
/// 
/// •Settlement: the settlement/purchase date of the treasury bill.
/// 
/// •Maturity: the maturity/expiry date of the treasury bill.
/// 
/// •Discount: the discount rate of the treasury bill.
///
/// __See also__: "TBILLEQ", "TBILLYIELD", 
#[inline]
pub fn tbillprice<A: DateTime, B: DateTime, C: Number>(settlement: A, maturity: B, discount: C) -> FnNumber3<A, B, C> {
    FnNumber3("TBILLPRICE", settlement, maturity, discount)
}

/// Compute the yield for a treasury bill.
///
/// __Syntax__: 
/// ```ods
///     TBILLYIELD( Settlement DateParam; Maturity DateParam; Price Number )
/// ```
///
/// __Constraints__:
/// The maturity date should be less than one year beyond settlement. Price is 
/// any positive value.
///
/// __Semantics__:
/// The parameters are:
/// 
/// •Settlement: the settlement/purchase date of the treasury bill.
/// 
/// •Maturity: the maturity/expiry date of the treasury bill.
/// 
/// •Price: the price of the treasury bill per 100 face value
///
/// __See also__: "TBILLEQ", "TBILLPRICE", 
#[inline]
pub fn tbillyield<A: DateTime, B: DateTime, C: Number>(settlement: A, maturity: B, price: C) -> FnNumber3<A, B, C> {
    FnNumber3("TBILLYIELD", settlement, maturity, price)
}

/// Calculates the depreciation allowance of an asset with an initial value, an 
/// expected useful life, and a final value of salvage for a period specified, 
/// using the variable-rate declining balance method..
///
/// __Syntax__: 
/// ```ods
///     VDB( Cost Number; Salvage Number; LifeTime Number; StartPeriod Number; EndPeriod Number )
/// ```
///
/// __Constraints__:
/// Salvage < Cost, LifeTime > 0, 0 ≤ StartPeriod ≤ LifeTime, StartPeriod 
/// ≤ EndPeriod ≤ LifeTime, DepreciationFactor ≥ 0
///
/// __Semantics__:
/// The parameters are:
/// 
/// •Cost is the amount paid for the asset. Cost can be any value greater 
/// than Salvage.
/// 
/// •Salvage is the value of the asset at the end of its life. Salvage can be 
/// any value.
/// 
/// •LifeTime is the number of periods the asset takes to depreciate to its 
/// salvage value. LifeTime can be any value greater than 0.
/// 
/// •StartPeriod is the point in the asset's life when you want to begin 
/// calculating depreciation. StartPeriod can be any value greater than or 
/// equal to 0, but cannot be greater than LifeTime.
/// 
/// •EndPeriod is the point in the asset's life when you want to stop 
/// calculating depreciation. EndPeriod can be any value greater than 
/// StartPeriod.
/// 
/// •StartPeriod and EndPeriod correspond to the asset's life, relative to 
/// the fiscal period. For example, if you want to find the first year's 
/// depreciation of an asset purchased at the beginning of the second quarter 
/// of a fiscal year, StartPeriod would be 0 and EndPeriod would be 0.75 (1 
/// minus 0.25 of a year).
/// 
/// VDB allows for the use of an initialPeriod option to calculate depreciation 
/// for the period the asset is placed in service. VDB uses the fractional part 
/// of StartPeriod and EndPeriod to determine the initialPeriod option. If both 
/// StartPeriod and EndPeriod have fractional parts, then VDB uses the 
/// fractional part of StartPeriod.
/// 
/// DepreciationFactor is an optional argument that specifies the percentage of 
/// straight-line depreciation you want to use as the depreciation rate. If you 
/// omit this argument, VDB uses 2, which is the double-declining balance rate. 
/// DepreciationFactor can be any value greater than or equal to 0; commonly 
/// used rates are 1.25, 1.50, 1.75, and 2.
/// 
/// NoSwitch is an optional argument that you include if you do not want VDB to 
/// switch to straight-line depreciation for the remaining useful life. 
/// Normally, declining-balance switches to such a straight-line calculation 
/// when it is greater than the declining-balance calculation.
/// 
/// If NoSwitch is FALSE or omitted, VDB automatically switches to 
/// straight-line depreciation when that is greater than declining-balance 
/// depreciation. If NoSwitch is TRUE, VDB never switches to straight-line 
/// depreciation.
///
/// __See also__: "DDB", "SLN", 
#[inline]
pub fn vdb<A: Number, B: Number, C: Number, D: Number, E: Number>(cost: A, salvage: B, life_time: C, start_period: D, end_period: E) -> FnNumber5<A, B, C, D, E> {
    FnNumber5("VDB", cost, salvage, life_time, start_period, end_period)
}

/// Calculates the depreciation allowance of an asset with an initial value, an 
/// expected useful life, and a final value of salvage for a period specified, 
/// using the variable-rate declining balance method..
///
/// __Syntax__: 
/// ```ods
///     VDB( Cost Number; Salvage Number; LifeTime Number; StartPeriod Number; EndPeriod Number; DepreciationFactor Number )
/// ```
///
/// __Constraints__:
/// Salvage < Cost, LifeTime > 0, 0 ≤ StartPeriod ≤ LifeTime, StartPeriod 
/// ≤ EndPeriod ≤ LifeTime, DepreciationFactor ≥ 0
///
/// __Semantics__:
/// The parameters are:
/// 
/// •Cost is the amount paid for the asset. Cost can be any value greater 
/// than Salvage.
/// 
/// •Salvage is the value of the asset at the end of its life. Salvage can be 
/// any value.
/// 
/// •LifeTime is the number of periods the asset takes to depreciate to its 
/// salvage value. LifeTime can be any value greater than 0.
/// 
/// •StartPeriod is the point in the asset's life when you want to begin 
/// calculating depreciation. StartPeriod can be any value greater than or 
/// equal to 0, but cannot be greater than LifeTime.
/// 
/// •EndPeriod is the point in the asset's life when you want to stop 
/// calculating depreciation. EndPeriod can be any value greater than 
/// StartPeriod.
/// 
/// •StartPeriod and EndPeriod correspond to the asset's life, relative to 
/// the fiscal period. For example, if you want to find the first year's 
/// depreciation of an asset purchased at the beginning of the second quarter 
/// of a fiscal year, StartPeriod would be 0 and EndPeriod would be 0.75 (1 
/// minus 0.25 of a year).
/// 
/// VDB allows for the use of an initialPeriod option to calculate depreciation 
/// for the period the asset is placed in service. VDB uses the fractional part 
/// of StartPeriod and EndPeriod to determine the initialPeriod option. If both 
/// StartPeriod and EndPeriod have fractional parts, then VDB uses the 
/// fractional part of StartPeriod.
/// 
/// DepreciationFactor is an optional argument that specifies the percentage of 
/// straight-line depreciation you want to use as the depreciation rate. If you 
/// omit this argument, VDB uses 2, which is the double-declining balance rate. 
/// DepreciationFactor can be any value greater than or equal to 0; commonly 
/// used rates are 1.25, 1.50, 1.75, and 2.
/// 
/// NoSwitch is an optional argument that you include if you do not want VDB to 
/// switch to straight-line depreciation for the remaining useful life. 
/// Normally, declining-balance switches to such a straight-line calculation 
/// when it is greater than the declining-balance calculation.
/// 
/// If NoSwitch is FALSE or omitted, VDB automatically switches to 
/// straight-line depreciation when that is greater than declining-balance 
/// depreciation. If NoSwitch is TRUE, VDB never switches to straight-line 
/// depreciation.
///
/// __See also__: "DDB", "SLN", 
#[inline]
pub fn vdb_<A: Number, B: Number, C: Number, D: Number, E: Number, F: Number>(cost: A, salvage: B, life_time: C, start_period: D, end_period: E, depreciation_factor: F) -> FnNumber6<A, B, C, D, E, F> {
    FnNumber6("VDB", cost, salvage, life_time, start_period, end_period, depreciation_factor)
}

/// Calculates the depreciation allowance of an asset with an initial value, an 
/// expected useful life, and a final value of salvage for a period specified, 
/// using the variable-rate declining balance method..
///
/// __Syntax__: 
/// ```ods
///     VDB( Cost Number; Salvage Number; LifeTime Number; StartPeriod Number; EndPeriod Number; DepreciationFactor Number; NoSwitch Logical )
/// ```
///
/// __Constraints__:
/// Salvage < Cost, LifeTime > 0, 0 ≤ StartPeriod ≤ LifeTime, StartPeriod 
/// ≤ EndPeriod ≤ LifeTime, DepreciationFactor ≥ 0
///
/// __Semantics__:
/// The parameters are:
/// 
/// •Cost is the amount paid for the asset. Cost can be any value greater 
/// than Salvage.
/// 
/// •Salvage is the value of the asset at the end of its life. Salvage can be 
/// any value.
/// 
/// •LifeTime is the number of periods the asset takes to depreciate to its 
/// salvage value. LifeTime can be any value greater than 0.
/// 
/// •StartPeriod is the point in the asset's life when you want to begin 
/// calculating depreciation. StartPeriod can be any value greater than or 
/// equal to 0, but cannot be greater than LifeTime.
/// 
/// •EndPeriod is the point in the asset's life when you want to stop 
/// calculating depreciation. EndPeriod can be any value greater than 
/// StartPeriod.
/// 
/// •StartPeriod and EndPeriod correspond to the asset's life, relative to 
/// the fiscal period. For example, if you want to find the first year's 
/// depreciation of an asset purchased at the beginning of the second quarter 
/// of a fiscal year, StartPeriod would be 0 and EndPeriod would be 0.75 (1 
/// minus 0.25 of a year).
/// 
/// VDB allows for the use of an initialPeriod option to calculate depreciation 
/// for the period the asset is placed in service. VDB uses the fractional part 
/// of StartPeriod and EndPeriod to determine the initialPeriod option. If both 
/// StartPeriod and EndPeriod have fractional parts, then VDB uses the 
/// fractional part of StartPeriod.
/// 
/// DepreciationFactor is an optional argument that specifies the percentage of 
/// straight-line depreciation you want to use as the depreciation rate. If you 
/// omit this argument, VDB uses 2, which is the double-declining balance rate. 
/// DepreciationFactor can be any value greater than or equal to 0; commonly 
/// used rates are 1.25, 1.50, 1.75, and 2.
/// 
/// NoSwitch is an optional argument that you include if you do not want VDB to 
/// switch to straight-line depreciation for the remaining useful life. 
/// Normally, declining-balance switches to such a straight-line calculation 
/// when it is greater than the declining-balance calculation.
/// 
/// If NoSwitch is FALSE or omitted, VDB automatically switches to 
/// straight-line depreciation when that is greater than declining-balance 
/// depreciation. If NoSwitch is TRUE, VDB never switches to straight-line 
/// depreciation.
///
/// __See also__: "DDB", "SLN", 
#[inline]
pub fn vdb__<A: Number, B: Number, C: Number, D: Number, E: Number, F: Number, G: Logical>(cost: A, salvage: B, life_time: C, start_period: D, end_period: E, depreciation_factor: F, no_switch: G) -> FnNumber7<A, B, C, D, E, F, G> {
    FnNumber7("VDB", cost, salvage, life_time, start_period, end_period, depreciation_factor, no_switch)
}

/// Compute the internal rate of return for a non-periodic series of cash 
/// flows.
///
/// __Syntax__: 
/// ```ods
///     XIRR( Values NumberSequence; Dates DateSequence )
/// ```
///
/// __Constraints__:
/// The size of Values and Dates are equal. Values contains at least one 
/// positive and one negative cash flow.
///
/// __Semantics__:
/// Compute the internal rate of return for a series of cash flows which is not 
/// necessarily periodic. The parameters are:
/// 
/// •Values: a series of cash flows. The first cash-flow amount is a negative 
/// number that represents the investment. The later cash flows are discounted 
/// based on the annual discount rate and the timing of the flow. The series of 
/// cash flow should contain at least one positive and one negative value.
/// 
/// •Dates: a series of dates that corresponds to values. The first date 
/// indicates the start of the cash flows. The range of Values and Dates shall 
/// be the same size.
/// 
/// •Guess: If provided, Guess is an estimate of the interest rate to start 
/// the iterative computation. If omitted, the value 0.1 (10%) is assumed. The 
/// result of XIRR is the rate at which the XNPV() function will return zero 
/// with the given cash flows. There is no closed form for XIRR. 
/// Implementations may return an approximate solution using an iterative 
/// method, in which case the Guess parameter may be used to initialize the 
/// iteration. If the implementation is unable to converge on a solution given 
/// a particular Guess, it may return an error.
///
/// __See also__: "IRR", "XNPV", 
#[inline]
pub fn xirr<A: Sequence, B: Sequence>(values: A, dates: B) -> FnNumber2<A, B> {
    FnNumber2("XIRR", values, dates)
}

/// Compute the internal rate of return for a non-periodic series of cash 
/// flows.
///
/// __Syntax__: 
/// ```ods
///     XIRR( Values NumberSequence; Dates DateSequence; Guess Number )
/// ```
///
/// __Constraints__:
/// The size of Values and Dates are equal. Values contains at least one 
/// positive and one negative cash flow.
///
/// __Semantics__:
/// Compute the internal rate of return for a series of cash flows which is not 
/// necessarily periodic. The parameters are:
/// 
/// •Values: a series of cash flows. The first cash-flow amount is a negative 
/// number that represents the investment. The later cash flows are discounted 
/// based on the annual discount rate and the timing of the flow. The series of 
/// cash flow should contain at least one positive and one negative value.
/// 
/// •Dates: a series of dates that corresponds to values. The first date 
/// indicates the start of the cash flows. The range of Values and Dates shall 
/// be the same size.
/// 
/// •Guess: If provided, Guess is an estimate of the interest rate to start 
/// the iterative computation. If omitted, the value 0.1 (10%) is assumed. The 
/// result of XIRR is the rate at which the XNPV() function will return zero 
/// with the given cash flows. There is no closed form for XIRR. 
/// Implementations may return an approximate solution using an iterative 
/// method, in which case the Guess parameter may be used to initialize the 
/// iteration. If the implementation is unable to converge on a solution given 
/// a particular Guess, it may return an error.
///
/// __See also__: "IRR", "XNPV", 
#[inline]
pub fn xirr_<A: Sequence, B: Sequence, C: Number>(values: A, dates: B, guess: C) -> FnNumber3<A, B, C> {
    FnNumber3("XIRR", values, dates, guess)
}

/// Compute the net present value of a series of cash flows.
///
/// __Syntax__: 
/// ```ods
///     XNPV( Rate Number; Values Reference|Array; Dates Reference|Array )
/// ```
///
/// __Constraints__:
/// 
/// Number of elements in Values equals number of elements in Dates.
/// 
/// All elements of Values are of type Number.
/// 
/// All elements of Dates are of type Number.
/// 
/// All elements of Dates ≥ Dates[1]
///
/// __Semantics__:
/// Compute the net present value for a series of cash flows which is not 
/// necessarily periodic. The parameters are:
/// 
/// •Rate: discount rate. The value should be greater than -1.
/// 
/// •Values: a series of cash flows. The first cash-flow amount is a negative 
/// number that represents the investment. The later cash flows are discounted 
/// based on the annual discount rate and the timing of the flow. The series of 
/// cash flow should contain at least one positive and one negative value.
/// 
/// •Dates: a series of dates that corresponds to values. The first date 
/// indicates the start of the cash flows. If the dimensions of the Values and 
/// Dates arrays differ, evaluators shall match value and date pairs row-wise 
/// starting from top left.
/// 
/// With N being the number of elements in Values and Dates each, the formula 
/// is:
///
/// __See also__: "NPV", 
#[inline]
pub fn xnpv<A: Number, B: ReferenceOrArray, C: ReferenceOrArray>(rate: A, values: B, dates: C) -> FnNumber3<A, B, C> {
    FnNumber3("XNPV", rate, values, dates)
}

/// Calculate the yield of a bond.
///
/// __Syntax__: 
/// ```ods
///     YIELD( Settlement DateParam; Maturity DateParam; Rate Number; Price Number; Redemption Number; Frequency Number )
/// ```
///
/// __Constraints__:
/// Rate, Price, and Redemption should be greater than 0.
///
/// __Semantics__:
/// The parameters are:
/// 
/// •Settlement: the settlement/purchase date of the bond.
/// 
/// •Maturity: the maturity/expiry date of the bond.
/// 
/// •Rate: the interest rate of the bond.
/// 
/// •Price: the price of the bond per 100 currency units face value.
/// 
/// •Redemption: the redemption value of the bond per 100 currency units face 
/// value.
/// 
/// •Frequency: the number of interest payments per year. 1 = annual; 2 = 
/// semiannual; 4 = quarterly.
/// 
/// •B: indicates the day-count convention to use in the calculation. 4.11.7
///
/// __See also__: "PRICE", "YIELDDISC", "YIELDMAT", 
#[inline]
pub fn yield_<A: DateTime, B: DateTime, C: Number, D: Number, E: Number>(settlement: A, maturity: B, rate: C, price: D, redemption: E, frequency: Frequency) -> FnNumber6<A, B, C, D, E, Frequency> {
    FnNumber6("YIELD", settlement, maturity, rate, price, redemption, frequency)
}

/// Calculate the yield of a bond.
///
/// __Syntax__: 
/// ```ods
///     YIELD( Settlement DateParam; Maturity DateParam; Rate Number; Price Number; Redemption Number; Frequency Number; B Basis )
/// ```
///
/// __Constraints__:
/// Rate, Price, and Redemption should be greater than 0.
///
/// __Semantics__:
/// The parameters are:
/// 
/// •Settlement: the settlement/purchase date of the bond.
/// 
/// •Maturity: the maturity/expiry date of the bond.
/// 
/// •Rate: the interest rate of the bond.
/// 
/// •Price: the price of the bond per 100 currency units face value.
/// 
/// •Redemption: the redemption value of the bond per 100 currency units face 
/// value.
/// 
/// •Frequency: the number of interest payments per year. 1 = annual; 2 = 
/// semiannual; 4 = quarterly.
/// 
/// •B: indicates the day-count convention to use in the calculation. 4.11.7
///
/// __See also__: "PRICE", "YIELDDISC", "YIELDMAT", 
#[inline]
pub fn yield__<A: DateTime, B: DateTime, C: Number, D: Number, E: Number>(settlement: A, maturity: B, rate: C, price: D, redemption: E, frequency: Frequency, b: YearFracMethod) -> FnNumber7<A, B, C, D, E, Frequency, YearFracMethod> {
    FnNumber7("YIELD", settlement, maturity, rate, price, redemption, frequency, b)
}

/// Calculate the yield of a discounted security per 100 currency units of face 
/// value.
///
/// __Syntax__: 
/// ```ods
///     YIELDDISC( Settlement DateParam; Maturity DateParam; Price Number; Redemption Number )
/// ```
///
/// __Constraints__:
/// Price and Redemption should be greater than 0.
///
/// __Semantics__:
/// The parameters are:
/// 
/// •Settlement: the settlement/purchase date of the security.
/// 
/// •Maturity: the maturity/expiry date of the security.
/// 
/// •Price: the price of the security per 100 currency units face value.
/// 
/// •Redemption: the redemption value per 100 currency units face value.
/// 
/// •B: indicates the day-count convention to use in the calculation. 4.11.7
/// 
/// The return value is
///
/// __See also__: "PRICEDISC", "YEARFRAC", 
#[inline]
pub fn yielddisc<A: DateTime, B: DateTime, C: Number, D: Number>(settlement: A, maturity: B, price: C, redemption: D) -> FnNumber4<A, B, C, D> {
    FnNumber4("YIELDDISC", settlement, maturity, price, redemption)
}

/// Calculate the yield of a discounted security per 100 currency units of face 
/// value.
///
/// __Syntax__: 
/// ```ods
///     YIELDDISC( Settlement DateParam; Maturity DateParam; Price Number; Redemption Number; B Basis )
/// ```
///
/// __Constraints__:
/// Price and Redemption should be greater than 0.
///
/// __Semantics__:
/// The parameters are:
/// 
/// •Settlement: the settlement/purchase date of the security.
/// 
/// •Maturity: the maturity/expiry date of the security.
/// 
/// •Price: the price of the security per 100 currency units face value.
/// 
/// •Redemption: the redemption value per 100 currency units face value.
/// 
/// •B: indicates the day-count convention to use in the calculation. 4.11.7
/// 
/// The return value is
///
/// __See also__: "PRICEDISC", "YEARFRAC", 
#[inline]
pub fn yielddisc_<A: DateTime, B: DateTime, C: Number, D: Number>(settlement: A, maturity: B, price: C, redemption: D, b: YearFracMethod) -> FnNumber5<A, B, C, D, YearFracMethod> {
    FnNumber5("YIELDDISC", settlement, maturity, price, redemption, b)
}

/// Calculate the yield of the security that pays interest on the maturity 
/// date.
///
/// __Syntax__: 
/// ```ods
///     YIELDMAT( Settlement DateParam; Maturity DateParam; Issue DateParam; Rate Number; Price Number )
/// ```
///
/// __Constraints__:
/// Rate and Price should be greater than 0.
///
/// __Semantics__:
/// The parameters are:
/// 
/// •Settlement: the settlement/purchase date of the security.
/// 
/// •Maturity: the maturity/expiry date of the security.
/// 
/// •Issue: the issue date of the security.
/// 
/// •Rate: the interest rate of the security.
/// 
/// •Price: the price of the security per 100 currency units face value.
/// 
/// •B: indicates the day-count convention to use in the calculation. 4.11.7
///
/// __See also__: "PRICE", "YIELD", "YIELDDISC", 
#[inline]
pub fn yieldmat<A: DateTime, B: DateTime, C: DateTime, D: Number, E: Number>(settlement: A, maturity: B, issue: C, rate: D, price: E) -> FnNumber5<A, B, C, D, E> {
    FnNumber5("YIELDMAT", settlement, maturity, issue, rate, price)
}

/// Calculate the yield of the security that pays interest on the maturity 
/// date.
///
/// __Syntax__: 
/// ```ods
///     YIELDMAT( Settlement DateParam; Maturity DateParam; Issue DateParam; Rate Number; Price Number; B Basis )
/// ```
///
/// __Constraints__:
/// Rate and Price should be greater than 0.
///
/// __Semantics__:
/// The parameters are:
/// 
/// •Settlement: the settlement/purchase date of the security.
/// 
/// •Maturity: the maturity/expiry date of the security.
/// 
/// •Issue: the issue date of the security.
/// 
/// •Rate: the interest rate of the security.
/// 
/// •Price: the price of the security per 100 currency units face value.
/// 
/// •B: indicates the day-count convention to use in the calculation. 4.11.7
///
/// __See also__: "PRICE", "YIELD", "YIELDDISC", 
#[inline]
pub fn yieldmat_<A: DateTime, B: DateTime, C: DateTime, D: Number, E: Number>(settlement: A, maturity: B, issue: C, rate: D, price: E, b: YearFracMethod) -> FnNumber6<A, B, C, D, E, YearFracMethod> {
    FnNumber6("YIELDMAT", settlement, maturity, issue, rate, price, b)
}
