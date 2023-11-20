//! 
//! This section describes functions for various mathematical functions, 
//! including trigonometric functions like SIN 6.16.55). Note that the 
//! constraint text presumes that a value of type Number is a real number (no 
//! imaginary component). Unless noted otherwise, all angle measurements are in 
//! radians.

use crate::*;
#[allow(unused_imports)]
use crate::math::*;

/// Return the absolute (nonnegative) value.
/// Syntax: ABS( N Number; )
///
/// Constraints:
/// None
///
/// Semantics:
/// If N < 0, returns -N, otherwise returns N.
///
/// See also: "Prefix Operator “-”", 
#[inline]
pub fn abs<A: Number>(n: A) -> FnNumber1<A> {
    FnNumber1("ABS", n)
}

/// Returns the principal value of the arc cosine of a number. The angle is 
/// returned in radians.
/// Syntax: ACOS( N Number; )
///
/// Constraints:
/// -1.0 ≤ N ≤ 1.0.
///
/// Semantics:
/// Computes the arc cosine of a number, in radians.
/// 
/// Returns a principal value 0 ≤ result ≤ π.
///
/// See also: "COS", "RADIANS", "DEGREES", 
#[inline]
pub fn acos<A: Number>(n: A) -> FnNumber1<A> {
    FnNumber1("ACOS", n)
}

/// Return the principal value of the inverse hyperbolic cosine.
/// Syntax: ACOSH( N Number; )
///
/// Constraints:
/// N ≥ 1
///
/// Semantics:
/// Computes the principal value of the inverse hyperbolic cosine.
///
/// See also: "COSH", "ASINH", 
#[inline]
pub fn acosh<A: Number>(n: A) -> FnNumber1<A> {
    FnNumber1("ACOSH", n)
}

/// Return the principal value of the arc cotangent of a number. The angle is 
/// returned in radians.
/// Syntax: ACOT( N Number; )
///
/// Semantics:
/// Computes the arc cotangent of a number, in radians.
/// 
/// Returns a principal value 0 < result < π.
///
/// See also: "COT", "ATAN", "TAN", "RADIANS", "DEGREES", 
#[inline]
pub fn acot<A: Number>(n: A) -> FnNumber1<A> {
    FnNumber1("ACOT", n)
}

/// Return the hyperbolic arc cotangent
/// Syntax: ACOTH( N Number; )
///
/// Constraints:
/// ABS(N) > 1
///
/// Semantics:
/// Computes the hyperbolic arc cotangent. The hyperbolic arc cotangent is an 
/// analog of the ordinary (circular) arc cotangent.
///
/// See also: "COSH", "ASINH", 
#[inline]
pub fn acoth<A: Number>(n: A) -> FnNumber1<A> {
    FnNumber1("ACOTH", n)
}

/// Return the principal value of the arc sine of a number. The angle is 
/// returned in radians.
/// Syntax: ASIN( N Number; )
///
/// Constraints:
/// -1 ≤ N ≤ 1.
///
/// Semantics:
/// Computes the arc sine of a number, in radians.
/// 
/// Returns a principal value -π/2 ≤ result ≤ π/2.
///
/// See also: "SIN", "RADIANS", "DEGREES", 
#[inline]
pub fn asin<A: Number>(n: A) -> FnNumber1<A> {
    FnNumber1("ASIN", n)
}

/// Return the principal value of the inverse hyperbolic sine
/// Syntax: ASINH( N Number; )
///
/// Constraints:
/// None
///
/// Semantics:
/// Computes the principal value of the inverse hyperbolic sine.
///
/// See also: "SINH", "ACOSH", 
#[inline]
pub fn asinh<A: Number>(n: A) -> FnNumber1<A> {
    FnNumber1("ASINH", n)
}

/// Return the principal value of the arc tangent of a number. The angle is 
/// returned in radians.
/// Syntax: ATAN( N Number; )
///
/// Semantics:
/// Computes the arc tangent of a number, in radians.
/// 
/// Returns a principal value -π/2 < result < π/2.
///
/// See also: "ATAN2", "TAN", "RADIANS", "DEGREES", 
#[inline]
pub fn atan<A: Number>(n: A) -> FnNumber1<A> {
    FnNumber1("ATAN", n)
}

/// Returns the principal value of the arc tangent given a coordinate of two 
/// numbers.
/// 
/// The angle is returned in radians.
/// Syntax: ATAN2( x Number;; y Number; )
///
/// Constraints:
/// x ≠ 0 or y ≠ 0
///
/// Semantics:
/// Computes the arc tangent of two numbers (the x and y coordinates of a 
/// point), in radians. This is similar to ATAN(y/x), but the signs of the two 
/// numbers are taken into account so that the result covers the full range 
/// from -π to +π. ATAN2(0;0) is implementation-defined, evaluators may 
/// return 0 or an Error.
/// 
/// Returns a principal value -π < result ≤ π.
///
/// See also: "ATAN", "TAN", "RADIANS", "DEGREES", 
#[inline]
pub fn atan2<A: Number, B: Number>(x: A, y: B) -> FnNumber2<A, B> {
    FnNumber2("ATAN2", x, y)
}

/// Return the principal value of the inverse hyperbolic tangent
/// Syntax: ATANH( N Number; )
///
/// Constraints:
/// -1 < N < 1
///
/// Semantics:
/// Computes the principal value of the inverse hyperbolic tangent.
///
/// See also: "COSH", "SINH", "ASINH", "ACOSH", "ATAN", "ATAN2", "FISHER", 
#[inline]
pub fn atanh<A: Number>(n: A) -> FnNumber1<A> {
    FnNumber1("ATANH", n)
}

/// Returns the modified Bessel function of integer order In(X).
/// Syntax: BESSELI( X Number;; N Number; )
///
/// Constraints:
/// N ≥ 0, INT(N) = N; Evaluators may evaluate expressions where N ≥ 0 
/// returns a non-error value.
///
/// Semantics:
/// Computes the modified Bessel function of integer order In(X). N is also 
/// known as the order.
///
/// See also: "BESSELJ", "BESSELK", "BESSELY", "INT", 
#[inline]
pub fn besseli<A: Number, B: Number>(x: A, n: B) -> FnNumber2<A, B> {
    FnNumber2("BESSELI", x, n)
}

/// Returns the Bessel function of integer order Jn(X) (cylinder function)
/// Syntax: BESSELJ( X Number;; N Number; )
///
/// Constraints:
/// N ≥ 0, INT(N) = N; Evaluators may evaluate expressions where N ≥ 0 
/// returns a non-error value.
///
/// Semantics:
/// Computes the Bessel function of integer order Jn(X). N is also known as the 
/// order.
///
/// See also: "BESSELI", "BESSELK", "BESSELY", "INT", 
#[inline]
pub fn besselj<A: Number, B: Number>(x: A, n: B) -> FnNumber2<A, B> {
    FnNumber2("BESSELJ", x, n)
}

/// Returns the modified Bessel function of integer order Kn(x).
/// Syntax: BESSELK( X Number;; N Number; )
///
/// Constraints:
/// X ≠ 0, N ≥ 0, INT(N) = N; Evaluators may evaluate expressions where N 
/// ≥ 0 returns a non-error value.
///
/// Semantics:
/// Computes the Bessel function of integer order Kn(x). N is also known as the 
/// order.
///
/// See also: "BESSELI", "BESSELJ", "BESSELY", "INT", 
#[inline]
pub fn besselk<A: Number, B: Number>(x: A, n: B) -> FnNumber2<A, B> {
    FnNumber2("BESSELK", x, n)
}

/// Returns the Bessel function of integer order Yn(X), also known as the 
/// Neumann function.
/// Syntax: BESSELY( X Number;; N Number; )
///
/// Constraints:
/// X ≠ 0, N ≥ 0, INT(N) = N; Evaluators may evaluate expressions where N 
/// ≥ 0 returns a non-error value.
///
/// Semantics:
/// Computes Bessel function of integer order Yn(X), also known as the Neumann 
/// function. N is also known as the order.
///
/// See also: "BESSELI", "BESSELJ", "BESSELK", "INT", 
#[inline]
pub fn bessely<A: Number, B: Number>(x: A, n: B) -> FnNumber2<A, B> {
    FnNumber2("BESSELY", x, n)
}

/// Returns the number of different R-length sets that can be selected from N 
/// items.
/// Syntax: COMBIN( N Integer;; R Integer; )
///
/// Constraints:
/// N ≥ 0, R ≥ 0, R ≤ N
///
/// Semantics:
/// COMBIN returns the binomial coefficient, which is the number of different 
/// R-length sets that can be selected from N items. Since they are sets, order 
/// in the sets is not relevant. The parameters are truncated (using INT) 
/// before use. For example, if a jar contains five marbles, each one a 
/// distinct color, the number of different three-marble groups COMBIN(5;3) = 
/// 10. The result is
/// 
/// Note that if order is important, use PERMUT instead.
///
/// See also: "INT", "PERMUT", 
#[inline]
pub fn combin<A: Number, B: Number>(n: A, r: B) -> FnNumber2<A, B> {
    FnNumber2("COMBIN", n, r)
}

/// Returns the number of combinations with repetitions.
/// Syntax: COMBINA( N Integer;; M Integer; )
///
/// Constraints:
/// N ≥ 0, M ≥ 0, N ≥ M; Evaluators may evaluate expressions where N ≥ 
/// 0, M ≥ 0 returns a non-error value.
///
/// Semantics:
/// Returns the number of possible combinations of M objects out of N possible 
/// ones, with repetitions allowed. Actual arguments that are not integers are 
/// truncated (using INT) before use. The result is
///
/// See also: "COMBIN", 
#[inline]
pub fn combina<A: Number, B: Number>(n: A, m: B) -> FnNumber2<A, B> {
    FnNumber2("COMBINA", n, m)
}

/// Returns a number converted from one unit system into another.
/// Syntax: CONVERT( N Number;; From Text;; Into Text; )
///
/// Constraints:
/// From and Into shall be legal units, and shall be in the same unit group.
///
/// Semantics:
/// Returns the number converted from the unit identified by From into the unit 
/// identified by Into. A unit is a unit symbol , optionally preceded by a unit 
/// prefix (either a decimal prefix or a binary prefix, as specified in Table 
/// 25 - Decimal Prefixes for use in CONVERT and Table 26 - Binary prefixes for 
/// use in CONVERT respectively). Units (including both the unit symbol and the 
/// optional unit prefix) are case-sensitive.
/// 
/// Evaluators claiming to implement this function shall support at least the 
/// following unit symbols (with conversions between them and other units in 
/// the same group):
/// 
/// Table 24 - Unit names
/// 
/// Unit group
/// 
/// Unit symbol
/// 
/// Description
/// 
/// Area
/// 
/// "uk_acre"
/// 
/// International acre (using international feet), exactly 4046.8564224 m2; 
/// normally not used for U.S. land areas
/// 
/// "us_acre"
/// 
/// U.S. survey/statute acre (using U.S. survey/statute feet), exactly 
/// 4046+13525426/15499969 m 2
/// 
/// "ang2" or "ang^2" *
/// 
/// Square angstrom (an Angstrom is exactly 10-10 m)
/// 
/// "ar" *
/// 
/// are, 100 m2 (not abbreviated as “a”)
/// 
/// "ft2" or "ft^2"
/// 
/// Square international feet (1 foot is exactly 0.3048 m)
/// 
/// "ha"
/// 
/// hectare, 10000 m2
/// 
/// "in2" or "in^2"
/// 
/// Square international inches (1 inch is exactly 2.54 cm)
/// 
/// "ly2" or "ly^2"
/// 
/// Square light-year (where year=365.25 days)
/// 
/// "m2" or "m^2" *
/// 
/// Square meters
/// 
/// "Morgen"
/// 
/// Morgen, 2500 m2
/// 
/// "mi2" or "mi^2"
/// 
/// Square international miles
/// 
/// "Nmi2" or "Nmi^2"
/// 
/// Square nautical miles (1 nautical mile is 1852 m)
/// 
/// "Pica2" or "Pica^2" or "picapt2" or "picapt^2"
/// 
/// Square Pica Point (one Pica point is 1/72 inch)
/// 
/// "pica2" or
/// 
/// "pica^2"
/// 
/// Square Pica (one Pica is 1/6 inch)
/// 
/// "yd2" or "yd^2"
/// 
/// Square international yards (1 yard is 0.9144 m)
/// 
/// Distance (Length)
/// 
/// "ang" *
/// 
/// Angstrom, exactly 10-10 m
/// 
/// "ell"
/// 
/// Ell, exactly 45 international inches
/// 
/// "ft"
/// 
/// International Foot, exactly 0.3048 m and also exactly 12 international 
/// inches.
/// 
/// "in"
/// 
/// International Inch, exactly 2.54 cm.
/// 
/// "ly" *
/// 
/// Light-year, (299792458 m/s) (3600 s/hr) (24 hr/day) (365.25 day)
/// 
/// "m" *
/// 
/// Meter
/// 
/// "mi"
/// 
/// International Mile, exactly 1609.344 m and exactly 5280 international feet. 
/// This is not a U.S. survey/statute mile (see “survey_mi”) nor a nautical 
/// mile (see “Nmi”), but this is the mile normally used in the U.S. 
/// customary system
/// 
/// "Nmi"
/// 
/// International nautical mile, exactly 1852 m. Note that this is not the 
/// obsolete U.S. nautical mile nor the Admiralty mile.
/// 
/// "parsec" or "pc" *
/// 
/// Distance from sun to a point having heliocentric parallax of one second 
/// (used for stellar distance), AU/tan(1/3600 degree) where an AU is exactly 
/// 149,597,870.691 kilometers. *
/// 
/// "Pica" or "picapt"
/// 
/// Pica point (1/72 inch)
/// 
/// "pica"
/// 
/// Pica (1/6 inch)
/// 
/// "survey_mi"
/// 
/// U.S. survey "mile, aka U.S. statute mile, exactly 6336000/3937 m; used in 
/// some U.S. maps. This is not the mile (see “mi”) normally used in the 
/// U.S.
/// 
/// "yd"
/// 
/// International yard, exactly 0.9144 m and exactly 3 international feet.
/// 
/// Energy
/// 
/// "BTU" ("btu")
/// 
/// International Table British Thermal Unit
/// 
/// "c" *
/// 
/// Thermodynamic calorie, 4.184 J. This is not a dietary Calorie 
/// (kilocalorie). For high accuracy, use Joule, due to the many conflicting 
/// definitions of calorie.
/// 
/// "cal" *
/// 
/// International Table (IT) calorie, 4.1868 J. This is not a dietary Calorie 
/// (kilocalorie). For high accuracy, use Joule, due to the many conflicting 
/// definitions of calorie.
/// 
/// "e" *
/// 
/// Erg
/// 
/// "eV" ("ev") *
/// 
/// Electron volt (eV preferred)
/// 
/// "flb"
/// 
/// Foot-pound (international foot, avoirdupois pound)
/// 
/// "HPh" ("hh")
/// 
/// Horsepower-hour (HPh preferred)
/// 
/// "J" *
/// 
/// Joule
/// 
/// "Wh" ("wh") *
/// 
/// Watt-hour
/// 
/// Force (Weight)
/// 
/// "dyn" ("dy") *
/// 
/// Dyne
/// 
/// "N" *
/// 
/// Newton
/// 
/// "lbf"
/// 
/// Pound force (see “lbm” for pound mass)
/// 
/// "pond" *
/// 
/// Pond, gravitational force on a mass of one gram, 9.80665E-3 N.
/// 
/// Information
/// 
/// "bit" * †
/// 
/// bit
/// 
/// "byte" * †
/// 
/// byte = 8 bits
/// 
/// Magnetic Flux Density
/// 
/// "ga" *
/// 
/// Gauss
/// 
/// "T" *
/// 
/// Tesla
/// 
/// Mass
/// 
/// "g" *
/// 
/// Gram
/// 
/// "grain"
/// 
/// Grain, 1/7000 international pound mass (avoirdupois) (U.S. usage).
/// 
/// "cwt" ("shweight")
/// 
/// U.S. (short) hundredweight, 100 lbm
/// 
/// "uk_cwt" or "lcwt" ("hweight")
/// 
/// Imperial hundredweight, aka long hundredweight; 112 lbm
/// 
/// "lbm"
/// 
/// International pound mass (avoirdupois), exactly 453.59237 g (see “lbf” 
/// for pound force)
/// 
/// "stone"
/// 
/// 14 international pound mass (avoirdupois)
/// 
/// "ton"
/// 
/// 2000 international pound mass (avoirdupois) (U.S. usage). Note that there 
/// are many other measures also called “ton”; in particular, this is not a 
/// metric ton (tonne).
/// 
/// "ozm"
/// 
/// Ounce mass (avoirdupois), exactly 1/16 of an international pound mass 
/// (avoirdupois) (see “oz” for fluid ounce)
/// 
/// "sg"
/// 
/// Slug; 32.174 international pound mass (avoirdupois)
/// 
/// "u" *
/// 
/// U (atomic mass unit)
/// 
/// "uk_ton" or "LTON" ("brton")
/// 
/// Imperial ton, aka “long ton”, "deadweight ton", or "weight ton". 2240 
/// lbm.
/// 
/// Power
/// 
/// "HP" ("h")
/// 
/// Mechanical horsepower aka Imperial horsepower. 550 foot-pounds per second. 
/// The unit “h” is deprecated and should be replaced with “HP”.
/// 
/// "PS"
/// 
/// Pferdestärke (German “horse strength”, close but not identical to 
/// “HP”), the amount of power to lift a mass of 75 kilograms in one second 
/// against the earth gravitation between a distance of one meter, 
/// approximately 735.49875 W.
/// 
/// "W" ("w") *
/// 
/// Watt
/// 
/// Pressure
/// 
/// "atm" ("at") *
/// 
/// Atmosphere
/// 
/// "mmHg" *
/// 
/// mm of Mercury
/// 
/// "Pa" *
/// 
/// Pascal
///
/// Note:
/// “P” or “p” in user input as abbreviations for Pascal may be 
/// accepted by implementations, but should be stored as “Pa”..
/// 
/// "psi"
/// 
/// Pounds per square inch, using avoirdupois pounds and international inches.
/// 
/// "Torr"
/// 
/// Torr, exactly 101325/760 Pa (this is close but not equal to mmHg)
/// 
/// Speed
/// 
/// "admkn"
/// 
/// Admiralty knot, exactly 6080 international feet/hour.
/// 
/// "kn"
/// 
/// Knot, exactly one Nautical mile per hour or exactly 1852/3600 m/s. Note 
/// that this is not an Admiralty knot (“admkn”).
/// 
/// "m/h" or "m/hr" *
/// 
/// Meters per hour
/// 
/// "m/s" or "m/sec" *
/// 
/// Meters per second
/// 
/// "mph"
/// 
/// Miles per hour (international miles)
/// 
/// Temperature
/// 
/// "C" ("cel")
/// 
/// degrees Celsius
/// 
/// "F" ("fah")
/// 
/// degrees Fahrenheit
/// 
/// "K" ("kel") *
/// 
/// Kelvin
/// 
/// "Rank"
/// 
/// degrees Rankine
/// 
/// "Reau"
/// 
/// degrees Réaumur; °C = °Ré · 5/4.
/// 
/// Time
/// 
/// "day" or "d"
/// 
/// Day (exactly 24 hours)
/// 
/// "hr"
/// 
/// Hour (exactly 60 minutes)
/// 
/// "mn" or "min"
/// 
/// Minute (exactly 60 seconds)
/// 
/// "sec" or "s" *
/// 
/// Second (“s” is the official abbreviation of this SI base unit, while 
/// “sec” is a widely-recognized abbreviation in the CONVERT function) *
/// 
/// "yr"
/// 
/// Year (exactly 365.25 days, for purposes of this function)
/// 
/// Volume
/// 
/// "ang3" or "ang^3" *
/// 
/// Cubic angstrom
/// 
/// "barrel"
/// 
/// U.S. oil barrel, exactly 42 U.S. customary gallons (liquid). Note that many 
/// other units are also called barrels (e.g., a beer barrel in the U.K. is 36 
/// Imperial gallons)
/// 
/// "bushel"
/// 
/// U.S. bushel (not Imperial bushel), interpreted as volume
/// 
/// "cup"
/// 
/// Cup (U.S. customary liquid measure)
/// 
/// "ft3" or "ft^3"
/// 
/// Cubic international feet
/// 
/// "gal"
/// 
/// Gallon (U.S. customary liquid measure), 3.785411784 liters.
/// 
/// "GRT" ("regton")
/// 
/// Gross Registered Ton, 100 cubic (international) feet
/// 
/// "in3" or "in^3"
/// 
/// Cubic international inch
/// 
/// "l" or "L" ("lt") *
/// 
/// Liter
/// 
/// "ly3" or "ly^3"
/// 
/// Cubic light-year
/// 
/// "m3" or "m^3" *
/// 
/// Cubic meter
/// 
/// "mi3" or "mi^3"
/// 
/// Cubic international mile
/// 
/// "MTON"
/// 
/// Measurement ton aka “freight ton”, 40 cubic feet
/// 
/// "Nmi3" or "Nmi^3"
/// 
/// Cubic nautical mile
/// 
/// "oz"
/// 
/// Fluid ounce (U.S. customary liquid measure; see “ozm” for ounce mass)
/// 
/// "Pica3" or "Pica^3"
/// 
/// "picapt3" or
/// 
/// "picapt^3"
/// 
/// Cubic Pica Point (one Pica point is 1/72 inch)
/// 
/// "pica3" or
/// 
/// "pica^3"
/// 
/// Cubic Pica (one Pica is 1/6 inch)
/// 
/// "pt" or "us_pt"
/// 
/// U.S. Pint (liquid measure)
/// 
/// "qt"
/// 
/// Quart (U.S. customary liquid measure). This is 0.946352946 liters, and thus 
/// not the same as the U.S. dry quart (1.101220 liters), nor is this the same 
/// as the Imperial quart (as used in the U.K. and Canada, which is 1.1365225 
/// liters exactly)
/// 
/// "tbs"
/// 
/// Tablespoon (U.S. customary, traditional meaning). This shall be 0.5 U.S. 
/// fluid ounce, not 15mL (common in U.S.) or 20mL (common in Australia).
/// 
/// "tsp"
/// 
/// Teaspoon (U.S. customary, traditional meaning), 1/6 fluid ounce in U.S. 
/// customary measure. This is not the 1/8 Imperial fl. oz. per Imperial units 
/// nor the modern teaspoon of 5 mL currently used in the U.S.; see “tspm”
/// 
/// "tspm"
/// 
/// Modern teaspoon, 5mL
/// 
/// "uk_gal"
/// 
/// U.K. / Imperial gallon, 4.54609 liters.
/// 
/// "uk_pt"
/// 
/// U.K. / Imperial pint,1/8 of a UK gallon.
/// 
/// "uk_qt"
/// 
/// U.K. / Imperial quart,1/4 of a UK gallon.
/// 
/// "yd3" or "yd^3"
/// 
/// Cubic international yard
/// 
/// If a conversion factor (as listed above) is not exact, an implementation 
/// may use a more accurate conversion factor instead.
/// 
/// Implementation-defined unit names should contain a 'FULL STOP' (U+002E) 
/// character.
/// 
/// Evaluators shall support decimal prefixes for unit symbols marked with * 
/// and binary prefixes for unit symbols marked with †. Evaluators should not 
/// support prefixes for other unit symbols.
/// 
/// The unit symbols in parentheses are deprecated unit symbols; evaluators 
/// shall support these unit symbols.
/// 
/// Evaluators should use internationally-standardized unit name abbreviations 
/// for such additions where possible. Evaluators may support the obsolete 
/// symbols “p” and “P” as unit names for Pascals.
/// 
/// For purposes of this function, a year is exactly 365.25 days long.
/// 
/// Evaluators claiming to support this function shall permit the unit decimal 
/// prefixes specified in Table 25 - Decimal Prefixes for use in CONVERT to be 
/// prepended to any unit symbol marked with * in Table 24 - Unit names. Adding 
/// a unit prefix indicates multiplication of the (scalar) unit by the given 
/// prefix value; for example km indicates kilometres, and km2 or km^2 indicate 
/// square kilometres.
/// 
/// Table 25 - Decimal Prefixes for use in CONVERT
/// 
/// Unit Prefix
/// 
/// Description
/// 
/// Prefix Value
/// 
/// "Y"
/// 
/// yotta
/// 
/// 1E+24
/// 
/// "Z"
/// 
/// zetta
/// 
/// 1E+21
/// 
/// "E"
/// 
/// exa
/// 
/// 1E+18
/// 
/// "P"
/// 
/// peta
/// 
/// 1E+15
/// 
/// "T"
/// 
/// tera
/// 
/// 1E+12
/// 
/// "G"
/// 
/// giga
/// 
/// 1E+09
/// 
/// "M"
/// 
/// mega
/// 
/// 1E+06
/// 
/// "k"
/// 
/// kilo
/// 
/// 1E+03
/// 
/// "h"
/// 
/// hecto
/// 
/// 1E+02
/// 
/// "da" or "e"
/// 
/// deka (
/// 
/// Note: “e” is not a standard SI prefix
/// 
/// 1E+01
/// 
/// "d"
/// 
/// deci
/// 
/// 1E-01
/// 
/// "c"
/// 
/// centi
/// 
/// 1E-02
/// 
/// "m"
/// 
/// milli
/// 
/// 1E-03
/// 
/// "u"
/// 
/// micro
/// 
/// Note: this is “u”, not the standard SI µ
/// 
/// 1E-06
/// 
/// "n"
/// 
/// nano
/// 
/// 1E-09
/// 
/// "p"
/// 
/// pico
/// 
/// 1E-12
/// 
/// "f"
/// 
/// femto
/// 
/// 1E-15
/// 
/// "a"
/// 
/// atto
/// 
/// 1E-18
/// 
/// "z"
/// 
/// zepto
/// 
/// 1E-21
/// 
/// "y"
/// 
/// yocto
/// 
/// 1E-24
/// 
/// Note: The prefix “e” for 10 1 is nonstandard and included for backward 
/// compatibility with legacy applications and documents.
/// 
/// The unit names marked with † in Table 24 - Unit names (see the 
/// Information Unit group) shall also support the following binary prefixes 
/// per IEC 60027-2:
/// 
/// Table 26 - Binary prefixes for use in CONVERT
/// 
/// Binary Unit Prefix
/// 
/// Description
/// 
/// Prefix Value
/// 
/// Derived from
/// 
/// "Yi"
/// 
/// yobi
/// 
/// 2^80 = 1 208 925 819 614 629 174 706 176
/// 
/// yotta
/// 
/// "Zi"
/// 
/// zebi
/// 
/// 2^70 = 1 180 591 620 717 411 303 424
/// 
/// zetta
/// 
/// "Ei"
/// 
/// exbi
/// 
/// 2^60 = 1 152 921 504 606 846 976
/// 
/// exa
/// 
/// "Pi"
/// 
/// pebi
/// 
/// 2^50 = 1 125 899 906 842 624
/// 
/// peta
/// 
/// "Ti"
/// 
/// tebi
/// 
/// 2^40 = 1 099 511 627 776
/// 
/// tera
/// 
/// "Gi"
/// 
/// gibi
/// 
/// 2^30 = 1 073 741 824
/// 
/// giga
/// 
/// "Mi"
/// 
/// mebi
/// 
/// 2^20 = 1 048 576
/// 
/// mega
/// 
/// "Ki"
/// 
/// kibi
/// 
/// 2^10 = 1024
/// 
/// kilo
/// 
/// In the case where there is a naming conflict (a unit name with a prefix is 
/// the same as an unprefixed name), the unprefixed name shall take precedence.
/// 
/// Evaluators may implement this conversion by first converting to some SI 
/// unit (e.g., meter and kilogram), and then convert again to the final unit.
///
/// See also: "EUROCONVERT", 
#[inline]
pub fn convert<A: Number, B: Text, C: Text>(n: A, from: B, into: C) -> FnNumber3<A, B, C> {
    FnNumber3("CONVERT", n, from, into)
}

/// Return the cosine of an angle specified in radians.
/// Syntax: COS( N Number; )
///
/// Constraints:
/// None
///
/// Semantics:
/// Computes the cosine of an angle specified in radians.
///
/// See also: "ACOS", "RADIANS", "DEGREES", 
#[inline]
pub fn cos<A: Number>(n: A) -> FnNumber1<A> {
    FnNumber1("COS", n)
}

/// Return the hyperbolic cosine of the given hyperbolic angle.
/// Syntax: COSH( N Number; )
///
/// Constraints:
/// None
///
/// Semantics:
/// Computes the hyperbolic cosine of a hyperbolic angle. The hyperbolic cosine 
/// is an analog of the ordinary (circular) cosine. The points (cosh t, sinh t) 
/// define the right half of the equilateral hyperbola, just as the points (cos 
/// t, sin t) define the points of a circle.
///
/// See also: "ACOSH", "SINH", "TANH", 
#[inline]
pub fn cosh<A: Number>(n: A) -> FnNumber1<A> {
    FnNumber1("COSH", n)
}

/// Return the cotangent of an angle specified in radians.
/// Syntax: COT( N Number; )
///
/// Constraints:
/// None
///
/// Semantics:
/// Computes the cotangent of an angle specified in radians.
/// 
/// COT(x) = 1 / TAN(x)
///
/// See also: "ACOT", "TAN", "RADIANS", "DEGREES", "SIN", "COS", 
#[inline]
pub fn cot<A: Number>(n: A) -> FnNumber1<A> {
    FnNumber1("COT", n)
}

/// Return the hyperbolic cotangent of the given hyperbolic angle.
/// Syntax: COTH( N Number; )
///
/// Constraints:
/// N ≠ 0
///
/// Semantics:
/// Computes the hyperbolic cotangent of a hyperbolic angle. The hyperbolic 
/// cotangent is an analog of the ordinary (circular) cotangent.
///
/// See also: "ACOSH", "COSH", "SINH", "TANH", 
#[inline]
pub fn coth<A: Number>(n: A) -> FnNumber1<A> {
    FnNumber1("COTH", n)
}

/// Return the cosecant of an angle specified in radians.
/// Syntax: CSC( N Number; )
///
/// Constraints:
/// None
///
/// Semantics:
/// Computes the cosecant cosine of an angle specified in radians. Equivalent 
/// to:
/// 
/// 1 / SIN(N)
///
/// See also: "SIN", 
#[inline]
pub fn csc<A: Number>(n: A) -> FnNumber1<A> {
    FnNumber1("CSC", n)
}

/// Return the hyperbolic cosecant of the given angle specified in radians.
/// Syntax: CSCH( N Number; )
///
/// Constraints:
/// None
///
/// Semantics:
/// Computes the hyperbolic cosecant of a hyperbolic angle. This is equivalent 
/// to:
/// 
/// 1 / SINH(N)
///
/// See also: "SINH", 
#[inline]
pub fn csch<A: Number>(n: A) -> FnNumber1<A> {
    FnNumber1("CSCH", n)
}

/// Convert radians to degrees.
/// Syntax: DEGREES( N Number; )
///
/// Constraints:
/// None
///
/// Semantics:
/// Converts a number in radians into a number in degrees. DEGREES(N) is equal 
/// to N * 180 / π.
///
/// See also: "RADIANS", "PI", 
#[inline]
pub fn degrees<A: Number>(n: A) -> FnNumber1<A> {
    FnNumber1("DEGREES", n)
}

/// Report if two numbers are equal, returns 1 if they are equal.
/// Syntax: DELTA( X Number; )
///
/// Constraints:
/// None
///
/// Semantics:
/// If X and Y are equal, return 1, else 0. Y is set to 0 if omitted.
///
/// See also: "Infix operator “=”", 
#[inline]
pub fn delta<A: Number>(x: A) -> FnNumber1<A> {
    FnNumber1("DELTA", x)
}

/// Report if two numbers are equal, returns 1 if they are equal.
/// Syntax: DELTA( X Number;[; Y Number] )
///
/// Constraints:
/// None
///
/// Semantics:
/// If X and Y are equal, return 1, else 0. Y is set to 0 if omitted.
///
/// See also: "Infix operator “=”", 
#[inline]
pub fn delta_<A: Number, B: Number>(x: A, y: B) -> FnNumber2<A, B> {
    FnNumber2("DELTA", x, y)
}

/// Calculates the error function.
/// Syntax: ERF( Z0 Number; )
///
/// Constraints:
/// None
///
/// Semantics:
/// With a single argument, returns the error function of Z0:
/// 
/// With two arguments, returns
///
/// See also: "ERFC", 
#[inline]
pub fn erf<A: Number>(z0: A) -> FnNumber1<A> {
    FnNumber1("ERF", z0)
}

/// Calculates the error function.
/// Syntax: ERF( Z0 Number;[; Z1 Number] )
///
/// Constraints:
/// None
///
/// Semantics:
/// With a single argument, returns the error function of Z0:
/// 
/// With two arguments, returns
///
/// See also: "ERFC", 
#[inline]
pub fn erf_<A: Number, B: Number>(z0: A, z1: B) -> FnNumber2<A, B> {
    FnNumber2("ERF", z0, z1)
}

/// Calculates the complementary error function.
/// Syntax: ERFC( Z Number; )
///
/// Constraints:
/// None
///
/// Semantics:
/// returns the complementary error function of Z: ERFC(Z) = 1 – ERF(Z)
///
/// See also: "ERF", 
#[inline]
pub fn erfc<A: Number>(z: A) -> FnNumber1<A> {
    FnNumber1("ERFC", z)
}

/// Converts a Number, representing a value in one European currency, to an 
/// equivalent value in another European currency, according to the fixed 
/// conversion rates defined by the Council of the European Union.
/// Syntax: EUROCONVERT( N Number;; From Text;; To Text; )
///
/// Constraints:
/// From and To shall be known to the evaluator. TriangulationPrecision shall 
/// be ≥ 3, if not omitted.
/// 
/// If an evaluator does not support the parameters FullPrecision and 
/// TriangulationPrecision, FullPrecision should be assumed to be false.
///
/// Semantics:
/// Returns the given money value of a conversion from From currency into To 
/// currency. Both From and To shall be the official [ISO4217] abbreviation for 
/// the given currency; note that these are in upper case, but the function 
/// accepts lower case or mixed case as well. If From and To are equal 
/// currencies, the value N is returned, no precision or triangulation is 
/// applied.
/// 
/// As new member countries adopt the Euro, new conversion rates will become 
/// active and evaluators may add them using the respective [ISO4217] codes and 
/// fixed rates as defined by the European Council, on the basis of a European 
/// Commission proposal.
///
/// Note:
/// 
/// The European Commission's Euro entry page is http://ec.europa.eu/euro/
/// The conversion rates and triangulation rules are available at 
/// http://ec.europa.eu/economy_finance/euro/adoption/conversion/index_en.htm 
/// with links to the European Council Regulation legal documents at the 
/// http://eur-lex.europa.eu/ European Union law database server.
/// 
/// If FullPrecision is omitted or FALSE, the result is rounded according to 
/// the decimals of the To currency. If FullPrecision is TRUE the result is not 
/// rounded.
/// 
/// If TriangulationPrecision is given and ≥ 3, the intermediate result of a 
/// triangular conversion (currency1,EUR,currency2) is rounded to that 
/// precision. If TriangulationPrecision is omitted, the intermediate result is 
/// not rounded. Also if To currency is “EUR”, TriangulationPrecision 
/// precision is used as if triangulation was needed and conversion from EUR to 
/// EUR was applied.
///
/// See also: "CONVERT", 
#[inline]
pub fn euroconvert<A: Number, B: Text, C: Text>(n: A, from: B, to: C) -> FnNumber3<A, B, C> {
    FnNumber3("EUROCONVERT", n, from, to)
}

/// Converts a Number, representing a value in one European currency, to an 
/// equivalent value in another European currency, according to the fixed 
/// conversion rates defined by the Council of the European Union.
/// Syntax: EUROCONVERT( N Number;; From Text;; To Text;[; FullPrecision Logical] )
///
/// Constraints:
/// From and To shall be known to the evaluator. TriangulationPrecision shall 
/// be ≥ 3, if not omitted.
/// 
/// If an evaluator does not support the parameters FullPrecision and 
/// TriangulationPrecision, FullPrecision should be assumed to be false.
///
/// Semantics:
/// Returns the given money value of a conversion from From currency into To 
/// currency. Both From and To shall be the official [ISO4217] abbreviation for 
/// the given currency; note that these are in upper case, but the function 
/// accepts lower case or mixed case as well. If From and To are equal 
/// currencies, the value N is returned, no precision or triangulation is 
/// applied.
/// 
/// As new member countries adopt the Euro, new conversion rates will become 
/// active and evaluators may add them using the respective [ISO4217] codes and 
/// fixed rates as defined by the European Council, on the basis of a European 
/// Commission proposal.
///
/// Note:
/// 
/// The European Commission's Euro entry page is http://ec.europa.eu/euro/
/// The conversion rates and triangulation rules are available at 
/// http://ec.europa.eu/economy_finance/euro/adoption/conversion/index_en.htm 
/// with links to the European Council Regulation legal documents at the 
/// http://eur-lex.europa.eu/ European Union law database server.
/// 
/// If FullPrecision is omitted or FALSE, the result is rounded according to 
/// the decimals of the To currency. If FullPrecision is TRUE the result is not 
/// rounded.
/// 
/// If TriangulationPrecision is given and ≥ 3, the intermediate result of a 
/// triangular conversion (currency1,EUR,currency2) is rounded to that 
/// precision. If TriangulationPrecision is omitted, the intermediate result is 
/// not rounded. Also if To currency is “EUR”, TriangulationPrecision 
/// precision is used as if triangulation was needed and conversion from EUR to 
/// EUR was applied.
///
/// See also: "CONVERT", 
#[inline]
pub fn euroconvert_<A: Number, B: Text, C: Text, D: Logical>(n: A, from: B, to: C, full_precision: D) -> FnNumber4<A, B, C, D> {
    FnNumber4("EUROCONVERT", n, from, to, full_precision)
}

/// Converts a Number, representing a value in one European currency, to an 
/// equivalent value in another European currency, according to the fixed 
/// conversion rates defined by the Council of the European Union.
/// Syntax: EUROCONVERT( N Number;; From Text;; To Text;[; FullPrecision Logical][; TriangulationPrecision Integer] )
///
/// Constraints:
/// From and To shall be known to the evaluator. TriangulationPrecision shall 
/// be ≥ 3, if not omitted.
/// 
/// If an evaluator does not support the parameters FullPrecision and 
/// TriangulationPrecision, FullPrecision should be assumed to be false.
///
/// Semantics:
/// Returns the given money value of a conversion from From currency into To 
/// currency. Both From and To shall be the official [ISO4217] abbreviation for 
/// the given currency; note that these are in upper case, but the function 
/// accepts lower case or mixed case as well. If From and To are equal 
/// currencies, the value N is returned, no precision or triangulation is 
/// applied.
/// 
/// As new member countries adopt the Euro, new conversion rates will become 
/// active and evaluators may add them using the respective [ISO4217] codes and 
/// fixed rates as defined by the European Council, on the basis of a European 
/// Commission proposal.
///
/// Note:
/// 
/// The European Commission's Euro entry page is http://ec.europa.eu/euro/
/// The conversion rates and triangulation rules are available at 
/// http://ec.europa.eu/economy_finance/euro/adoption/conversion/index_en.htm 
/// with links to the European Council Regulation legal documents at the 
/// http://eur-lex.europa.eu/ European Union law database server.
/// 
/// If FullPrecision is omitted or FALSE, the result is rounded according to 
/// the decimals of the To currency. If FullPrecision is TRUE the result is not 
/// rounded.
/// 
/// If TriangulationPrecision is given and ≥ 3, the intermediate result of a 
/// triangular conversion (currency1,EUR,currency2) is rounded to that 
/// precision. If TriangulationPrecision is omitted, the intermediate result is 
/// not rounded. Also if To currency is “EUR”, TriangulationPrecision 
/// precision is used as if triangulation was needed and conversion from EUR to 
/// EUR was applied.
///
/// See also: "CONVERT", 
#[inline]
pub fn euroconvert__<A: Number, B: Text, C: Text, D: Logical, E: Number>(n: A, from: B, to: C, full_precision: D, triangulation_precision: E) -> FnNumber5<A, B, C, D, E> {
    FnNumber5("EUROCONVERT", n, from, to, full_precision, triangulation_precision)
}

/// Rounds a number up to the nearest even integer. Rounding is away from zero.
/// Syntax: EVEN( N Number; )
///
/// Constraints:
/// None
///
/// Semantics:
/// Returns the even integer whose sign is the same as N's and whose absolute 
/// value is greater than or equal to the absolute value of N.
///
/// See also: "ODD", 
#[inline]
pub fn even<A: Number>(n: A) -> FnNumber1<A> {
    FnNumber1("EVEN", n)
}

/// Returns e raised by the given number.
/// Syntax: EXP( X Number; )
///
/// Constraints:
/// None
///
/// Semantics:
/// Computes
///
/// See also: "LOG", "LN", 
#[inline]
pub fn exp<A: Number>(x: A) -> FnNumber1<A> {
    FnNumber1("EXP", x)
}

/// Return factorial (!).
/// Syntax: FACT( F Integer; )
///
/// Constraints:
/// F ≥ 0
///
/// Semantics:
/// Return the factorial
/// 
/// F(0) = F(1) = 1.
///
/// See also: "Infix Operator \"*\"", "GAMMA", 
#[inline]
pub fn fact<A: Number>(f: A) -> FnNumber1<A> {
    FnNumber1("FACT", f)
}

/// Returns double factorial (!!).
/// Syntax: FACTDOUBLE( F Integer; )
///
/// Constraints:
/// F ≥ 0
///
/// Semantics:
/// Return
/// 
/// Double factorial is computed by multiplying every other number in the 1..N 
/// range, with N always being included.
///
/// See also: "Infix Operator \"*\"", "GAMMA", "FACT", 
#[inline]
pub fn factdouble<A: Number>(f: A) -> FnNumber1<A> {
    FnNumber1("FACTDOUBLE", f)
}

/// Return gamma function value.
/// Syntax: GAMMA( N Number; )
///
/// Constraints:
/// N ≠ 0 and N not a negative integer.
///
/// Semantics:
/// Return
/// 
/// with Γ(N + 1) = N * Γ(N). Note that for non-negative integers N, Γ(N + 
/// 1) = N! = FACT(N). Note that GAMMA can accept non-integers.
///
/// See also: "FACT", 
#[inline]
pub fn gamma<A: Number>(n: A) -> FnNumber1<A> {
    FnNumber1("GAMMA", n)
}

/// Returns the natural logarithm of the GAMMA function.
/// Syntax: GAMMALN( X Number; )
///
/// Constraints:
/// For each X, X > 0
///
/// Semantics:
/// Returns the same value as LN(GAMMA(X))
///
/// See also: "GAMMA", "FACT", 
#[inline]
pub fn gammaln<A: Number>(x: A) -> FnNumber1<A> {
    FnNumber1("GAMMALN", x)
}

/// Returns the greatest common divisor (GCD)
/// Syntax: GCD({ X NumberSequenceList}+ )
///
/// Constraints:
/// For all a in X: INT(a) ≥ 0 and for at least one a in X: INT(a) > 0
///
/// Semantics:
/// Return the largest integer N such that for every a in X: INT(a) is a 
/// multiple of N.
///
/// Note:
/// If for all a in X: INT(a) = 0 the return value is implementation-defined 
/// but is either an Error or 0.
///
/// See also: "LCM", "INT", 
#[inline]
pub fn gcd<A: Sequence>(x: A) -> FnNumber1<A> {
    FnNumber1("GCD", x)
}

/// Returns 1 if a number is greater than or equal to another number, else 
/// returns 0.
/// Syntax: GESTEP( X Number; )
///
/// Semantics:
/// Number X is tested against number Step. If greater or equal 1 is returned, 
/// else 0. The second parameter is assumed 0 if omitted. If one of the 
/// parameters is not a Number, the function results in an Error.
#[inline]
pub fn gestep<A: Number>(x: A) -> FnNumber1<A> {
    FnNumber1("GESTEP", x)
}

/// Returns 1 if a number is greater than or equal to another number, else 
/// returns 0.
/// Syntax: GESTEP( X Number;[; Step Number] )
///
/// Semantics:
/// Number X is tested against number Step. If greater or equal 1 is returned, 
/// else 0. The second parameter is assumed 0 if omitted. If one of the 
/// parameters is not a Number, the function results in an Error.
#[inline]
pub fn gestep_<A: Number, B: Number>(x: A, step: B) -> FnNumber2<A, B> {
    FnNumber2("GESTEP", x, step)
}

/// Returns the least common multiplier
/// Syntax: LCM({ X NumberSequenceList}+ )
///
/// Constraints:
/// For all in X: INT(X) = X, X ≥ 0
///
/// Semantics:
/// Return the smallest integer that is the multiple of the given values. Each 
/// value has INT applied to it first. Note that if given two numbers, ABS(a * 
/// b) = LCM(a;b) * GCD(a;b).
///
/// See also: "GCD", "INT", 
#[inline]
pub fn lcm<A: Sequence>(x: A) -> FnNumber1<A> {
    FnNumber1("LCM", x)
}

/// Return the natural logarithm of a number.
/// Syntax: LN( X Number; )
///
/// Constraints:
/// X > 0
///
/// Semantics:
/// Computes the natural logarithm (base e) of the given number.
///
/// See also: "LOG", "LOG10", "POWER", "EXP", 
#[inline]
pub fn ln<A: Number>(x: A) -> FnNumber1<A> {
    FnNumber1("LN", x)
}

/// Return the logarithm of a number in a specified base.
/// Syntax: LOG( N Number; )
///
/// Constraints:
/// N > 0
///
/// Semantics:
/// Computes the logarithm of a number in the specified base. Note that if the 
/// base is not specified, the logarithm base 10 is returned.
///
/// See also: "LOG10", "LN", "POWER", "EXP", 
#[inline]
pub fn log<A: Number>(n: A) -> FnNumber1<A> {
    FnNumber1("LOG", n)
}

/// Return the logarithm of a number in a specified base.
/// Syntax: LOG( N Number;[; Base Number] )
///
/// Constraints:
/// N > 0
///
/// Semantics:
/// Computes the logarithm of a number in the specified base. Note that if the 
/// base is not specified, the logarithm base 10 is returned.
///
/// See also: "LOG10", "LN", "POWER", "EXP", 
#[inline]
pub fn log_<A: Number, B: Number>(n: A, base: B) -> FnNumber2<A, B> {
    FnNumber2("LOG", n, base)
}

/// Return the base 10 logarithm of a number.
/// Syntax: LOG10( N Number; )
///
/// Constraints:
/// N > 0
///
/// Semantics:
/// Computes the base 10 logarithm of a number.
///
/// See also: "LOG", "LN", "POWER", "EXP", 
#[inline]
pub fn log10<A: Number>(n: A) -> FnNumber1<A> {
    FnNumber1("LOG10", n)
}

/// Return the remainder when one number is divided by another number.
/// Syntax: MOD( A Number;; B Number; )
///
/// Constraints:
/// B != 0
///
/// Semantics:
/// Computes the remainder of A / B. The remainder has the same sign as B.
///
/// See also: "Infix Operator \"/\"", "QUOTIENT", 
#[inline]
pub fn mod_<A: Number, B: Number>(a: A, b: B) -> FnNumber2<A, B> {
    FnNumber2("MOD", a, b)
}

/// Returns the multinomial for the given values.
/// Syntax: MULTINOMIAL({ A NumberSequence}+ )
///
/// Constraints:
/// None
///
/// Semantics:
/// Returns the multinomial of the sequence A = (a1, a2, ..., an). Multinomial 
/// is defined as FACT(a1 + a2 +...+ an) / (FACT(a1) * FACT(a2) *...* FACT(an))
///
/// See also: "FACT", 
#[inline]
pub fn multinomial<A: Sequence>(a: A) -> FnNumber1<A> {
    FnNumber1("MULTINOMIAL", a)
}

/// Rounds a number up to the nearest odd integer, where "up" means "away from 
/// 0".
/// Syntax: ODD( N Number; )
///
/// Constraints:
/// None
///
/// Semantics:
/// Returns the odd integer whose sign is the same as N's and whose absolute 
/// value is greater than or equal to the absolute value of N. In other words, 
/// any "rounding" is away from zero. By definition, ODD(0) is 1.
///
/// See also: "EVEN", 
#[inline]
pub fn odd<A: Number>(n: A) -> FnNumber1<A> {
    FnNumber1("ODD", n)
}

/// Return the approximate value of π.
/// Syntax: PI( )
///
/// Constraints:
/// None.
///
/// Semantics:
/// This function takes no arguments and returns the (approximate) value of π 
/// (pi). Evaluators should use the closest possible numerical representation 
/// that is possible in their representation of numbers.
///
/// See also: "SIN", "COS", 
#[inline]
pub fn pi() -> FnNumber0 {
    FnNumber0("PI", )
}

/// Return the value of one number raised to the power of another number.
/// Syntax: POWER( A Number;; B Number; )
///
/// Constraints:
/// None
///
/// Semantics:
/// Computes A raised to the power B.
/// 
/// •POWER(0,0) is implementation-defined, but shall be one of 0,1, or an 
/// Error.
/// 
/// •POWER(0,B), where B < 0, shall return an Error.
/// 
/// •POWER(A,B), where A ≤ 0 and INT(B) != B, is implementation-defined.
///
/// See also: "LOG", "LOG10", "LN", "EXP", 
#[inline]
pub fn power<A: Number, B: Number>(a: A, b: B) -> FnNumber2<A, B> {
    FnNumber2("POWER", a, b)
}

/// Multiply the set of numbers, including all numbers inside ranges.
/// Syntax: PRODUCT({ N NumberSequenceList}+ )
///
/// Constraints:
/// None
///
/// Semantics:
/// Returns the product of the Numbers (and only the Numbers, i.e., not Text 
/// inside ranges).
///
/// See also: "SUM", 
#[inline]
pub fn product<A: Sequence>(n: A) -> FnNumber1<A> {
    FnNumber1("PRODUCT", n)
}

/// Return the integer portion of a division.
/// Syntax: QUOTIENT( A Number;; B Number; )
///
/// Constraints:
/// B ≠ 0
///
/// Semantics:
/// Return the integer portion of a division.
///
/// See also: "MOD", 
#[inline]
pub fn quotient<A: Number, B: Number>(a: A, b: B) -> FnNumber2<A, B> {
    FnNumber2("QUOTIENT", a, b)
}

/// Convert degrees to radians.
/// Syntax: RADIANS( N Number; )
///
/// Constraints:
/// None
///
/// Semantics:
/// Converts a number in degrees into a number in radians. RADIANS(N) is equal 
/// to N * PI() / 180.
///
/// See also: "DEGREES", "PI", 
#[inline]
pub fn radians<A: Number>(n: A) -> FnNumber1<A> {
    FnNumber1("RADIANS", n)
}

/// Return a random number between 0 (inclusive) and 1 (exclusive).
/// Syntax: RAND( )
///
/// Semantics:
/// This function takes no arguments and returns a random number between 0 
/// (inclusive) and 1 (exclusive). Note that unlike most functions, this 
/// function will typically return different values when called each time with 
/// the same (empty set of) parameters.
///
/// See also: "RANDBETWEEN", 
#[inline]
pub fn rand() -> FnNumber0 {
    FnNumber0("RAND", )
}

/// Return a random integer number between A and B.
/// Syntax: RANDBETWEEN( A Integer;; B Integer; )
///
/// Constraints:
/// A ≤ B
///
/// Semantics:
/// The function returns a random integer number between A and B inclusive. 
/// Note that unlike most functions, this function will often return different 
/// values when called each time with the same parameters.
///
/// See also: "RAND", 
#[inline]
pub fn randbetween<A: Number, B: Number>(a: A, b: B) -> FnNumber2<A, B> {
    FnNumber2("RANDBETWEEN", a, b)
}

/// Return the secant of an angle specified in radians.
/// Syntax: SEC( N Number; )
///
/// Constraints:
/// None
///
/// Semantics:
/// Computes the secant cosine of an angle specified in radians. Equivalent to:
/// 
/// 1 / COS(N)
///
/// See also: "SIN", 
#[inline]
pub fn sec<A: Number>(n: A) -> FnNumber1<A> {
    FnNumber1("SEC", n)
}

/// Returns the sum of a power series.
/// Syntax: SERIESSUM( X Number;; N Number;; M Number;; Coefficients Array; )
///
/// Arguments:
/// 
/// •X: the independent variable of the power series.
/// 
/// •N: the initial power to which X is to be raised.
/// 
/// •M: the increment by which to increase N for each term in the series.
/// 
/// •Coefficients: a set of coefficients by which each successive power of 
/// the variable X is multiplied.
///
/// Constraints:
/// 
/// All elements of Coefficients are of type Number.
/// 
/// X ≠ 0 if any of the exponents, which are generated from N and M, are 
/// negative.
///
/// Semantics:
/// Returns a sum of powers of the number X.
/// 
/// With C being the number of coefficients the function is computed as:
/// 
/// If X = 0 and all of the exponents are non-negative then
/// shall be set to 1 and
/// shall be set to 0.
#[inline]
pub fn seriessum<A: Number, B: Number, C: Number, D: Array>(x: A, n: B, m: C, coefficients: D) -> FnNumber4<A, B, C, D> {
    FnNumber4("SERIESSUM", x, n, m, coefficients)
}

/// Return the sign of a number.
/// Syntax: SIGN( N Number; )
///
/// Constraints:
/// None
///
/// Semantics:
/// If N < 0, returns -1; if N > 0, returns +1; if N = 0, returns 0.
///
/// See also: "ABS", 
#[inline]
pub fn sign<A: Number>(n: A) -> FnNumber1<A> {
    FnNumber1("SIGN", n)
}

/// Return the sine of an angle specified in radians.
/// Syntax: SIN( N Number; )
///
/// Constraints:
/// None
///
/// Semantics:
/// Computes the sine of an angle specified in radians.
///
/// See also: "ASIN", "RADIANS", "DEGREES", 
#[inline]
pub fn sin<A: Number>(n: A) -> FnNumber1<A> {
    FnNumber1("SIN", n)
}

/// Return the hyperbolic sine of the given hyperbolic angle.
/// Syntax: SINH( N Number; )
///
/// Constraints:
/// None
///
/// Semantics:
/// Computes the hyperbolic sine of a hyperbolic angle. The hyperbolic sine is 
/// an analog of the ordinary (circular) sine. The points (cosh t, sinh t) 
/// define the right half of the equilateral hyperbola, just as the points (cos 
/// t, sin t) define the points of a circle.
///
/// See also: "ASINH", "COSH", "TANH", 
#[inline]
pub fn sinh<A: Number>(n: A) -> FnNumber1<A> {
    FnNumber1("SINH", n)
}

/// Return the hyperbolic secant of the given angle specified in radians.
/// Syntax: SECH( N Number; )
///
/// Constraints:
/// None
///
/// Semantics:
/// Computes the hyperbolic secant of a hyperbolic angle. This is equivalent 
/// to:
/// 
/// 1 / COSH(N)
///
/// See also: "SINH", "COSH", "CSCH", 
#[inline]
pub fn sech<A: Number>(n: A) -> FnNumber1<A> {
    FnNumber1("SECH", n)
}

/// Return the square root of a number.
/// Syntax: SQRT( N Number; )
///
/// Constraints:
/// N ≥ 0
///
/// Semantics:
/// Returns the square root of a non-negative number. This function shall 
/// produce an Error if given a negative number; for producing complex numbers, 
/// see IMSQRT.
///
/// See also: "POWER", "IMSQRT", "SQRTPI", 
#[inline]
pub fn sqrt<A: Number>(n: A) -> FnNumber1<A> {
    FnNumber1("SQRT", n)
}

/// Return the square root of a number multiplied by π (pi).
/// Syntax: SQRTPI( N Number; )
///
/// Constraints:
/// N ≥ 0
///
/// Semantics:
/// Returns the square root of a non-negative number after it was first 
/// multiplied by π, that is, SQRT(N * PI()). This function shall produce an 
/// Error if given a negative number; for producing complex numbers, see 
/// IMSQRT.
///
/// See also: "POWER", "SQRT", "PI", "IMSQRT", 
#[inline]
pub fn sqrtpi<A: Number>(n: A) -> FnNumber1<A> {
    FnNumber1("SQRTPI", n)
}

/// Evaluates a function on a range.
/// Syntax: SUBTOTAL( Function Integer;; Sequence NumberSequence; )
///
/// Constraints:
/// None
///
/// Semantics:
/// Computes a given function on a number sequence. The function is denoted by 
/// the first parameter: The difference from standard functions is that all 
/// members of the sequence are excluded which:
/// 
/// •include a call to SUBTOTAL in their formula
/// 
/// •are in a row that is hidden by a table:visibility=”filter” attribute 
/// of the <table:table-row> element (OpenDocument, Part 3, 19.754).
/// 
/// •are in a row that is hidden by a table:visibility=”collapse” 
/// attribute of the <table:table-row> element if the function ID is one of 
/// 101...111.
///
/// See also: "SUM", "AVERAGE", 
#[inline]
pub fn subtotal<A: Sequence>(function: SubtotalFunction, sequence: A) -> FnNumber2<SubtotalFunction, A> {
    FnNumber2("SUBTOTAL", function, sequence)
}

/// Sum (add) the set of numbers, including all numbers in ranges.
/// Syntax: SUM({ N NumberSequenceList}+ )
///
/// Constraints:
/// N != {}; Evaluators may evaluate expressions that do not meet this 
/// constraint.
///
/// Semantics:
/// Adds Numbers (and only Numbers) together (see the text on conversions).
///
/// See also: "AVERAGE", 
#[inline]
pub fn sum<A: Sequence>(n: A) -> FnNumber1<A> {
    FnNumber1("SUM", n)
}

/// Sum the values of cells in a range that meet a criteria.
/// Syntax: SUMIF( R ReferenceList|Reference;; C Criterion; )
///
/// Constraints:
/// Does not accept constant values as the range parameter.
///
/// Semantics:
/// Sums the values of type Number in the range R or S that meet the Criterion 
/// C (4.11.8).
/// 
/// If S is not given, R may be a reference list. If S is given, R shall not be 
/// a reference list with more than 1 references and an Error be generated if 
/// it was.
/// 
/// If the optional range S is included, then the values of S starting from the 
/// top left cell and matching the geometry of R (same number of rows and 
/// columns) are summed if the corresponding value in R meets the Criterion. 
/// The actual range S is not considered. If the resulting range exceeds the 
/// sheet bounds, column numbers larger than the maximum column and row numbers 
/// larger than the maximum row are silently ignored, no Error is generated for 
/// this case.
/// 
/// The values returned may vary depending upon the 
/// HOST-USE-REGULAR-EXPRESSIONS or HOST-USE-WILDCARDS or 
/// HOST-SEARCH-CRITERIA-MUST-APPLY-TO-WHOLE-CELL properties. 3.4
///
/// See also: "COUNTIF", "SUM", "Infix Operator \"=\"", "Infix Operator \"<>\"", "Infix Operator Ordered Comparison (\"<\", \"<=\", \">\", \">=\")", 
#[inline]
pub fn sumif<A: Reference, B: Criterion>(r: A, c: B) -> FnNumber2<A, B> {
    FnNumber2("SUMIF", r, c)
}

/// Sum the values of cells in a range that meet a criteria.
/// Syntax: SUMIF( R ReferenceList|Reference;; C Criterion;[; S Reference] )
///
/// Constraints:
/// Does not accept constant values as the range parameter.
///
/// Semantics:
/// Sums the values of type Number in the range R or S that meet the Criterion 
/// C (4.11.8).
/// 
/// If S is not given, R may be a reference list. If S is given, R shall not be 
/// a reference list with more than 1 references and an Error be generated if 
/// it was.
/// 
/// If the optional range S is included, then the values of S starting from the 
/// top left cell and matching the geometry of R (same number of rows and 
/// columns) are summed if the corresponding value in R meets the Criterion. 
/// The actual range S is not considered. If the resulting range exceeds the 
/// sheet bounds, column numbers larger than the maximum column and row numbers 
/// larger than the maximum row are silently ignored, no Error is generated for 
/// this case.
/// 
/// The values returned may vary depending upon the 
/// HOST-USE-REGULAR-EXPRESSIONS or HOST-USE-WILDCARDS or 
/// HOST-SEARCH-CRITERIA-MUST-APPLY-TO-WHOLE-CELL properties. 3.4
///
/// See also: "COUNTIF", "SUM", "Infix Operator \"=\"", "Infix Operator \"<>\"", "Infix Operator Ordered Comparison (\"<\", \"<=\", \">\", \">=\")", 
#[inline]
pub fn sumif_<A: Reference, B: Criterion, C: Reference>(r: A, c: B, s: C) -> FnNumber3<A, B, C> {
    FnNumber3("SUMIF", r, c, s)
}

/// Returns the sum of the products of the matrix elements.
/// Syntax: SUMPRODUCT({ A Array}+ )
///
/// Constraints:
/// All matrices shall have the same dimensions.
///
/// Semantics:
/// Multiplies the corresponding elements of all matrices and returns the sum 
/// of them.
/// 
/// where
/// denotes an element of the matrix
/// .
#[inline]
pub fn sumproduct<A: Sequence>(a: A) -> FnNumber1<A> {
    FnNumber1("SUMPRODUCT", a)
}

/// Sum (add) the set of squares of numbers, including all numbers in ranges
/// Syntax: SUMSQ({ N NumberSequence}+ )
///
/// Constraints:
/// N != {}; Evaluators may evaluate expressions that do not meet this 
/// constraint.
///
/// Semantics:
/// Adds squares of Numbers (and only Numbers) together. See the text on 
/// conversions.
#[inline]
pub fn sumsq<A: Sequence>(n: A) -> FnNumber1<A> {
    FnNumber1("SUMSQ", n)
}

/// Returns the sum of the difference between the squares of the matrices A and 
/// B.
/// Syntax: SUMX2MY2( A Array;; B Array; )
///
/// Constraints:
/// Both matrices shall have the same dimensions.
///
/// Semantics:
/// Sums up the differences of the corresponding elements squares for two 
/// matrices.
#[inline]
pub fn sumx2my2<A: Array, B: Array>(a: A, b: B) -> FnNumber2<A, B> {
    FnNumber2("SUMX2MY2", a, b)
}

/// Returns the total sum of the squares of the matrices A and B.
/// Syntax: SUMX2PY2( A Array;; B Array; )
///
/// Constraints:
/// Both matrices shall have the same dimensions.
///
/// Semantics:
/// Sums up the squares of each element of the two matrices.
#[inline]
pub fn sumx2py2<A: Array, B: Array>(a: A, b: B) -> FnNumber2<A, B> {
    FnNumber2("SUMX2PY2", a, b)
}

/// Returns the sum of the squares of the differences between matrix A and B.
/// Syntax: SUMXMY2( A Array;; B Array; )
///
/// Constraints:
/// Both matrices shall have the same dimensions.
///
/// Semantics:
/// Sums up the squares of the differences of the corresponding elements for 
/// two matrices.
#[inline]
pub fn sumxmy2<A: Array, B: Array>(a: A, b: B) -> FnNumber2<A, B> {
    FnNumber2("SUMXMY2", a, b)
}

/// Return the tangent of an angle specified in radians
/// Syntax: TAN( N Number; )
///
/// Constraints:
/// None
///
/// Semantics:
/// Computes the tangent of an angle specified in radians.
/// 
/// TAN(x) = SIN(x) / COS(x)
///
/// See also: "ATAN", "ATAN2", "RADIANS", "DEGREES", "SIN", "COS", "COT", 
#[inline]
pub fn tan<A: Number>(n: A) -> FnNumber1<A> {
    FnNumber1("TAN", n)
}

/// Return the hyperbolic tangent of the given hyperbolic angle
/// Syntax: TANH( N Number; )
///
/// Constraints:
/// None
///
/// Semantics:
/// Computes the hyperbolic tangent of a hyperbolic angle. The hyperbolic 
/// tangent is an analog of the ordinary (circular) tangent. The points (cosh 
/// t, sinh t) define the right half of the equilateral hyperbola, just as the 
/// points (cos t, sin t) define the points of a circle.
///
/// See also: "ATANH", "SINH", "COSH", "FISHERINV", 
#[inline]
pub fn tanh<A: Number>(n: A) -> FnNumber1<A> {
    FnNumber1("TANH", n)
}
