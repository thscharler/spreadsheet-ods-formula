use crate::{FnLogical0, Param};

/// Method for DATEDIF()
pub enum DateDifMethod {
    /// Years
    Years,
    /// Months
    Months,
    /// Days
    Days,
    /// Days, ignoring months and years.
    DaysIgnoreMonthsYears,
    /// Months, ignoring years.
    MonthsIgnoreYears,
    /// Days, ignoring years.
    DaysIgnoreYears,
}

impl Param for DateDifMethod {
    type Type = &'static str;

    fn as_param(&self) -> Self::Type {
        match self {
            DateDifMethod::Years => "y",
            DateDifMethod::Months => "m",
            DateDifMethod::Days => "d",
            DateDifMethod::DaysIgnoreMonthsYears => "md",
            DateDifMethod::MonthsIgnoreYears => "ym",
            DateDifMethod::DaysIgnoreYears => "yd",
        }
    }
}

/// Method for DAYS360()
pub enum Days360Method {
    /// NASD Method
    USNasd,
    /// European Method
    Europe,
}

impl Param for Days360Method {
    type Type = FnLogical0;

    fn as_param(&self) -> Self::Type {
        match self {
            Days360Method::USNasd => FnLogical0("FALSE()"),
            Days360Method::Europe => FnLogical0("TRUE()"),
        }
    }
}

/// Method for WEEKDAY()
pub enum WeekdayMethod {
    /// Monday first, value 0.
    Monday0,
    /// Monday first, value 1.
    Monday1,
    /// Tuesday first, value 1.
    Tuesday1,
    /// Wednesday first, value 1.
    Wednesday1,
    /// Thursday first, value 1.
    Thursday1,
    /// Friday first, value 1.
    Friday1,
    /// Saturday first, value 1.
    Saturday1,
    /// Sunday first, value 1.
    Sunday1,
}

impl Param for WeekdayMethod {
    type Type = &'static str;

    fn as_param(&self) -> Self::Type {
        match self {
            WeekdayMethod::Monday0 => "3",
            WeekdayMethod::Monday1 => "11",
            WeekdayMethod::Tuesday1 => "12",
            WeekdayMethod::Wednesday1 => "13",
            WeekdayMethod::Thursday1 => "14",
            WeekdayMethod::Friday1 => "15",
            WeekdayMethod::Saturday1 => "16",
            WeekdayMethod::Sunday1 => "17",
        }
    }
}

/// Method for WEEKNUM()
pub enum WeeknumMethod {
    /// First week contains Jan 1, week starts on Monday.
    Jan1WeekMonday,
    /// First week contains Jan 1, week starts on Tuesday.
    Jan1WeekTuesday,
    /// First week contains Jan 1, week starts on Wednesday.
    Jan1WeekWednesday,
    /// First week contains Jan 1, week starts on Thursday.
    Jan1WeekThursday,
    /// First week contains Jan 1, week starts on Friday.
    Jan1WeekFriday,
    /// First week contains Jan 1, week starts on Saturday.
    Jan1WeekSaturday,
    /// First week contains Jan 1, week starts on Sunday.
    Jan1WeekSunday,
    /// ISO week number.
    ISOWeeknum,
}

impl Param for WeeknumMethod {
    type Type = &'static str;

    fn as_param(&self) -> Self::Type {
        match self {
            WeeknumMethod::Jan1WeekMonday => "11",
            WeeknumMethod::Jan1WeekTuesday => "12",
            WeeknumMethod::Jan1WeekWednesday => "13",
            WeeknumMethod::Jan1WeekThursday => "14",
            WeeknumMethod::Jan1WeekFriday => "15",
            WeeknumMethod::Jan1WeekSaturday => "16",
            WeeknumMethod::Jan1WeekSunday => "17",
            WeeknumMethod::ISOWeeknum => "21",
        }
    }
}

/// Method for YEARFRAC()
pub enum YearFracMethod {
    ///
    USNasd30_360,
    ///
    ActualActual,
    ///
    Actual360,
    ///
    Actual365,
    ///
    European30_360,
}

impl Param for YearFracMethod {
    type Type = &'static str;

    fn as_param(&self) -> Self::Type {
        match self {
            YearFracMethod::USNasd30_360 => "0",
            YearFracMethod::ActualActual => "1",
            YearFracMethod::Actual360 => "2",
            YearFracMethod::Actual365 => "3",
            YearFracMethod::European30_360 => "4",
        }
    }
}