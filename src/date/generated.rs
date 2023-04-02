use crate::*;
#[allow(unused_imports)]
use super::*;

///  Constructs a date from year, month, and day of month
#[inline]
pub fn date<A: Number, B: Number, C: Number>(year: A, month: B, day: C) -> FnNumber3<A, B, C> {
    FnNumber3("DATE", year, month, day)
}

///  Returns the difference in years, months, or days of two date numbers.
#[inline]
pub fn datedif<A: DateTimeParam, B: DateTimeParam, C: Text>(start_date: A, end_date: B, format: C) -> FnNumber3<A, B, C> {
    FnNumber3("DATEDIF", start_date, end_date, format)
}

///  Returns the date serial number from given text
#[inline]
pub fn datevalue<A: Text>(d: A) -> FnNumber1<A> {
    FnNumber1("DATEVALUE", d)
}

///  Returns the day from a date.
#[inline]
pub fn day<A: DateTimeParam>(d: A) -> FnNumber1<A> {
    FnNumber1("DAY", d)
}

///  Returns the number of days between two dates
#[inline]
pub fn days<A: DateTimeParam, B: DateTimeParam>(end_date: A, start_date: B) -> FnNumber2<A, B> {
    FnNumber2("DAYS", end_date, start_date)
}

///  Returns the number of days between two dates using the 360-day year
#[inline]
pub fn days360<A: DateTimeParam, B: DateTimeParam>(start_date: A, end_date: B, method: Days360Method) -> FnNumber3<A, B, Days360Method> {
    FnNumber3("DAYS360", start_date, end_date, method)
}

///  Returns the serial number of a given date when MonthAdd months is added
#[inline]
pub fn edate<A: DateTimeParam, B: DateTimeParam>(start_date: A, month_add: B) -> FnNumber2<A, B> {
    FnNumber2("EDATE", start_date, month_add)
}

///  Returns the serial number of the end of a month, given date plus MonthAdd months
#[inline]
pub fn eomonth<A: DateTimeParam, B: DateTimeParam>(start_date: A, month_add: B) -> FnNumber2<A, B> {
    FnNumber2("EOMONTH", start_date, month_add)
}

///  Extracts the hour (0 through 23) from a time
#[inline]
pub fn hour<A: DateTimeParam>(t: A) -> FnNumber1<A> {
    FnNumber1("HOUR", t)
}

///  Determines the ISO week number of the year for a given date.
#[inline]
pub fn isoweeknum<A: DateTimeParam>(d: A) -> FnNumber1<A> {
    FnNumber1("ISOWEEKNUM", d)
}

///  Extracts the minute (0 through 59) from a time.
#[inline]
pub fn minute<A: DateTimeParam>(t: A) -> FnNumber1<A> {
    FnNumber1("MINUTE", t)
}

///  Extracts the month from a date
#[inline]
pub fn month<A: DateTimeParam>(d: A) -> FnNumber1<A> {
    FnNumber1("MONTH", d)
}

///  Returns the whole number of work days between two dates.
#[inline]
pub fn networkdays<A: DateTimeParam, B: DateTimeParam, C: Array, D: Array>(date1: A, date2: B, holidays: Option<C>, workdays: Option<D>) -> FnNumber4<A, B, Option<C>, Option<D>> {
    FnNumber4("NETWORKDAYS", date1, date2, holidays, workdays)
}

///  Returns the serial number of the current date and time.
#[inline]
pub fn now() -> FnNumber0 {
    FnNumber0("NOW", )
}

///  Extracts the second (the integer 0 through 59) from a time. This function presumes that leap seconds never exist.
#[inline]
pub fn second<A: DateTimeParam>(t: A) -> FnNumber1<A> {
    FnNumber1("SECOND", t)
}

///  Constructs a time value from hours, minutes, and seconds.
#[inline]
pub fn time<A: Number, B: Number, C: Number>(hours: A, minutes: B, seconds: C) -> FnNumber3<A, B, C> {
    FnNumber3("TIME", hours, minutes, seconds)
}

///  Returns a time serial number from given text
#[inline]
pub fn timevalue<A: Text>(t: A) -> FnNumber1<A> {
    FnNumber1("TIMEVALUE", t)
}

///  Returns the serial number of today.
#[inline]
pub fn today() -> FnNumber0 {
    FnNumber0("TODAY", )
}

///  Extracts the day of the week from a date; if text, uses current locale to convert to a date.
#[inline]
pub fn weekday<A: DateTimeParam>(d: A, t: WeekdayMethod) -> FnNumber2<A, WeekdayMethod> {
    FnNumber2("WEEKDAY", d, t)
}

///  Determines the week number of the year for a given date.
#[inline]
pub fn weeknum<A: DateTimeParam>(d: A, m: WeeknumMethod) -> FnNumber2<A, WeeknumMethod> {
    FnNumber2("WEEKNUM", d, m)
}

///  Returns the date serial number which is a specified number of work days before or after an input date.
#[inline]
pub fn workday<A: DateTimeParam, B: Number, C: Sequence, D: Sequence>(d: A, offset: B, holidays: Option<C>, workdays: Option<D>) -> FnNumber4<A, B, Option<C>, Option<D>> {
    FnNumber4("WORKDAY", d, offset, holidays, workdays)
}

///  Extracts the year from a date given in the current locale of the evaluator
#[inline]
pub fn year<A: DateTimeParam>(d: A) -> FnNumber1<A> {
    FnNumber1("YEAR", d)
}

///  Extracts the number of years (including fractional part) between two dates
#[inline]
pub fn yearfrac<A: DateTimeParam, B: DateTimeParam>(start_date: A, end_date: B, b: Option<YearFracMethod>) -> FnNumber3<A, B, Option<YearFracMethod>> {
    FnNumber3("YEARFRAC", start_date, end_date, b)
}
