//!
//! Mathematical functions.
//!

pub use crate::generated::math::*;

use crate::{Any, Criterion, FnNumberVar, Reference};
use std::fmt::Write;

/// Parameter for CONVERT().
pub enum BaseUnit {
    UKAcre,
    USAcre,
    SqAngstrom,
    Ar,
    SqFoot,
    Hectare,
    SqInch,
    SqMeter,
    Morgen,
    SqMile,
    SqNauticalMile,
    SqPica,
    SqYard,
    Angstrom,
    Ell,
    Foot,
    Inch,
    LightYear,
    Meter,
    Mile,
    NauticalMile,
    Parsec,
    Pica,
    SurveyMile,
    Yard,
    BTU,
    ThermCalorie,
    ITCalorie,
    Erg,
    ElectronVolt,
    Flb,
    HPh,
    Joule,
    WattHour,
    Dyne,
    Newton,
    Lbf,
    Pond,
    Bit,
    Byte,
    Gauss,
    Tesla,
    Gram,
    Grain,
    Cwt,
    UKCwt,
    Lbm,
    Stone,
    Ton,
    Ozm,
    Sg,
    AtomicMassUnit,
    UKTon,
    HP,
    PS,
    Watt,
    Atm,
    MmHg,
    Pascal,
    Psi,
    Torr,
    AdmiralityKnot,
    Knot,
    MetersPerHour,
    MetersPerSecond,
    MilesPerHour,
    Celsius,
    Fahrenheit,
    Kelvin,
    Rankine,
    Reaumur,
    Day,
    Hour,
    Minute,
    Second,
    Year,
    CbAngstrom,
    Barrel,
    Bushel,
    Cup,
    CbFoot,
    Gallon,
    GRT,
    CbInch,
    Liter,
    CbLightYear,
    CbMeter,
    CbMile,
    MTon,
    CbNauticalMile,
    FluidOunce,
    CbPica,
    Pint,
    Quart,
    Tablespoon,
    Teaspoon,
    ModernTeaspoon,
    UKGallon,
    UKPint,
    UKQuart,
    CbYard,
}

impl Any for BaseUnit {
    fn formula(&self, buf: &mut String) {
        buf.push_str(match self {
            BaseUnit::UKAcre => "uk_acre",
            BaseUnit::USAcre => "us_acre",
            BaseUnit::SqAngstrom => "ang2",
            BaseUnit::Ar => "ar",
            BaseUnit::SqFoot => "ft2",
            BaseUnit::Hectare => "ha",
            BaseUnit::SqInch => "in2",
            BaseUnit::SqMeter => "ly2",
            BaseUnit::Morgen => "Morgen",
            BaseUnit::SqMile => "mi2",
            BaseUnit::SqNauticalMile => "Nmi2",
            BaseUnit::SqPica => "Pica2",
            BaseUnit::SqYard => "yd2",
            BaseUnit::Angstrom => "ang",
            BaseUnit::Ell => "ell",
            BaseUnit::Foot => "ft",
            BaseUnit::Inch => "in",
            BaseUnit::LightYear => "ly",
            BaseUnit::Meter => "m",
            BaseUnit::Mile => "mi",
            BaseUnit::NauticalMile => "Nmi",
            BaseUnit::Parsec => "pc",
            BaseUnit::Pica => "Pica",
            BaseUnit::SurveyMile => "survey_mi",
            BaseUnit::Yard => "yd",
            BaseUnit::BTU => "BTU",
            BaseUnit::ThermCalorie => "c",
            BaseUnit::ITCalorie => "cal",
            BaseUnit::Erg => "e",
            BaseUnit::ElectronVolt => "eV",
            BaseUnit::Flb => "flb",
            BaseUnit::HPh => "HPh",
            BaseUnit::Joule => "J",
            BaseUnit::WattHour => "Wh",
            BaseUnit::Dyne => "dyn",
            BaseUnit::Newton => "N",
            BaseUnit::Lbf => "lbf",
            BaseUnit::Pond => "pond",
            BaseUnit::Bit => "bit",
            BaseUnit::Byte => "byte",
            BaseUnit::Gauss => "ga",
            BaseUnit::Tesla => "T",
            BaseUnit::Gram => "g",
            BaseUnit::Grain => "grain",
            BaseUnit::Cwt => "cwt",
            BaseUnit::UKCwt => "uk_cwt",
            BaseUnit::Lbm => "lbm",
            BaseUnit::Stone => "stone",
            BaseUnit::Ton => "ton",
            BaseUnit::Ozm => "ozm",
            BaseUnit::Sg => "sg",
            BaseUnit::AtomicMassUnit => "u",
            BaseUnit::UKTon => "uk_ton",
            BaseUnit::HP => "HP",
            BaseUnit::PS => "PS",
            BaseUnit::Watt => "W",
            BaseUnit::Atm => "atm",
            BaseUnit::MmHg => "mmHg",
            BaseUnit::Pascal => "Pa",
            BaseUnit::Psi => "psi",
            BaseUnit::Torr => "Torr",
            BaseUnit::AdmiralityKnot => "admkn",
            BaseUnit::Knot => "kn",
            BaseUnit::MetersPerHour => "m/h",
            BaseUnit::MetersPerSecond => "m/s",
            BaseUnit::MilesPerHour => "mph",
            BaseUnit::Celsius => "C",
            BaseUnit::Fahrenheit => "F",
            BaseUnit::Kelvin => "K",
            BaseUnit::Rankine => "Rank",
            BaseUnit::Reaumur => "Reau",
            BaseUnit::Day => "day",
            BaseUnit::Hour => "hr",
            BaseUnit::Minute => "min",
            BaseUnit::Second => "sec",
            BaseUnit::Year => "yr",
            BaseUnit::CbAngstrom => "ang3",
            BaseUnit::Barrel => "barrel",
            BaseUnit::Bushel => "bushel",
            BaseUnit::Cup => "cup",
            BaseUnit::CbFoot => "ft3",
            BaseUnit::Gallon => "gal",
            BaseUnit::GRT => "GRT",
            BaseUnit::CbInch => "in3",
            BaseUnit::Liter => "l",
            BaseUnit::CbLightYear => "ly3",
            BaseUnit::CbMeter => "m3",
            BaseUnit::CbMile => "mi3",
            BaseUnit::MTon => "MTON",
            BaseUnit::CbNauticalMile => "Mmi3",
            BaseUnit::FluidOunce => "oz",
            BaseUnit::CbPica => "Pica3",
            BaseUnit::Pint => "pt",
            BaseUnit::Quart => "qt",
            BaseUnit::Tablespoon => "tbs",
            BaseUnit::Teaspoon => "tsp",
            BaseUnit::ModernTeaspoon => "tspm",
            BaseUnit::UKGallon => "uk_gal",
            BaseUnit::UKPint => "gu_pt",
            BaseUnit::UKQuart => "uk_qt",
            BaseUnit::CbYard => "yd3",
        });
    }
}

/// Parameter for CONVERT().
pub enum DecimalPrefix {
    Yotta,
    Zetta,
    Exa,
    Peta,
    Tera,
    Giga,
    Mega,
    Kilo,
    Hecto,
    Deka,
    Deci,
    Centi,
    Milli,
    Micro,
    Nano,
    Pico,
    Femto,
    Atto,
    Zepto,
}

impl Any for DecimalPrefix {
    fn formula(&self, buf: &mut String) {
        buf.push_str(match self {
            DecimalPrefix::Yotta => "Y",
            DecimalPrefix::Zetta => "Z",
            DecimalPrefix::Exa => "E",
            DecimalPrefix::Peta => "P",
            DecimalPrefix::Tera => "T",
            DecimalPrefix::Giga => "G",
            DecimalPrefix::Mega => "M",
            DecimalPrefix::Kilo => "k",
            DecimalPrefix::Hecto => "h",
            DecimalPrefix::Deka => "da",
            DecimalPrefix::Deci => "d",
            DecimalPrefix::Centi => "c",
            DecimalPrefix::Milli => "m",
            DecimalPrefix::Micro => "u",
            DecimalPrefix::Nano => "n",
            DecimalPrefix::Pico => "p",
            DecimalPrefix::Femto => "f",
            DecimalPrefix::Atto => "a",
            DecimalPrefix::Zepto => "z",
        });
    }
}

/// Parameter for CONVERT().
pub enum BinaryPrefix {
    Yobi,
    Zebi,
    Exbi,
    Pebi,
    Tebi,
    Gibi,
    Mebi,
    Kibi,
}

impl Any for BinaryPrefix {
    fn formula(&self, buf: &mut String) {
        buf.push_str(match self {
            BinaryPrefix::Yobi => "Yi",
            BinaryPrefix::Zebi => "Zi",
            BinaryPrefix::Exbi => "Ei",
            BinaryPrefix::Pebi => "Pi",
            BinaryPrefix::Tebi => "Ti",
            BinaryPrefix::Gibi => "Gi",
            BinaryPrefix::Mebi => "Mi",
            BinaryPrefix::Kibi => "Ki",
        });
    }
}

/// Parameter for CONVERT().
pub enum ConvertUnit {
    Unit(BaseUnit),
    SI(DecimalPrefix, BaseUnit),
    Bin(BinaryPrefix, BaseUnit),
}

impl Any for ConvertUnit {
    fn formula(&self, buf: &mut String) {
        match self {
            ConvertUnit::Unit(v) => {
                v.formula(buf);
            }
            ConvertUnit::SI(p, v) => {
                p.formula(buf);
                v.formula(buf);
            }
            ConvertUnit::Bin(p, v) => {
                p.formula(buf);
                v.formula(buf);
            }
        }
    }
}

/// Parameter for SUBTOTAL().
pub enum SubtotalFunction {
    Average,
    Count,
    CountA,
    Max,
    Min,
    Product,
    StDev,
    StDevP,
    Sum,
    Var,
    VarP,
    AverageExclCollapsed,
    CountExclCollapsed,
    CountAExclCollapsed,
    MaxExclCollapsed,
    MinExclCollapsed,
    ProductExclCollapsed,
    StDevExclCollapsed,
    StDevPExclCollapsed,
    SumExclCollapsed,
    VarExclCollapsed,
    VarPExclCollapsed,
}

impl Any for SubtotalFunction {
    fn formula(&self, buf: &mut String) {
        let _ = write!(
            buf,
            "{}",
            match self {
                SubtotalFunction::Average => 1,
                SubtotalFunction::Count => 2,
                SubtotalFunction::CountA => 3,
                SubtotalFunction::Max => 4,
                SubtotalFunction::Min => 5,
                SubtotalFunction::Product => 6,
                SubtotalFunction::StDev => 7,
                SubtotalFunction::StDevP => 8,
                SubtotalFunction::Sum => 9,
                SubtotalFunction::Var => 10,
                SubtotalFunction::VarP => 11,
                SubtotalFunction::AverageExclCollapsed => 101,
                SubtotalFunction::CountExclCollapsed => 102,
                SubtotalFunction::CountAExclCollapsed => 103,
                SubtotalFunction::MaxExclCollapsed => 104,
                SubtotalFunction::MinExclCollapsed => 105,
                SubtotalFunction::ProductExclCollapsed => 106,
                SubtotalFunction::StDevExclCollapsed => 107,
                SubtotalFunction::StDevPExclCollapsed => 108,
                SubtotalFunction::SumExclCollapsed => 109,
                SubtotalFunction::VarExclCollapsed => 110,
                SubtotalFunction::VarPExclCollapsed => 111,
            }
        );
    }
}

#[inline]
pub fn sumifs<
    A: Reference + 'static,
    R: Reference + 'static,
    C: Criterion + 'static,
    const N: usize,
>(
    range: A,
    list: [(R, C); N],
) -> FnNumberVar {
    let mut param = Vec::new();

    param.push(Box::new(range) as Box<dyn Any>);
    for (r, c) in list {
        param.push(Box::new(r) as Box<dyn Any>);
        param.push(Box::new(c) as Box<dyn Any>);
    }

    FnNumberVar("SUMIFS", param)
}
