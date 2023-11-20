
use crate::*;
#[allow(unused_imports)]
use crate::date::*;

/// Constructs a date from year, month, and day of month.
/// Syntax: DATE( Year Integer;; Month Integer;; Day Integer; )
///
/// Constraints:
/// 1904 ≤ Year ≤ 9956; 1 ≤ Month ≤ 12; 1 ≤ Day ≤ 31; Evaluators 
/// may evaluate expressions that do no meet this constraint.
///
/// Semantics:
/// This computes the date's serial number given Year, Month, and Day of the 
/// Gregorian calendar. Fractional values are truncated. Month > 12 and Day > 
/// days of Month will roll over the date, computing the result by adding 
/// months and days as necessary. The value of the serial number depends on the 
/// current epoch.
///
/// See also: "TIME", "DATEVALUE", 
#[inline]
pub fn date<A: Number, B: Number, C: Number>(year: A, month: B, day: C) -> FnNumber3<A, B, C> {
    FnNumber3("DATE", year, month, day)
}

/// Returns the difference in years, months, or days of two date numbers.
/// Syntax: DATEDIF( StartDate DateParam;; EndDate DateParam;; Format Text; )
///
/// Constraints:
/// None
///
/// Semantics:
/// Compute difference of StartDate and EndDate, in the units given by Format.
/// 
/// The Format is a code from the following table, entered as text, that 
/// specifies the format you want the result of DATEDIF to have.
/// 
/// Table 5 - DATEDIF
/// 
/// Format
/// 
/// Returns the number of
/// 
/// y
/// 
/// Years
/// 
/// m
/// 
/// Months. If there is not a complete month between the dates, 0 will be 
/// returned.
/// 
/// d
/// 
/// Days
/// 
/// md
/// 
/// Days, ignoring months and years
/// 
/// ym
/// 
/// Months, ignoring years
/// 
/// yd
/// 
/// Days, ignoring years
///
/// See also: "DAYS360", "DAYS", "Infix Operator “-”", 
#[inline]
pub fn datedif<A: DateTime, B: DateTime, C: Text>(start_date: A, end_date: B, format: C) -> FnNumber3<A, B, C> {
    FnNumber3("DATEDIF", start_date, end_date, format)
}

/// Returns the date serial number from given text.
/// Syntax: DATEVALUE( D Text; )
///
/// Constraints:
/// None
///
/// Semantics:
/// This computes the serial number of the text string D, using the current 
/// locale. This function shall accept ISO date format (YYYY-MM-DD), which is 
/// locale-independent. It is semantically equal to VALUE(Date), if Date has a 
/// date format, since text matching a date format is automatically converted 
/// to a serial number when used as a Number. If the text of D has a combined 
/// date and time format, e.g. YYYY-MM-DD HH:MM:SS, the integer part of the 
/// date serial number is returned. If the text of D does not have a date or 
/// time format, an evaluator may return an Error. See VALUE for more 
/// information on date formats. The value of the serial number depends on the 
/// current epoch.
///
/// See also: "TIME", "DATE", "TIMEVALUE", "VALUE", 
#[inline]
pub fn datevalue<A: Text>(d: A) -> FnNumber1<A> {
    FnNumber1("DATEVALUE", d)
}

/// Returns the day from a date.
/// Syntax: DAY( D DateParam; )
///
/// Constraints:
/// None
///
/// Semantics:
/// Returns the day portion of D.
///
/// See also: "MONTH", "YEAR", 
#[inline]
pub fn day<A: DateTime>(d: A) -> FnNumber1<A> {
    FnNumber1("DAY", d)
}

/// Returns the number of days between two dates
/// Syntax: DAYS( EndDate DateParam;; StartDate DateParam; )
///
/// Constraints:
/// None
///
/// Semantics:
/// Returns the number of days between two dates. If StartDate and EndDate are 
/// Numbers, this is EndDate – StartDate. If they are both Text, this is 
/// DATEVALUE(StartDate) – DATEVALUE(EndDate).
///
/// See also: "DATEDIF", "DATEVALUE", "DAYS360", "MONTH", "YEAR", "Infix Operator “-”", 
#[inline]
pub fn days<A: DateTime, B: DateTime>(end_date: A, start_date: B) -> FnNumber2<A, B> {
    FnNumber2("DAYS", end_date, start_date)
}

/// Returns the number of days between two dates using the 360-day year
/// Syntax: DAYS360( StartDate DateParam;; EndDate DateParam; )
///
/// Constraints:
/// None
///
/// Semantics:
/// If Method is FALSE, it uses the National Association of Securities Dealers 
/// (NASD) method, also known as the U.S. method. If Method is TRUE, the 
/// European method is used.
/// 
/// The US/NASD method (30US/360):
/// 
/// 1.Truncate date values, set sign = 1.
/// 
/// 2.If StartDate's day-of-month is 31, it is changed to 30.
/// 
/// 3.Otherwise, if StartDate's day-of-month is the last day of February, it is 
/// changed to 30.
/// 
/// 4.If EndDate's day-of-month is 31 and StartDate's day-of-month is 30 (after 
/// having applied a change for #2 or #3, if necessary), EndDate's day-of-month 
/// is changed to 30.
/// 
/// Note 1: This calculation is slightly different from Basis 0 (4.11.7 Basis). 
/// Dates are never swapped.
/// 
/// The European method (30E/360):
/// 
/// 1.Truncate date values, set sign = 1.
/// 
/// 2.If StartDate is after EndDate then swap dates and set sign = -1.
/// 
/// 3.If StartDate's day-of-month is 31, it is changed to 30.
/// 
/// 4.If EndDate's day-of-month is 31, it is changed to 30.
/// 
/// Note 2: Days in February are never changed.
/// 
/// Note 3: This calculation is identical to Basis 4 (4.11.7 Basis)
/// 
/// For both methods the value then returned is
/// sign * ((EndDate.year * 360 + EndDate.month * 30 + EndDate.day) – 
/// (StartDate.year * 360 + StartDate.month * 30 + StartDate.day))
///
/// See also: "DAYS", "DATEDIF", 
#[inline]
pub fn days360<A: DateTime, B: DateTime>(start_date: A, end_date: B) -> FnNumber2<A, B> {
    FnNumber2("DAYS360", start_date, end_date)
}

/// Returns the number of days between two dates using the 360-day year
/// Syntax: DAYS360( StartDate DateParam;; EndDate DateParam;[; Method Logical] )
///
/// Constraints:
/// None
///
/// Semantics:
/// If Method is FALSE, it uses the National Association of Securities Dealers 
/// (NASD) method, also known as the U.S. method. If Method is TRUE, the 
/// European method is used.
/// 
/// The US/NASD method (30US/360):
/// 
/// 1.Truncate date values, set sign = 1.
/// 
/// 2.If StartDate's day-of-month is 31, it is changed to 30.
/// 
/// 3.Otherwise, if StartDate's day-of-month is the last day of February, it is 
/// changed to 30.
/// 
/// 4.If EndDate's day-of-month is 31 and StartDate's day-of-month is 30 (after 
/// having applied a change for #2 or #3, if necessary), EndDate's day-of-month 
/// is changed to 30.
/// 
/// Note 1: This calculation is slightly different from Basis 0 (4.11.7 Basis). 
/// Dates are never swapped.
/// 
/// The European method (30E/360):
/// 
/// 1.Truncate date values, set sign = 1.
/// 
/// 2.If StartDate is after EndDate then swap dates and set sign = -1.
/// 
/// 3.If StartDate's day-of-month is 31, it is changed to 30.
/// 
/// 4.If EndDate's day-of-month is 31, it is changed to 30.
/// 
/// Note 2: Days in February are never changed.
/// 
/// Note 3: This calculation is identical to Basis 4 (4.11.7 Basis)
/// 
/// For both methods the value then returned is
/// sign * ((EndDate.year * 360 + EndDate.month * 30 + EndDate.day) – 
/// (StartDate.year * 360 + StartDate.month * 30 + StartDate.day))
///
/// See also: "DAYS", "DATEDIF", 
#[inline]
pub fn days360_<A: DateTime, B: DateTime>(start_date: A, end_date: B, method: Days360Method) -> FnNumber3<A, B, Days360Method> {
    FnNumber3("DAYS360", start_date, end_date, method)
}

/// Returns the serial number of a given date when MonthAdd months is added
/// Syntax: EDATE( StartDate DateParam;; MonthAdd Number; )
///
/// Constraints:
/// None
///
/// Semantics:
/// First truncate StartDate and MonthAdd, then add MonthAdd number of months. 
/// MonthAdd can be positive, negative, or 0; if zero, the number representing 
/// StartDate (in the current epoch) is returned.
/// 
/// If after adding the given number of months, the day of month in the new 
/// month is larger than the number of days in the given month, the day of 
/// month is adjusted to the last day of the new month. Then the serial number 
/// of that date is returned.
///
/// See also: "DAYS", "DATEDIF", "EOMONTH", 
#[inline]
pub fn edate<A: DateTime, B: Number>(start_date: A, month_add: B) -> FnNumber2<A, B> {
    FnNumber2("EDATE", start_date, month_add)
}

/// Returns the serial number of the end of a month, given date plus MonthAdd 
/// months
/// Syntax: EOMONTH( StartDate DateParam;; MonthAdd Integer; )
///
/// Constraints:
/// None
///
/// Semantics:
/// First truncate StartDate and MonthAdd, then add MonthAdd number of months. 
/// MonthAdd can be positive, negative, or 0. Then return the serial number 
/// representing the end of that month. Due to the semantics of this function, 
/// the value of DAY(StartDate) is irrelevant.
///
/// See also: "DAY", "EDATE", 
#[inline]
pub fn eomonth<A: DateTime, B: Number>(start_date: A, month_add: B) -> FnNumber2<A, B> {
    FnNumber2("EOMONTH", start_date, month_add)
}

/// Extracts the hour (0 through 23) from a time.
/// Syntax: HOUR( T TimeParam; )
///
/// Constraints:
/// None
///
/// Semantics:
/// Extract from T the hour value, 0 through 23, as per a 24-hour clock. This 
/// is equal to:
/// 
/// DayFraction = (T - INT(T))
/// 
/// Hour = INT(DayFraction * 24)
///
/// See also: "MONTH", "DAY", "MINUTE", "SECOND", "INT", 
#[inline]
pub fn hour<A: DateTime>(t: A) -> FnNumber1<A> {
    FnNumber1("HOUR", t)
}

/// Determines the ISO week number of the year for a given date.
/// Syntax: ISOWEEKNUM( D DateParam; )
///
/// Constraints:
/// None
///
/// Semantics:
/// Returns the ordinal number of the [ISO8601] calendar week in the year for 
/// the given date D. ISO 8601 defines the calendar week as a time interval of 
/// seven calendar days starting with a Monday, and the first calendar week of 
/// a year as the one that includes the first Thursday of that year.
///
/// See also: "DAY", "MONTH", "YEAR", "WEEKDAY", "WEEKNUM", 
#[inline]
pub fn isoweeknum<A: DateTime>(d: A) -> FnNumber1<A> {
    FnNumber1("ISOWEEKNUM", d)
}

/// Extracts the minute (0 through 59) from a time.
/// Syntax: MINUTE( T TimeParam; )
///
/// Constraints:
/// None
///
/// Semantics:
/// Extract from T the minute value, 0 through 59, as per a clock. This is 
/// equal to:
/// 
/// DayFraction = (T - INT(T))
/// 
/// HourFraction = (DayFraction * 24 - INT(DayFraction * 24))
/// 
/// Minute = INT(HourFraction * 60)
///
/// See also: "MONTH", "DAY", "HOUR", "SECOND", "INT", 
#[inline]
pub fn minute<A: DateTime>(t: A) -> FnNumber1<A> {
    FnNumber1("MINUTE", t)
}

/// Extracts the month from a date.
/// Syntax: MONTH( Date DateParam; )
///
/// Constraints:
/// None
///
/// Semantics:
/// Takes Date and returns the month portion.
///
/// See also: "YEAR", "DAY", 
#[inline]
pub fn month<A: DateTime>(date: A) -> FnNumber1<A> {
    FnNumber1("MONTH", date)
}

/// Returns the whole number of work days between two dates.
/// Syntax: NETWORKDAYS( Date1 DateParam;; Date2 DateParam; )
///
/// Constraints:
/// None
///
/// Semantics:
/// Returns the whole number of work days between two dates.
/// 
/// Work days are defined as non-weekend, non-holiday days. By default, 
/// weekends are Saturdays and Sundays and there are no holidays.
/// 
/// The optional 3rd parameter Holidays can be used to specify a list of dates 
/// to be treated as holidays. Note that this parameter can be omitted as an 
/// empty parameter (two consecutive ;; semicolons) to be able to pass the set 
/// of Workdays without Holidays.
/// 
/// The optional 4th parameter Workdays can be used to specify a different 
/// definition for the standard work week by passing in a list of numbers which 
/// define which days of the week are workdays (indicated by 0) or not 
/// (indicated by non-zero) in order Sunday, Monday,...,Saturday. So, the 
/// default definition of the work week excludes Saturday and Sunday and is: 
/// {1;0;0;0;0;0;1}. To define the work week as excluding Friday and Saturday, 
/// the third parameter would be: {0;0;0;0;0;1;1}.
#[inline]
pub fn networkdays<A: DateTime, B: DateTime>(date1: A, date2: B) -> FnNumber2<A, B> {
    FnNumber2("NETWORKDAYS", date1, date2)
}

/// Returns the whole number of work days between two dates.
/// Syntax: NETWORKDAYS( Date1 DateParam;; Date2 DateParam;[; Holidays DateSequence] )
///
/// Constraints:
/// None
///
/// Semantics:
/// Returns the whole number of work days between two dates.
/// 
/// Work days are defined as non-weekend, non-holiday days. By default, 
/// weekends are Saturdays and Sundays and there are no holidays.
/// 
/// The optional 3rd parameter Holidays can be used to specify a list of dates 
/// to be treated as holidays. Note that this parameter can be omitted as an 
/// empty parameter (two consecutive ;; semicolons) to be able to pass the set 
/// of Workdays without Holidays.
/// 
/// The optional 4th parameter Workdays can be used to specify a different 
/// definition for the standard work week by passing in a list of numbers which 
/// define which days of the week are workdays (indicated by 0) or not 
/// (indicated by non-zero) in order Sunday, Monday,...,Saturday. So, the 
/// default definition of the work week excludes Saturday and Sunday and is: 
/// {1;0;0;0;0;0;1}. To define the work week as excluding Friday and Saturday, 
/// the third parameter would be: {0;0;0;0;0;1;1}.
#[inline]
pub fn networkdays_<A: DateTime, B: DateTime, C: Sequence>(date1: A, date2: B, holidays: C) -> FnNumber3<A, B, C> {
    FnNumber3("NETWORKDAYS", date1, date2, holidays)
}

/// Returns the whole number of work days between two dates.
/// Syntax: NETWORKDAYS( Date1 DateParam;; Date2 DateParam;[; Holidays DateSequence][; Workdays LogicalSequence] )
///
/// Constraints:
/// None
///
/// Semantics:
/// Returns the whole number of work days between two dates.
/// 
/// Work days are defined as non-weekend, non-holiday days. By default, 
/// weekends are Saturdays and Sundays and there are no holidays.
/// 
/// The optional 3rd parameter Holidays can be used to specify a list of dates 
/// to be treated as holidays. Note that this parameter can be omitted as an 
/// empty parameter (two consecutive ;; semicolons) to be able to pass the set 
/// of Workdays without Holidays.
/// 
/// The optional 4th parameter Workdays can be used to specify a different 
/// definition for the standard work week by passing in a list of numbers which 
/// define which days of the week are workdays (indicated by 0) or not 
/// (indicated by non-zero) in order Sunday, Monday,...,Saturday. So, the 
/// default definition of the work week excludes Saturday and Sunday and is: 
/// {1;0;0;0;0;0;1}. To define the work week as excluding Friday and Saturday, 
/// the third parameter would be: {0;0;0;0;0;1;1}.
#[inline]
pub fn networkdays__<A: DateTime, B: DateTime, C: Sequence, D: Sequence>(date1: A, date2: B, holidays: C, workdays: D) -> FnNumber4<A, B, C, D> {
    FnNumber4("NETWORKDAYS", date1, date2, holidays, workdays)
}

/// Returns the serial number of the current date and time.
/// Syntax: NOW( )
///
/// Constraints:
/// None
///
/// Semantics:
/// This returns the current day and time serial number, using the current 
/// locale. If you want only the serial number of the current day, use TODAY 
/// 6.10.19.
///
/// See also: "DATE", "TIME", "TODAY", 
#[inline]
pub fn now() -> FnNumber0 {
    FnNumber0("NOW", )
}

/// Extracts the second (the integer 0 through 59) from a time. This function 
/// presumes that leap seconds never exist.
/// Syntax: SECOND( T TimeParam; )
///
/// Constraints:
/// None
///
/// Semantics:
/// Extract from T the second value, 0 through 59, as per a clock. Note that 
/// this returns an integer, without a fractional part. Note also that this 
/// rounds to the nearest second, instead of returning the integer part of the 
/// seconds. This is equal to:
/// 
/// DayFraction = (T - INT(T))
/// 
/// HourFraction = (DayFraction * 24 - INT(DayFraction * 24))
/// 
/// MinuteFraction = (HourFraction * 60 - INT(HourFraction * 60))
/// 
/// Second = ROUND(MinuteFraction * 60)
///
/// See also: "MONTH", "DAY", "HOUR", "MINUTE", "INT", 
#[inline]
pub fn second<A: DateTime>(t: A) -> FnNumber1<A> {
    FnNumber1("SECOND", t)
}

/// Constructs a time value from hours, minutes, and seconds.
/// Syntax: TIME( Hours Number;; Minutes Number;; Seconds Number; )
///
/// Constraints:
/// None. Evaluators may first perform INT() on the hour, minute, and second 
/// before doing the calculation.
///
/// Semantics:
/// Returns the fraction of the day consumed by the given time, i.e.:
/// 
/// ((Hours * 60 * 60) + (Minutes * 60) + Seconds) / (24 * 60 * 60)
/// 
/// Time is a subtype of Number, where a time value of 1 = 1 day = 24 hours.
/// 
/// Hours, Minutes, and Seconds may be any number (they shall not be limited to 
/// the ranges 0..24, 0..59, or 0..60 respectively).
///
/// See also: "DATE", "INT", 
#[inline]
pub fn time<A: Number, B: Number, C: Number>(hours: A, minutes: B, seconds: C) -> FnNumber3<A, B, C> {
    FnNumber3("TIME", hours, minutes, seconds)
}

/// Returns a time serial number from given text.
/// Syntax: TIMEVALUE( T Text; )
///
/// Constraints:
/// None
///
/// Semantics:
/// This computes the serial number of the text string T, which is a time, 
/// using the current locale. This function shall accept ISO time format 
/// (HH:MM:SS), which is locale-independent. If the text of T has a combined 
/// date and time format, e.g. YYYY-MM-DD HH:MM:SS, the fractional part of the 
/// date serial number is returned. If the text of T does not have a time 
/// format, an evaluator may attempt to convert the number another way (e.g., 
/// using VALUE), or it may return an Error (this is implementation-dependent).
///
/// See also: "TIME", "DATE", "DATEVALUE", "VALUE", 
#[inline]
pub fn timevalue<A: Text>(t: A) -> FnNumber1<A> {
    FnNumber1("TIMEVALUE", t)
}

/// Returns the serial number of today.
/// Syntax: TODAY( )
///
/// Constraints:
/// None
///
/// Semantics:
/// This returns the current day's serial number, using current locale. This 
/// only returns the date, not the datetime value. For the specific time of day 
/// as well, use NOW 6.10.15.
///
/// See also: "TIME", "NOW", 
#[inline]
pub fn today() -> FnNumber0 {
    FnNumber0("TODAY", )
}

/// Extracts the day of the week from a date; if text, uses current locale to 
/// convert to a date.
/// Syntax: WEEKDAY( D DateParam; )
///
/// Constraints:
/// None
///
/// Semantics:
/// Returns the day of the week from a date D, as a number from 0 through 7. 
/// The exact meaning depends on the value of Type:
/// 
/// 1.When Type is 1, Sunday is the first day of the week, with value 1; 
/// Saturday has value 7.
/// 
/// 2.When Type is 2, Monday is the first day of the week, with value 1; Sunday 
/// has value 7.
/// 
/// 3.When Type is 3, Monday is the first day of the week, with value 0; Sunday 
/// has value 6.
/// 
/// 4.When Type is 11, Monday is the first day of the week, with value 1; 
/// Sunday has value 7.
/// 
/// 5.When Type is 12, Tuesday is the first day of the week, with value 1; 
/// Monday has value 7.
/// 
/// 6.When Type is 13, Wednesday is the first day of the week, with value 1; 
/// Tuesday has value 7.
/// 
/// 7.When Type is 14, Thursday is the first day of the week, with value 1; 
/// Wednesday has value 7.
/// 
/// 8.When Type is 15, Friday is the first day of the week, with value 1; 
/// Thursday has value 7.
/// 
/// 9.When Type is 16, Saturday is the first day of the week, with value 1; 
/// Friday has value 7.
/// 
/// 10. When Type is 17, Sunday is the first day of the week, with value 1; 
/// Saturday has value 7.
/// 
/// Table 6 - WEEKDAY
/// 
/// Weekday Type
/// 
/// 1
/// 
/// 2
/// 
/// 3
/// 
/// 11
/// 
/// 12
/// 
/// 13
/// 
/// 14
/// 
/// 15
/// 
/// 16
/// 
/// 17
/// 
/// Sunday
/// 
/// 1
/// 
/// 7
/// 
/// 6
/// 
/// 7
/// 
/// 6
/// 
/// 5
/// 
/// 4
/// 
/// 3
/// 
/// 2
/// 
/// 1
/// 
/// Monday
/// 
/// 2
/// 
/// 1
/// 
/// 0
/// 
/// 1
/// 
/// 7
/// 
/// 6
/// 
/// 5
/// 
/// 4
/// 
/// 3
/// 
/// 2
/// 
/// Tuesday
/// 
/// 3
/// 
/// 2
/// 
/// 1
/// 
/// 2
/// 
/// 1
/// 
/// 7
/// 
/// 6
/// 
/// 5
/// 
/// 4
/// 
/// 3
/// 
/// Wednesday
/// 
/// 4
/// 
/// 3
/// 
/// 2
/// 
/// 3
/// 
/// 2
/// 
/// 1
/// 
/// 7
/// 
/// 6
/// 
/// 5
/// 
/// 4
/// 
/// Thursday
/// 
/// 5
/// 
/// 4
/// 
/// 3
/// 
/// 4
/// 
/// 3
/// 
/// 2
/// 
/// 1
/// 
/// 7
/// 
/// 6
/// 
/// 5
/// 
/// Friday
/// 
/// 6
/// 
/// 5
/// 
/// 4
/// 
/// 5
/// 
/// 4
/// 
/// 3
/// 
/// 2
/// 
/// 1
/// 
/// 7
/// 
/// 6
/// 
/// Saturday
/// 
/// 7
/// 
/// 6
/// 
/// 5
/// 
/// 6
/// 
/// 5
/// 
/// 4
/// 
/// 3
/// 
/// 2
/// 
/// 1
/// 
/// 7
///
/// See also: "DAY", "MONTH", "YEAR", 
#[inline]
pub fn weekday<A: DateTime>(d: A) -> FnNumber1<A> {
    FnNumber1("WEEKDAY", d)
}

/// Extracts the day of the week from a date; if text, uses current locale to 
/// convert to a date.
/// Syntax: WEEKDAY( D DateParam;[; Type Integer] )
///
/// Constraints:
/// None
///
/// Semantics:
/// Returns the day of the week from a date D, as a number from 0 through 7. 
/// The exact meaning depends on the value of Type:
/// 
/// 1.When Type is 1, Sunday is the first day of the week, with value 1; 
/// Saturday has value 7.
/// 
/// 2.When Type is 2, Monday is the first day of the week, with value 1; Sunday 
/// has value 7.
/// 
/// 3.When Type is 3, Monday is the first day of the week, with value 0; Sunday 
/// has value 6.
/// 
/// 4.When Type is 11, Monday is the first day of the week, with value 1; 
/// Sunday has value 7.
/// 
/// 5.When Type is 12, Tuesday is the first day of the week, with value 1; 
/// Monday has value 7.
/// 
/// 6.When Type is 13, Wednesday is the first day of the week, with value 1; 
/// Tuesday has value 7.
/// 
/// 7.When Type is 14, Thursday is the first day of the week, with value 1; 
/// Wednesday has value 7.
/// 
/// 8.When Type is 15, Friday is the first day of the week, with value 1; 
/// Thursday has value 7.
/// 
/// 9.When Type is 16, Saturday is the first day of the week, with value 1; 
/// Friday has value 7.
/// 
/// 10. When Type is 17, Sunday is the first day of the week, with value 1; 
/// Saturday has value 7.
/// 
/// Table 6 - WEEKDAY
/// 
/// Weekday Type
/// 
/// 1
/// 
/// 2
/// 
/// 3
/// 
/// 11
/// 
/// 12
/// 
/// 13
/// 
/// 14
/// 
/// 15
/// 
/// 16
/// 
/// 17
/// 
/// Sunday
/// 
/// 1
/// 
/// 7
/// 
/// 6
/// 
/// 7
/// 
/// 6
/// 
/// 5
/// 
/// 4
/// 
/// 3
/// 
/// 2
/// 
/// 1
/// 
/// Monday
/// 
/// 2
/// 
/// 1
/// 
/// 0
/// 
/// 1
/// 
/// 7
/// 
/// 6
/// 
/// 5
/// 
/// 4
/// 
/// 3
/// 
/// 2
/// 
/// Tuesday
/// 
/// 3
/// 
/// 2
/// 
/// 1
/// 
/// 2
/// 
/// 1
/// 
/// 7
/// 
/// 6
/// 
/// 5
/// 
/// 4
/// 
/// 3
/// 
/// Wednesday
/// 
/// 4
/// 
/// 3
/// 
/// 2
/// 
/// 3
/// 
/// 2
/// 
/// 1
/// 
/// 7
/// 
/// 6
/// 
/// 5
/// 
/// 4
/// 
/// Thursday
/// 
/// 5
/// 
/// 4
/// 
/// 3
/// 
/// 4
/// 
/// 3
/// 
/// 2
/// 
/// 1
/// 
/// 7
/// 
/// 6
/// 
/// 5
/// 
/// Friday
/// 
/// 6
/// 
/// 5
/// 
/// 4
/// 
/// 5
/// 
/// 4
/// 
/// 3
/// 
/// 2
/// 
/// 1
/// 
/// 7
/// 
/// 6
/// 
/// Saturday
/// 
/// 7
/// 
/// 6
/// 
/// 5
/// 
/// 6
/// 
/// 5
/// 
/// 4
/// 
/// 3
/// 
/// 2
/// 
/// 1
/// 
/// 7
///
/// See also: "DAY", "MONTH", "YEAR", 
#[inline]
pub fn weekday_<A: DateTime>(d: A, type_: WeekdayMethod) -> FnNumber2<A, WeekdayMethod> {
    FnNumber2("WEEKDAY", d, type_)
}

/// Determines the week number of the year for a given date.
/// Syntax: WEEKNUM( D DateParam; )
///
/// Constraints:
/// 1 ≤ Mode ≤ 2, or 11 ≤ Mode ≤ 17, or Mode = 21, or Mode = 150
///
/// Semantics:
/// Returns the number of the week in the year for the given date.
/// 
/// For Mode = {1, 2, 11, 12, ..., 17} the week containing January 1 is the 
/// first week of the year, and is numbered week 1. The week starts on {Sunday, 
/// Monday, Monday, Tuesday, ..., Sunday}.
/// 
/// Mode 21 and Mode 150 are both [ISO8601], the week starts on Monday and the 
/// week containing the first Thursday of the year is the first week of the 
/// year, and is numbered week 1.
///
/// See also: "DAY", "MONTH", "YEAR", "WEEKDAY", "ISOWEEKNUM", 
#[inline]
pub fn weeknum<A: DateTime>(d: A) -> FnNumber1<A> {
    FnNumber1("WEEKNUM", d)
}

/// Determines the week number of the year for a given date.
/// Syntax: WEEKNUM( D DateParam;[; Mode Number] )
///
/// Constraints:
/// 1 ≤ Mode ≤ 2, or 11 ≤ Mode ≤ 17, or Mode = 21, or Mode = 150
///
/// Semantics:
/// Returns the number of the week in the year for the given date.
/// 
/// For Mode = {1, 2, 11, 12, ..., 17} the week containing January 1 is the 
/// first week of the year, and is numbered week 1. The week starts on {Sunday, 
/// Monday, Monday, Tuesday, ..., Sunday}.
/// 
/// Mode 21 and Mode 150 are both [ISO8601], the week starts on Monday and the 
/// week containing the first Thursday of the year is the first week of the 
/// year, and is numbered week 1.
///
/// See also: "DAY", "MONTH", "YEAR", "WEEKDAY", "ISOWEEKNUM", 
#[inline]
pub fn weeknum_<A: DateTime>(d: A, mode: WeeknumMethod) -> FnNumber2<A, WeeknumMethod> {
    FnNumber2("WEEKNUM", d, mode)
}

/// Returns the date serial number which is a specified number of work days 
/// before or after an input date.
/// Syntax: WORKDAY( Date DateParam;; Offset Number; )
///
/// Constraints:
/// None
///
/// Semantics:
/// Returns the date serial number for the day that is offset from the input 
/// Date parameter by the number of work days specified in the Offset 
/// parameter. If Offset is negative, the offset will return a date prior to 
/// Date. If Offset is positive, a date later Date is returned. If Offset is 
/// zero, then Date is returned.
/// 
/// Work days are defined as non-weekend, non-holiday days. By default, 
/// weekends are Saturdays and Sundays and there are no holidays.
/// 
/// The optional 3rd parameter Holidays can be used to specify a list of dates 
/// to be treated as holidays. Note that this parameter can be omitted as an 
/// empty parameter (two consecutive ;; semicolons) to be able to pass the set 
/// of Workdays without Holidays.
/// 
/// The optional 4th parameter Workdays can be used to specify a different 
/// definition for the standard work week by passing in a list of numbers which 
/// define which days of the week are workdays (indicated by 0) or not 
/// (indicated by non-zero) in order Sunday, Monday,...,Saturday. If all seven 
/// numbers in Workdays are non-zero and Offset is also non-zero, WORKDAY 
/// returns an error.
///
/// Note:
/// The default definition of the work week that excludes Saturday and Sunday 
/// and is: {1;0;0;0;0;0;1}. To define the workweek as excluding Friday and 
/// Saturday, the third parameter would be: {0;0;0;0;0;1;1}.
#[inline]
pub fn workday<A: DateTime, B: Number>(date: A, offset: B) -> FnNumber2<A, B> {
    FnNumber2("WORKDAY", date, offset)
}

/// Returns the date serial number which is a specified number of work days 
/// before or after an input date.
/// Syntax: WORKDAY( Date DateParam;; Offset Number;[; Holidays DateSequence] )
///
/// Constraints:
/// None
///
/// Semantics:
/// Returns the date serial number for the day that is offset from the input 
/// Date parameter by the number of work days specified in the Offset 
/// parameter. If Offset is negative, the offset will return a date prior to 
/// Date. If Offset is positive, a date later Date is returned. If Offset is 
/// zero, then Date is returned.
/// 
/// Work days are defined as non-weekend, non-holiday days. By default, 
/// weekends are Saturdays and Sundays and there are no holidays.
/// 
/// The optional 3rd parameter Holidays can be used to specify a list of dates 
/// to be treated as holidays. Note that this parameter can be omitted as an 
/// empty parameter (two consecutive ;; semicolons) to be able to pass the set 
/// of Workdays without Holidays.
/// 
/// The optional 4th parameter Workdays can be used to specify a different 
/// definition for the standard work week by passing in a list of numbers which 
/// define which days of the week are workdays (indicated by 0) or not 
/// (indicated by non-zero) in order Sunday, Monday,...,Saturday. If all seven 
/// numbers in Workdays are non-zero and Offset is also non-zero, WORKDAY 
/// returns an error.
///
/// Note:
/// The default definition of the work week that excludes Saturday and Sunday 
/// and is: {1;0;0;0;0;0;1}. To define the workweek as excluding Friday and 
/// Saturday, the third parameter would be: {0;0;0;0;0;1;1}.
#[inline]
pub fn workday_<A: DateTime, B: Number, C: Sequence>(date: A, offset: B, holidays: C) -> FnNumber3<A, B, C> {
    FnNumber3("WORKDAY", date, offset, holidays)
}

/// Returns the date serial number which is a specified number of work days 
/// before or after an input date.
/// Syntax: WORKDAY( Date DateParam;; Offset Number;[; Holidays DateSequence][; Workdays LogicalSequence] )
///
/// Constraints:
/// None
///
/// Semantics:
/// Returns the date serial number for the day that is offset from the input 
/// Date parameter by the number of work days specified in the Offset 
/// parameter. If Offset is negative, the offset will return a date prior to 
/// Date. If Offset is positive, a date later Date is returned. If Offset is 
/// zero, then Date is returned.
/// 
/// Work days are defined as non-weekend, non-holiday days. By default, 
/// weekends are Saturdays and Sundays and there are no holidays.
/// 
/// The optional 3rd parameter Holidays can be used to specify a list of dates 
/// to be treated as holidays. Note that this parameter can be omitted as an 
/// empty parameter (two consecutive ;; semicolons) to be able to pass the set 
/// of Workdays without Holidays.
/// 
/// The optional 4th parameter Workdays can be used to specify a different 
/// definition for the standard work week by passing in a list of numbers which 
/// define which days of the week are workdays (indicated by 0) or not 
/// (indicated by non-zero) in order Sunday, Monday,...,Saturday. If all seven 
/// numbers in Workdays are non-zero and Offset is also non-zero, WORKDAY 
/// returns an error.
///
/// Note:
/// The default definition of the work week that excludes Saturday and Sunday 
/// and is: {1;0;0;0;0;0;1}. To define the workweek as excluding Friday and 
/// Saturday, the third parameter would be: {0;0;0;0;0;1;1}.
#[inline]
pub fn workday__<A: DateTime, B: Number, C: Sequence, D: Sequence>(date: A, offset: B, holidays: C, workdays: D) -> FnNumber4<A, B, C, D> {
    FnNumber4("WORKDAY", date, offset, holidays, workdays)
}

/// Extracts the year from a date given in the current locale of the evaluator.
/// Syntax: YEAR( D DateParam; )
///
/// Constraints:
/// None
///
/// Semantics:
/// Parses a date-formatted string in the current locale's format and returns 
/// the year portion.
/// 
/// If a year is given as a two-digit number, as in "05-21-15", then the year 
/// returned is either 1915 or 2015, depending upon the break point in the 
/// calculation context. In an OpenDocument document, this break point is 
/// determined by HOST-NULL-YEAR.
/// 
/// Evaluators shall support extracting the year from a date beginning in 1900. 
/// Three-digit year numbers precede adoption of the Gregorian calendar, and 
/// may return either an Error or the year number. Four-digit year numbers 
/// preceding 1582 (inception of the Gregorian Calendar) may return either an 
/// Error or the year number. Four-digit year numbers following 1582 should 
/// return the year number.
///
/// See also: "MONTH", "DAY", "VALUE", 
#[inline]
pub fn year<A: DateTime>(d: A) -> FnNumber1<A> {
    FnNumber1("YEAR", d)
}

/// Extracts the number of years (including fractional part) between two dates
/// Syntax: YEARFRAC( StartDate DateParam;; EndDate DateParam; )
///
/// Constraints:
/// None
///
/// Semantics:
/// Computes the fraction of the number of years between a StartDate and 
/// EndDate.
/// 
/// B indicates the day-count convention to use in the calculation. 4.11.7
///
/// See also: "DATEDIF", 
#[inline]
pub fn yearfrac<A: DateTime, B: DateTime>(start_date: A, end_date: B) -> FnNumber2<A, B> {
    FnNumber2("YEARFRAC", start_date, end_date)
}

/// Extracts the number of years (including fractional part) between two dates
/// Syntax: YEARFRAC( StartDate DateParam;; EndDate DateParam;[; B Basis] )
///
/// Constraints:
/// None
///
/// Semantics:
/// Computes the fraction of the number of years between a StartDate and 
/// EndDate.
/// 
/// B indicates the day-count convention to use in the calculation. 4.11.7
///
/// See also: "DATEDIF", 
#[inline]
pub fn yearfrac_<A: DateTime, B: DateTime>(start_date: A, end_date: B, b: YearFracMethod) -> FnNumber3<A, B, YearFracMethod> {
    FnNumber3("YEARFRAC", start_date, end_date, b)
}
