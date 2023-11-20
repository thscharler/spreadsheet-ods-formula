use crate::*;
#[allow(unused_imports)]
use crate::date::*;

#[inline]
pub fn date<A: Number, B: Number, C: Number>(year: A, month: B, day: C) -> FnNumber3<A, B, C> {
    FnNumber3("DATE", year, month, day)
}

#[inline]
pub fn datedif<A: DateTime, B: DateTime, C: Text>(start_date: A, end_date: B, format: C) -> FnNumber3<A, B, C> {
    FnNumber3("DATEDIF", start_date, end_date, format)
}

#[inline]
pub fn datevalue<A: Text>(d: A) -> FnNumber1<A> {
    FnNumber1("DATEVALUE", d)
}

#[inline]
pub fn day<A: DateTime>(d: A) -> FnNumber1<A> {
    FnNumber1("DAY", d)
}

#[inline]
pub fn days<A: DateTime, B: DateTime>(end_date: A, start_date: B) -> FnNumber2<A, B> {
    FnNumber2("DAYS", end_date, start_date)
}

#[inline]
pub fn days360<A: DateTime, B: DateTime>(start_date: A, end_date: B) -> FnNumber2<A, B> {
    FnNumber2("DAYS360", start_date, end_date)
}

#[inline]
pub fn days360_<A: DateTime, B: DateTime>(start_date: A, end_date: B, method: Days360Method) -> FnNumber3<A, B, Days360Method> {
    FnNumber3("DAYS360", start_date, end_date, method)
}

#[inline]
pub fn edate<A: DateTime, B: Number>(start_date: A, month_add: B) -> FnNumber2<A, B> {
    FnNumber2("EDATE", start_date, month_add)
}

#[inline]
pub fn eomonth<A: DateTime, B: Number>(start_date: A, month_add: B) -> FnNumber2<A, B> {
    FnNumber2("EOMONTH", start_date, month_add)
}

#[inline]
pub fn hour<A: DateTime>(t: A) -> FnNumber1<A> {
    FnNumber1("HOUR", t)
}

#[inline]
pub fn isoweeknum<A: DateTime>(d: A) -> FnNumber1<A> {
    FnNumber1("ISOWEEKNUM", d)
}

#[inline]
pub fn minute<A: DateTime>(t: A) -> FnNumber1<A> {
    FnNumber1("MINUTE", t)
}

#[inline]
pub fn month<A: DateTime>(date: A) -> FnNumber1<A> {
    FnNumber1("MONTH", date)
}

#[inline]
pub fn networkdays<A: DateTime, B: DateTime>(date1: A, date2: B) -> FnNumber2<A, B> {
    FnNumber2("NETWORKDAYS", date1, date2)
}

#[inline]
pub fn networkdays_<A: DateTime, B: DateTime, C: Sequence>(date1: A, date2: B, holidays: C) -> FnNumber3<A, B, C> {
    FnNumber3("NETWORKDAYS", date1, date2, holidays)
}

#[inline]
pub fn networkdays__<A: DateTime, B: DateTime, C: Sequence, D: Sequence>(date1: A, date2: B, holidays: C, workdays: D) -> FnNumber4<A, B, C, D> {
    FnNumber4("NETWORKDAYS", date1, date2, holidays, workdays)
}

#[inline]
pub fn now() -> FnNumber0 {
    FnNumber0("NOW", )
}

#[inline]
pub fn second<A: DateTime>(t: A) -> FnNumber1<A> {
    FnNumber1("SECOND", t)
}

#[inline]
pub fn time<A: Number, B: Number, C: Number>(hours: A, minutes: B, seconds: C) -> FnNumber3<A, B, C> {
    FnNumber3("TIME", hours, minutes, seconds)
}

#[inline]
pub fn timevalue<A: Text>(t: A) -> FnNumber1<A> {
    FnNumber1("TIMEVALUE", t)
}

#[inline]
pub fn today() -> FnNumber0 {
    FnNumber0("TODAY", )
}

#[inline]
pub fn weekday<A: DateTime>(d: A) -> FnNumber1<A> {
    FnNumber1("WEEKDAY", d)
}

#[inline]
pub fn weekday_<A: DateTime>(d: A, type_: WeekdayMethod) -> FnNumber2<A, WeekdayMethod> {
    FnNumber2("WEEKDAY", d, type_)
}

#[inline]
pub fn weeknum<A: DateTime>(d: A) -> FnNumber1<A> {
    FnNumber1("WEEKNUM", d)
}

#[inline]
pub fn weeknum_<A: DateTime>(d: A, mode: WeeknumMethod) -> FnNumber2<A, WeeknumMethod> {
    FnNumber2("WEEKNUM", d, mode)
}

#[inline]
pub fn workday<A: DateTime, B: Number>(date: A, offset: B) -> FnNumber2<A, B> {
    FnNumber2("WORKDAY", date, offset)
}

#[inline]
pub fn workday_<A: DateTime, B: Number, C: Sequence>(date: A, offset: B, holidays: C) -> FnNumber3<A, B, C> {
    FnNumber3("WORKDAY", date, offset, holidays)
}

#[inline]
pub fn workday__<A: DateTime, B: Number, C: Sequence, D: Sequence>(date: A, offset: B, holidays: C, workdays: D) -> FnNumber4<A, B, C, D> {
    FnNumber4("WORKDAY", date, offset, holidays, workdays)
}

#[inline]
pub fn year<A: DateTime>(d: A) -> FnNumber1<A> {
    FnNumber1("YEAR", d)
}

#[inline]
pub fn yearfrac<A: DateTime, B: DateTime>(start_date: A, end_date: B) -> FnNumber2<A, B> {
    FnNumber2("YEARFRAC", start_date, end_date)
}

#[inline]
pub fn yearfrac_<A: DateTime, B: DateTime>(start_date: A, end_date: B, b: YearFracMethod) -> FnNumber3<A, B, YearFracMethod> {
    FnNumber3("YEARFRAC", start_date, end_date, b)
}
