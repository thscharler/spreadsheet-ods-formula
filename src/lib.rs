use spreadsheet_ods::{CellRange, CellRef};
use std::borrow::Borrow;
use std::borrow::Cow;
use std::fmt::{Display, Formatter, Write};
use std::ops::{Add, BitAnd, BitXor, Div, Mul, Neg, Sub};

mod generated;

pub mod bit;
pub mod cmp;
pub mod complex;
pub mod date;
pub mod db;
pub mod ext;
pub mod fin;
pub mod info;
pub mod logic;
pub mod lookup;
pub mod math;
pub mod matrix;
pub mod op;
pub mod round;
pub mod stat;

/// The traits for this crate.
/// And the function p() for parentheses.
pub mod prelude {
    pub use crate::Any;
    pub use crate::{AnyOp, LogicalOp, NumberOp, ReferenceOp, TextOp};
}

/// All functions in one module, and also all families of functions.
pub mod of {
    pub use crate::bit::*;
    pub use crate::cmp::*;
    pub use crate::complex::*;
    pub use crate::date::*;
    pub use crate::db::*;
    pub use crate::ext::*;
    pub use crate::fin::*;
    pub use crate::info::*;
    pub use crate::logic::*;
    pub use crate::lookup::*;
    pub use crate::math::*;
    pub use crate::matrix::*;
    pub use crate::op::*;
    pub use crate::round::*;
    pub use crate::stat::*;
    pub use crate::{
        bit, cmp, complex, date, db, ext, fin, info, logic, lookup, math, matrix, op, round, stat,
    };
}

// -----------------------------------------------------------------------

/// Base trait for any part of a formula.
pub trait Any {
    /// Output to a formula.
    fn formula(&self, buf: &mut String);
}
/// Numeric parameter.
pub trait Number: Any {}
/// Text parameter.
pub trait Text: Any {}
/// Logical parameter.
pub trait Logical: Any {}
/// Reference parameter.
pub trait Reference: Any {}
/// Matrix parameter.
pub trait Matrix: Any {}
/// Array parameter.
pub trait Array: Any {}
/// Alias for a cell reference. A cell range containing headers and a data set.
pub trait Database: Any {}
/// Filter/Search criterion.
pub trait Criterion: Any {}
/// A cell range containing headers and filters.
pub trait Criteria: Any {}
/// Sequence of values.
pub trait Sequence: Any {}
/// Text or Number
pub trait TextOrNumber: Any {}
/// A single scalar value.
pub trait Scalar: Any {}
/// A field denominator for a db.
pub trait Field: Any {}
/// A date/time parameter.
pub trait DateTimeParam: Any {}

// -----------------------------------------------------------------------

/// Comparision operators
pub trait AnyOp<T: Any> {
    /// equal
    fn eq<U: Any>(self, other: U) -> OpLogical<T, U>;
    /// not equal
    fn ne<U: Any>(self, other: U) -> OpLogical<T, U>;
    /// less than
    fn lt<U: Any>(self, other: U) -> OpLogical<T, U>;
    /// less than or equal
    fn le<U: Any>(self, other: U) -> OpLogical<T, U>;
    /// greater than
    fn gt<U: Any>(self, other: U) -> OpLogical<T, U>;
    /// greater than or equal
    fn ge<U: Any>(self, other: U) -> OpLogical<T, U>;
}

/// Operations on number-like values.
pub trait NumberOp<T: Any> {
    /// add
    fn add<U: Number>(self, other: U) -> OpNumber<T, U>;
    /// subtract
    fn sub<U: Number>(self, other: U) -> OpNumber<T, U>;
    /// multiply
    fn mul<U: Number>(self, other: U) -> OpNumber<T, U>;
    /// divide
    fn div<U: Number>(self, other: U) -> OpNumber<T, U>;
    /// exponential
    fn pow<U: Number>(self, other: U) -> OpNumber<T, U>;
    /// as percentage
    fn percent(self) -> OpNumber<T, ()>;
}

/// Operations on text-like values.
pub trait TextOp<T: Any> {
    /// concat text
    fn concat<U: Text>(self, other: U) -> OpText<T, U>;
}

/// Operations on boolean-like values.
pub trait LogicalOp<T: Any> {
    /// and
    fn and<U: Logical>(self, other: U) -> FnLogical2<T, U>;
    /// or
    fn or<U: Logical>(self, other: U) -> FnLogical2<T, U>;
    /// xor
    fn xor<U: Logical>(self, other: U) -> FnLogical2<T, U>;
}

/// Operations on references.
pub trait ReferenceOp<T: Any> {
    /// intersection of references
    fn intersect<U: Reference>(self, other: U) -> OpReference<T, U>;
    /// concatenation of references
    fn refcat<U: Reference>(self, other: U) -> OpReference<T, U>;
}

// -----------------------------------------------------------------------

impl<T: Any> AnyOp<T> for T {
    #[inline]
    fn eq<U: Any>(self, other: U) -> OpLogical<T, U> {
        OpLogical(self, "=", other)
    }

    #[inline]
    fn ne<U: Any>(self, other: U) -> OpLogical<T, U> {
        OpLogical(self, "<>", other)
    }

    #[inline]
    fn lt<U: Any>(self, other: U) -> OpLogical<T, U> {
        OpLogical(self, "<", other)
    }

    #[inline]
    fn le<U: Any>(self, other: U) -> OpLogical<T, U> {
        OpLogical(self, "<=", other)
    }

    #[inline]
    fn gt<U: Any>(self, other: U) -> OpLogical<T, U> {
        OpLogical(self, ">", other)
    }

    #[inline]
    fn ge<U: Any>(self, other: U) -> OpLogical<T, U> {
        OpLogical(self, ">=", other)
    }
}

impl<T: Number> NumberOp<T> for T {
    #[inline]
    fn add<U: Number>(self, other: U) -> OpNumber<T, U> {
        OpNumber(self, "+", other)
    }

    #[inline]
    fn sub<U: Number>(self, other: U) -> OpNumber<T, U> {
        OpNumber(self, "-", other)
    }

    #[inline]
    fn mul<U: Number>(self, other: U) -> OpNumber<T, U> {
        OpNumber(self, "*", other)
    }

    #[inline]
    fn div<U: Number>(self, other: U) -> OpNumber<T, U> {
        OpNumber(self, "/", other)
    }

    #[inline]
    fn pow<U: Number>(self, other: U) -> OpNumber<T, U> {
        OpNumber(self, "^", other)
    }

    #[inline]
    fn percent(self) -> OpNumber<T, ()> {
        OpNumber(self, "%", ())
    }
}

impl<T: Text> TextOp<T> for T {
    #[inline]
    fn concat<U: Text>(self, other: U) -> OpText<T, U> {
        OpText(self, "&", other)
    }
}

impl<T: Logical> LogicalOp<T> for T {
    #[inline]
    fn and<U: Logical>(self, other: U) -> FnLogical2<T, U> {
        FnLogical2("AND", self, other)
    }

    #[inline]
    fn or<U: Logical>(self, other: U) -> FnLogical2<T, U> {
        FnLogical2("OR", self, other)
    }

    #[inline]
    fn xor<U: Logical>(self, other: U) -> FnLogical2<T, U> {
        FnLogical2("XOR", self, other)
    }
}

impl<T: Reference> ReferenceOp<T> for T {
    #[inline]
    fn intersect<U: Reference>(self, other: U) -> OpReference<T, U> {
        OpReference(self, "!", other)
    }

    #[inline]
    fn refcat<U: Reference>(self, other: U) -> OpReference<T, U> {
        OpReference(self, "~", other)
    }
}

// -----------------------------------------------------------------------

macro_rules! any_struct {
    (VAL $t:ident) => {

        /// A newtype wrapper for a simple value.
        /// Useful in combination with overloaded operators.
        #[derive(Debug)]
        pub struct $t<A:Any>(pub A);

        impl<A:Any> Any for $t<A> {
             #[inline]
             fn formula(&self, buf: &mut String) {
                self.0.formula(buf);
            }
        }

    };
    (OP $t:ident) => {

        /// Operator definition.
        #[derive(Debug)]
        pub struct $t<A:Any, B:Any>(
            pub A,
            pub &'static str,
            pub B
        );

        impl<A:Any, B: Any> Any for $t<A,B> {
            #[inline]
            fn formula(&self, buf: &mut String) {
                self.0.formula(buf);
                buf.push_str(self.1.as_ref());
                self.2.formula(buf);
            }
        }

    };
    (VAR $t:ident) => {

        /// Function with variable number of parameters.
        pub struct $t(
            pub &'static str,
            pub Vec<Box<dyn Any>>
        );

        impl Any for $t {
            #[inline]
            fn formula(&self, buf: &mut String) {
                buf.push_str(self.0);
                buf.push('(');
                for (i, v) in self.1.iter().enumerate() {
                    if i > 0 {
                        buf.push(';');
                    }
                    let _ = v.formula(buf);
                }
                buf.push(')');
            }
        }

    };
    ($t:ident) => {

        /// Parameterless function.
        #[derive(Debug)]
        pub struct $t(
            pub &'static str
        );

        impl Any for $t {
            #[inline]
            fn formula(&self, buf: &mut String) {
                buf.push_str(self.0.as_ref());
                buf.push('(');
                buf.push(')');
            }
        }

    };
    ($t:ident : $tname0:tt $($tname:tt $tidx:tt)*) => {

        /// Function with parameters.
        #[derive(Debug)]
        pub struct $t<$tname0: Any+'static $(,$tname: Any+'static)*>(
            pub &'static str,
            pub $tname0
            $(, pub $tname)*
        );

        impl <$tname0: Any+'static $(,$tname: Any+'static)*> Any for $t<$tname0 $(,$tname)*> {
            #[inline]
            fn formula(&self, buf: &mut String) {
                buf.push_str(self.0.as_ref());
                buf.push('(');
                self.1.formula(buf);
                $(
                    buf.push(';');
                    self.$tidx.formula(buf);
                )*
                buf.push(')');
            }
        }

    }
}

macro_rules! fn_any {
    (VAL $t:ident) => {
        any_struct!(VAL $t);
        fn_any!(__IMPL $t: A);
    };
    (OP $t:ident) => {
        any_struct!(OP $t);
        fn_any!(__IMPL $t: A B);
    };
    (VAR $t:ident) => {
        any_struct!(VAR $t);
        fn_any!(__IMPL $t:);
    };
    ($t:ident) => {
        any_struct!($t);
        fn_any!(__IMPL $t:);
    };
    ($t:ident : $tname0:tt $($tname:tt $tidx:tt)*) => {
        any_struct!($t: $tname0 $($tname $tidx)*);
        fn_any!(__IMPL $t: $tname0 $($tname)*);
    };
    (__IMPL $t:ident : $($l:lifetime)?) => {
        impl $(<$l>)? Number for $t$(<$l>)?  {}
        impl $(<$l>)? Text for $t$(<$l>)?  {}
        impl $(<$l>)? Logical for $t$(<$l>)?  {}
        impl $(<$l>)? Sequence for $t$(<$l>)?  {}
        impl $(<$l>)? TextOrNumber for $t$(<$l>)?  {}
        impl $(<$l>)? Scalar for $t$(<$l>)?  {}
        impl $(<$l>)? Field for $t$(<$l>)?  {}
        impl $(<$l>)? DateTimeParam for $t$(<$l>)?  {}
    };
    (__IMPL $t:ident : $($l:lifetime)? $tname0:tt $($tname:tt)*) => {
        impl <$($l, )?$tname0: Any $(,$tname: Any)*> Number for $t<$($l, )?$tname0 $(,$tname)*> {}
        impl <$($l, )?$tname0: Any $(,$tname: Any)*> Text for $t<$($l, )?$tname0 $(,$tname)*> {}
        impl <$($l, )?$tname0: Any $(,$tname: Any)*> Logical for $t<$($l, )?$tname0 $(,$tname)*> {}
        impl <$($l, )?$tname0: Any $(,$tname: Any)*> Sequence for $t<$($l, )?$tname0 $(,$tname)*> {}
        impl <$($l, )?$tname0: Any $(,$tname: Any)*> TextOrNumber for $t<$($l, )?$tname0 $(,$tname)*> {}
        impl <$($l, )?$tname0: Any $(,$tname: Any)*> Scalar for $t<$($l, )?$tname0 $(,$tname)*> {}
        impl <$($l, )?$tname0: Any $(,$tname: Any)*> Field for $t<$($l, )?$tname0 $(,$tname)*> {}
        impl <$($l, )?$tname0: Any $(,$tname: Any)*> DateTimeParam for $t<$($l, )?$tname0 $(,$tname)*> {}
    };
}

fn_any!(VAL ValAny);
fn_any!(OP OpAny);
fn_any!(VAR FnAnyVar);
fn_any!(FnAny0);
fn_any!(FnAny1: A);
fn_any!(FnAny2: A B 2);
fn_any!(FnAny3: A B 2 C 3);
fn_any!(FnAny4: A B 2 C 3 D 4);
fn_any!(FnAny5: A B 2 C 3 D 4 E 5);
fn_any!(FnAny6: A B 2 C 3 D 4 E 5 F 6);

macro_rules! fn_number {
    (VAL $t:ident) => {
        any_struct!(VAL $t);
        fn_number!(__IMPL $t: A);
    };
    (OP $t:ident) => {
        any_struct!(OP $t);
        fn_number!(__IMPL $t: A B);
    };
    (VAR $t:ident) => {
        any_struct!(VAR $t);
        fn_number!(__IMPL $t:);
    };
    (REFVAR $t:ident) => {
        any_struct!(REFVAR $t);
        fn_number!(__IMPL $t: 'a);
    };
    ($t:ident) => {
        any_struct!($t);
        fn_number!(__IMPL $t:);
    };
    ($t:ident : $tname0:tt $($tname:tt $tidx:tt)*) => {
        any_struct!($t: $tname0 $($tname $tidx)*);
        fn_number!(__IMPL $t: $tname0 $($tname)*);
    };
    (__IMPL $t:ident : $($l:lifetime)?) => {
        impl $(<$l>)? Number for $t$(<$l>)?  {}
        impl $(<$l>)? Logical for $t$(<$l>)?  {}
        impl $(<$l>)? Sequence for $t$(<$l>)?  {}
        impl $(<$l>)? TextOrNumber for $t$(<$l>)?  {}
        impl $(<$l>)? Scalar for $t$(<$l>)?  {}
        impl $(<$l>)? Field for $t$(<$l>)?  {}
        impl $(<$l>)? DateTimeParam for $t$(<$l>)?  {}
    };
    (__IMPL $t:ident : $($l:lifetime)? $tname0:tt $($tname:tt)*) => {
        impl <$($l, )?$tname0: Any $(,$tname: Any)*> Number for $t<$($l, )?$tname0 $(,$tname)*> {}
        impl <$($l, )?$tname0: Any $(,$tname: Any)*> Logical for $t<$($l, )?$tname0 $(,$tname)*> {}
        impl <$($l, )?$tname0: Any $(,$tname: Any)*> Sequence for $t<$($l, )?$tname0 $(,$tname)*> {}
        impl <$($l, )?$tname0: Any $(,$tname: Any)*> TextOrNumber for $t<$($l, )?$tname0 $(,$tname)*> {}
        impl <$($l, )?$tname0: Any $(,$tname: Any)*> Scalar for $t<$($l, )?$tname0 $(,$tname)*> {}
        impl <$($l, )?$tname0: Any $(,$tname: Any)*> Field for $t<$($l, )?$tname0 $(,$tname)*> {}
        impl <$($l, )?$tname0: Any $(,$tname: Any)*> DateTimeParam for $t<$($l, )?$tname0 $(,$tname)*> {}
    };
}

fn_number!(VAL ValNumber);
fn_number!(OP OpNumber);
fn_number!(VAR FnNumberVar);
fn_number!(FnNumber0);
fn_number!(FnNumber1: A);
fn_number!(FnNumber2: A B 2);
fn_number!(FnNumber3: A B 2 C 3);
fn_number!(FnNumber4: A B 2 C 3 D 4);
fn_number!(FnNumber5: A B 2 C 3 D 4 E 5);
fn_number!(FnNumber6: A B 2 C 3 D 4 E 5 F 6);
fn_number!(FnNumber7: A B 2 C 3 D 4 E 5 F 6 G 7);

macro_rules! fn_text {
    (VAL $t:ident) => {
        any_struct!(VAL $t);
        fn_text!(__IMPL $t: A);
    };
    (OP $t:ident) => {
        any_struct!(OP $t);
        fn_text!(__IMPL $t: A B);
    };
    (VAR $t:ident) => {
        any_struct!(VAR $t);
        fn_text!(__IMPL $t:);
    };
    (REFVAR $t:ident) => {
        any_struct!(REFVAR $t);
        fn_text!(__IMPL $t: 'a);
    };
    ($t:ident) => {
        any_struct!($t);
        fn_text!(__IMPL $t:);
    };
    ($t:ident : $tname0:tt $($tname:tt $tidx:tt)*) => {
        any_struct!($t: $tname0 $($tname $tidx)*);
        fn_text!(__IMPL $t: $tname0 $($tname)*);
    };
    (__IMPL $t:ident : $($l:lifetime)?) => {
        impl $(<$l>)? Text for $t$(<$l>)?  {}
        impl $(<$l>)? Sequence for $t$(<$l>)?  {}
        impl $(<$l>)? TextOrNumber for $t$(<$l>)?  {}
        impl $(<$l>)? Scalar for $t$(<$l>)?  {}
        impl $(<$l>)? Field for $t$(<$l>)?  {}
        impl $(<$l>)? DateTimeParam for $t$(<$l>)?  {}
    };
    (__IMPL $t:ident : $($l:lifetime)? $tname0:tt $($tname:tt)*) => {
        impl <$($l, )?$tname0: Any $(,$tname: Any)*> Text for $t<$($l, )?$tname0 $(,$tname)*> {}
        impl <$($l, )?$tname0: Any $(,$tname: Any)*> Sequence for $t<$($l, )?$tname0 $(,$tname)*> {}
        impl <$($l, )?$tname0: Any $(,$tname: Any)*> TextOrNumber for $t<$($l, )?$tname0 $(,$tname)*> {}
        impl <$($l, )?$tname0: Any $(,$tname: Any)*> Scalar for $t<$($l, )?$tname0 $(,$tname)*> {}
        impl <$($l, )?$tname0: Any $(,$tname: Any)*> Field for $t<$($l, )?$tname0 $(,$tname)*> {}
        impl <$($l, )?$tname0: Any $(,$tname: Any)*> DateTimeParam for $t<$($l, )?$tname0 $(,$tname)*> {}
    };
}

fn_text!(VAL ValText);
fn_text!(OP OpText);
fn_text!(VAR FnTextVar);
fn_text!(FnText0);
fn_text!(FnText1: A);
fn_text!(FnText2: A B 2);
fn_text!(FnText3: A B 2 C 3);
fn_text!(FnText4: A B 2 C 3 D 4);
fn_text!(FnText5: A B 2 C 3 D 4 E 5);
fn_text!(FnText6: A B 2 C 3 D 4 E 5 F 6);

macro_rules! fn_logical {
    (VAL $t:ident) => {
        any_struct!(VAL $t);
        fn_logical!(__IMPL $t: A);
    };
    (OP $t:ident) => {
        any_struct!(OP $t);
        fn_logical!(__IMPL $t: A B);
    };
    (VAR $t:ident) => {
        any_struct!(VAR $t);
        fn_logical!(__IMPL $t:);
    };
    (REFVAR $t:ident) => {
        any_struct!(REFVAR $t);
        fn_logical!(__IMPL $t: 'a);
    };
    ($t:ident) => {
        any_struct!($t);
        fn_logical!(__IMPL $t:);
    };
    ($t:ident : $tname0:tt $($tname:tt $tidx:tt)*) => {
        any_struct!($t: $tname0 $($tname $tidx)*);
        fn_logical!(__IMPL $t: $tname0 $($tname)*);
    };
    (__IMPL $t:ident : $($l:lifetime)?) => {
        impl $(<$l>)? Number for $t$(<$l>)?  {}
        impl $(<$l>)? Logical for $t$(<$l>)?  {}
        impl $(<$l>)? Sequence for $t$(<$l>)?  {}
        impl $(<$l>)? TextOrNumber for $t$(<$l>)?  {}
        impl $(<$l>)? Scalar for $t$(<$l>)?  {}
    };
    (__IMPL $t:ident : $($l:lifetime)? $tname0:tt $($tname:tt)*) => {
        impl <$($l, )?$tname0: Any $(,$tname: Any)*> Number for $t<$($l, )?$tname0 $(,$tname)*> {}
        impl <$($l, )?$tname0: Any $(,$tname: Any)*> Logical for $t<$($l, )?$tname0 $(,$tname)*> {}
        impl <$($l, )?$tname0: Any $(,$tname: Any)*> Sequence for $t<$($l, )?$tname0 $(,$tname)*> {}
        impl <$($l, )?$tname0: Any $(,$tname: Any)*> TextOrNumber for $t<$($l, )?$tname0 $(,$tname)*> {}
        impl <$($l, )?$tname0: Any $(,$tname: Any)*> Scalar for $t<$($l, )?$tname0 $(,$tname)*> {}
    };
}

fn_logical!(VAL ValLogical);
fn_logical!(OP OpLogical);
fn_logical!(VAR FnLogicalVar);
fn_logical!(FnLogical0);
fn_logical!(FnLogical1: A);
fn_logical!(FnLogical2: A B 2);
fn_logical!(FnLogical3: A B 2 C 3);
fn_logical!(FnLogical4: A B 2 C 3 D 4);
fn_logical!(FnLogical5: A B 2 C 3 D 4 E 5);
fn_logical!(FnLogical6: A B 2 C 3 D 4 E 5 F 6);

macro_rules! fn_matrix {
    (VAL $t:ident) => {
        any_struct!(VAL $t);
        fn_matrix!(__IMPL $t: A);
    };
    (OP $t:ident) => {
        any_struct!(OP $t);
        fn_matrix!(__IMPL $t: A B);
    };
    (VAR $t:ident) => {
        any_struct!(VAR $t);
        fn_matrix!(__IMPL $t:);
    };
    (REFVAR $t:ident) => {
        any_struct!(REFVAR $t);
        fn_matrix!(__IMPL $t: 'a);
    };
    ($t:ident) => {
        any_struct!($t);
        fn_matrix!(__IMPL $t:);
    };
    ($t:ident : $tname0:tt $($tname:tt $tidx:tt)*) => {
        any_struct!($t: $tname0 $($tname $tidx)*);
        fn_matrix!(__IMPL $t: $tname0 $($tname)*);
    };
    (__IMPL $t:ident : $($l:lifetime)?) => {
        impl $(<$l>)? Matrix for $t$(<$l>)?  {}
    };
    (__IMPL $t:ident : $($l:lifetime)? $tname0:tt $($tname:tt)*) => {
        impl <$($l, )?$tname0: Any $(,$tname: Any)*> Matrix for $t<$($l, )?$tname0 $(,$tname)*> {}
    };
}

fn_matrix!(VAL ValMatrix);
fn_matrix!(OP OpMatrix);
fn_matrix!(VAR FnMatrixVar);
fn_matrix!(FnMatrix0);
fn_matrix!(FnMatrix1: A);

macro_rules! fn_reference {
    (VAL $t:ident) => {
        any_struct!(VAL $t);
        fn_reference!(__IMPL $t: A);
    };
    (OP $t:ident) => {
        any_struct!(OP $t);
        fn_reference!(__IMPL $t: A B);
    };
    (VAR $t:ident) => {
        any_struct!(VAR $t);
        fn_reference!(__IMPL $t:);
    };
    (REFVAR $t:ident) => {
        any_struct!(REFVAR $t);
        fn_reference!(__IMPL $t: 'a);
    };
    ($t:ident) => {
        any_struct!($t);
        fn_reference!(__IMPL $t:);
    };
    ($t:ident : $tname0:tt $($tname:tt $tidx:tt)*) => {
        any_struct!($t: $tname0 $($tname $tidx)*);
        fn_reference!(__IMPL $t: $tname0 $($tname)*);
    };
    (__IMPL $t:ident : $($l:lifetime)?) => {
        impl $(<$l>)? Number for $t$(<$l>)?  {}
        impl $(<$l>)? Text for $t$(<$l>)?  {}
        impl $(<$l>)? Logical for $t$(<$l>)?  {}
        impl $(<$l>)? Reference for $t$(<$l>)?  {}
        impl $(<$l>)? Matrix for $t$(<$l>)?  {}
        impl $(<$l>)? Array for $t$(<$l>)?  {}
        impl $(<$l>)? Database for $t$(<$l>)?  {}
        impl $(<$l>)? Criteria for $t$(<$l>)?  {}
        impl $(<$l>)? Sequence for $t$(<$l>)?  {}
        impl $(<$l>)? TextOrNumber for $t$(<$l>)?  {}
        impl $(<$l>)? Scalar for $t$(<$l>)?  {}
        impl $(<$l>)? Field for $t$(<$l>)?  {}
        impl $(<$l>)? DateTimeParam for $t$(<$l>)?  {}
    };
    (__IMPL $t:ident : $($l:lifetime)? $tname0:tt $($tname:tt)*) => {
        impl <$($l, )?$tname0: Any $(,$tname: Any)*> Number for $t<$($l, )?$tname0 $(,$tname)*> {}
        impl <$($l, )?$tname0: Any $(,$tname: Any)*> Text for $t<$($l, )?$tname0 $(,$tname)*> {}
        impl <$($l, )?$tname0: Any $(,$tname: Any)*> Logical for $t<$($l, )?$tname0 $(,$tname)*> {}
        impl <$($l, )?$tname0: Any $(,$tname: Any)*> Reference for $t<$($l, )?$tname0 $(,$tname)*> {}
        impl <$($l, )?$tname0: Any $(,$tname: Any)*> Matrix for $t<$($l, )?$tname0 $(,$tname)*> {}
        impl <$($l, )?$tname0: Any $(,$tname: Any)*> Array for $t<$($l, )?$tname0 $(,$tname)*> {}
        impl <$($l, )?$tname0: Any $(,$tname: Any)*> Database for $t<$($l, )?$tname0 $(,$tname)*> {}
        impl <$($l, )?$tname0: Any $(,$tname: Any)*> Criteria for $t<$($l, )?$tname0 $(,$tname)*> {}
        impl <$($l, )?$tname0: Any $(,$tname: Any)*> Sequence for $t<$($l, )?$tname0 $(,$tname)*> {}
        impl <$($l, )?$tname0: Any $(,$tname: Any)*> TextOrNumber for $t<$($l, )?$tname0 $(,$tname)*> {}
        impl <$($l, )?$tname0: Any $(,$tname: Any)*> Scalar for $t<$($l, )?$tname0 $(,$tname)*> {}
        impl <$($l, )?$tname0: Any $(,$tname: Any)*> Field for $t<$($l, )?$tname0 $(,$tname)*> {}
        impl <$($l, )?$tname0: Any $(,$tname: Any)*> DateTimeParam for $t<$($l, )?$tname0 $(,$tname)*> {}
    };
}

fn_reference!(VAL ValReference);
fn_reference!(OP OpReference);
fn_reference!(VAR FnReferenceVar);
fn_reference!(FnReference0);
fn_reference!(FnReference1: A);
fn_reference!(FnReference2: A B 2);
fn_reference!(FnReference3: A B 2 C 3);
fn_reference!(FnReference4: A B 2 C 3 D 4);
fn_reference!(FnReference5: A B 2 C 3 D 4 E 5);
fn_reference!(FnReference6: A B 2 C 3 D 4 E 5 F 6);

macro_rules! fn_array {
    (VAL $t:ident) => {
        any_struct!(VAL $t);
        fn_array!(__IMPL $t: A);
    };
    (OP $t:ident) => {
        any_struct!(OP $t);
        fn_array!(__IMPL $t: A B);
    };
    (VAR $t:ident) => {
        any_struct!(VAR $t);
        fn_array!(__IMPL $t:);
    };
    (REFVAR $t:ident) => {
        any_struct!(REFVAR $t);
        fn_array!(__IMPL $t: 'a);
    };
    ($t:ident) => {
        any_struct!($t);
        fn_array!(__IMPL $t:);
    };
    ($t:ident : $tname0:tt $($tname:tt $tidx:tt)*) => {
        any_struct!($t: $tname0 $($tname $tidx)*);
        fn_array!(__IMPL $t: $tname0 $($tname)*);
    };
    (__IMPL $t:ident : $($l:lifetime)?) => {
        impl $(<$l>)? Array for $t$(<$l>)?  {}
    };
    (__IMPL $t:ident : $($l:lifetime)? $tname0:tt $($tname:tt)*) => {
        impl <$($l, )?$tname0: Any $(,$tname: Any)*> Array for $t<$($l, )?$tname0 $(,$tname)*> {}
    };
}

fn_array!(VAL ValArray);
fn_array!(OP OpArray);
fn_array!(VAR FnArrayVar);
fn_array!(FnArray0);
fn_array!(FnArray1: A);
fn_array!(FnArray2: A B 2);
fn_array!(FnArray3: A B 2 C 3);
fn_array!(FnArray4: A B 2 C 3 D 4);
fn_array!(FnArray5: A B 2 C 3 D 4 E 5);
fn_array!(FnArray6: A B 2 C 3 D 4 E 5 F 6);

// -----------------------------------------------------------------------

impl Any for Vec<Box<dyn Any>> {
    #[inline]
    fn formula(&self, buf: &mut String) {
        for (i, v) in self.iter().enumerate() {
            if i > 0 {
                buf.push(';');
            }
            let _ = v.formula(buf);
        }
    }
}

impl Sequence for Vec<Box<dyn Any>> {}

macro_rules! tup {
    ( $tname0:ident $($tname:tt $tnum:tt)* ) => {

        impl<$tname0: Any, $($tname: Any,)*> Any for ($tname0, $($tname,)*) {
            #[inline]
            fn formula(&self, buf: &mut String) {
                self.0.formula(buf);
                $(
                    buf.push(';');
                    self.$tnum.formula(buf);
                )*
            }
        }

        impl<$tname0: Any + 'static, $($tname: Any + 'static,)*> Sequence for ($tname0, $($tname,)*) {}

    }
}

impl Any for () {
    #[inline]
    fn formula(&self, _buf: &mut String) {}
}
impl Sequence for () {}

tup!(A);
tup!(A B 1 );
tup!(A B 1 C 2 );
tup!(A B 1 C 2 D 3);
tup!(A B 1 C 2 D 3 E 4);
tup!(A B 1 C 2 D 3 E 4 F 5);
tup!(A B 1 C 2 D 3 E 4 F 5 G 6);
tup!(A B 1 C 2 D 3 E 4 F 5 G 6 H 7);
tup!(A B 1 C 2 D 3 E 4 F 5 G 6 H 7 I 8);
tup!(A B 1 C 2 D 3 E 4 F 5 G 6 H 7 I 8 J 9);
tup!(A B 1 C 2 D 3 E 4 F 5 G 6 H 7 I 8 J 9 K 10);
tup!(A B 1 C 2 D 3 E 4 F 5 G 6 H 7 I 8 J 9 K 10 L 11);
tup!(A B 1 C 2 D 3 E 4 F 5 G 6 H 7 I 8 J 9 K 10 L 11 M 12);
tup!(A B 1 C 2 D 3 E 4 F 5 G 6 H 7 I 8 J 9 K 10 L 11 M 12 N 13);
tup!(A B 1 C 2 D 3 E 4 F 5 G 6 H 7 I 8 J 9 K 10 L 11 M 12 N 13 O 14);
tup!(A B 1 C 2 D 3 E 4 F 5 G 6 H 7 I 8 J 9 K 10 L 11 M 12 N 13 O 14 P 15 );
tup!(A B 1 C 2 D 3 E 4 F 5 G 6 H 7 I 8 J 9 K 10 L 11 M 12 N 13 O 14 P 15 Q 16);
tup!(A B 1 C 2 D 3 E 4 F 5 G 6 H 7 I 8 J 9 K 10 L 11 M 12 N 13 O 14 P 15 Q 16 R 17);
tup!(A B 1 C 2 D 3 E 4 F 5 G 6 H 7 I 8 J 9 K 10 L 11 M 12 N 13 O 14 P 15 Q 16 R 17 S 18);
tup!(A B 1 C 2 D 3 E 4 F 5 G 6 H 7 I 8 J 9 K 10 L 11 M 12 N 13 O 14 P 15 Q 16 R 17 S 18 T 19);
tup!(A B 1 C 2 D 3 E 4 F 5 G 6 H 7 I 8 J 9 K 10 L 11 M 12 N 13 O 14 P 15 Q 16 R 17 S 18 T 19 U 20);

// -----------------------------------------------------------------------

/// Filter criteria.
#[derive(Debug)]
pub enum CriterionCmp {
    Cmp,
    Eq,
    Ne,
    Lt,
    Gt,
    LtEq,
    GtEq,
}

impl Display for CriterionCmp {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            CriterionCmp::Cmp => write!(f, ""),
            CriterionCmp::Eq => write!(f, "="),
            CriterionCmp::Ne => write!(f, "<>"),
            CriterionCmp::Lt => write!(f, "<"),
            CriterionCmp::Gt => write!(f, ">"),
            CriterionCmp::LtEq => write!(f, "<="),
            CriterionCmp::GtEq => write!(f, ">="),
        }
    }
}

/// Filter/search
#[derive(Debug)]
pub struct FCriterion<A: Any>(CriterionCmp, A);
impl<A: Any> FCriterion<A> {
    #[inline]
    pub fn cmp(f: A) -> Self {
        Self(CriterionCmp::Cmp, f)
    }

    #[inline]
    pub fn eq(f: A) -> Self {
        Self(CriterionCmp::Eq, f)
    }

    #[inline]
    pub fn ne(f: A) -> Self {
        Self(CriterionCmp::Ne, f)
    }

    #[inline]
    pub fn lt(f: A) -> Self {
        Self(CriterionCmp::Lt, f)
    }

    #[inline]
    pub fn gt(f: A) -> Self {
        Self(CriterionCmp::Gt, f)
    }

    #[inline]
    pub fn lte(f: A) -> Self {
        Self(CriterionCmp::LtEq, f)
    }

    #[inline]
    pub fn gte(f: A) -> Self {
        Self(CriterionCmp::GtEq, f)
    }
}

impl<A: Any> Any for FCriterion<A> {
    #[inline]
    fn formula(&self, buf: &mut String) {
        let _ = write!(buf, "\"{}\"", self.0);
        buf.push('&');
        self.1.formula(buf);
    }
}
impl<A: Any> Criterion for FCriterion<A> {}

impl<A: Any> Any for (CriterionCmp, A) {
    #[inline]
    fn formula(&self, buf: &mut String) {
        let _ = write!(buf, "\"{}\"", self.0);
        buf.push('&');
        self.1.formula(buf);
    }
}
impl<A: Any> Criterion for (CriterionCmp, A) {}

// -----------------------------------------------------------------------

pub struct FMatrix<T: Any, const N: usize, const M: usize>(pub [[T; M]; N]);

impl<T: Any, const N: usize, const M: usize> Any for FMatrix<T, N, M> {
    #[inline]
    fn formula(&self, buf: &mut String) {
        buf.push('{');
        for (i, r) in self.0.iter().enumerate() {
            if i > 0 {
                buf.push('|');
            }
            for (j, v) in r.iter().enumerate() {
                if j > 0 {
                    buf.push(';');
                }
                v.formula(buf);
            }
        }
        buf.push('}');
    }
}

impl<T: Any, const N: usize, const M: usize> Matrix for FMatrix<T, N, M> {}

pub struct FArray<T: Any, const N: usize>(pub [T; N]);

impl<T: Any, const N: usize> Any for FArray<T, N> {
    #[inline]
    fn formula(&self, buf: &mut String) {
        buf.push('{');
        for (i, v) in self.0.iter().enumerate() {
            if i > 0 {
                buf.push(';');
            }
            v.formula(buf);
        }
        buf.push('}');
    }
}

impl<T: Any, const N: usize> Array for FArray<T, N> {}

// -----------------------------------------------------------------------

impl<T: Any + ?Sized> Any for Box<T> {
    #[inline]
    fn formula(&self, buf: &mut String) {
        self.as_ref().formula(buf);
    }
}
impl<T: Number + Any + ?Sized> Number for Box<T> {}
impl<T: Text + Any + ?Sized> Text for Box<T> {}
impl<T: Logical + Any + ?Sized> Logical for Box<T> {}
impl<T: Reference + Any + ?Sized> Reference for Box<T> {}
impl<T: Matrix + Any + ?Sized> Matrix for Box<T> {}
impl<T: Array + Any + ?Sized> Array for Box<T> {}
impl<T: Database + Any + ?Sized> Database for Box<T> {}
impl<T: Criterion + Any + ?Sized> Criterion for Box<T> {}
impl<T: Criteria + Any + ?Sized> Criteria for Box<T> {}
impl<T: Sequence + Any + ?Sized + 'static> Sequence for Box<T> {}
impl<T: TextOrNumber + Any + ?Sized> TextOrNumber for Box<T> {}
impl<T: Scalar + Any + ?Sized> Scalar for Box<T> {}
impl<T: Field + Any + ?Sized> Field for Box<T> {}
impl<T: DateTimeParam + Any + ?Sized> DateTimeParam for Box<T> {}

impl<T: Any + Sized> Any for Option<T> {
    #[inline]
    fn formula(&self, buf: &mut String) {
        if let Some(v) = self {
            v.formula(buf);
        }
    }
}

// impl<T: Number + Any + Sized> Number for Option<T> {}
// impl<T: Text + Any + Sized> Text for Option<T> {}
// impl<T: Logical + Any + Sized> Logical for Option<T> {}
// impl<T: Reference + Any + Sized> Reference for Option<T> {}
// impl<T: Matrix + Any + Sized> Matrix for Option<T> {}
// impl<T: Array + Any + Sized> Array for Option<T> {}
// impl<T: Database + Any + Sized> Database for Option<T> {}
// impl<T: Criterion + Any + Sized> Criterion for Option<T> {}
// impl<T: Criteria + Any + Sized> Criteria for Option<T> {}
// impl<T: Sequence + Any + Sized> Sequence for Option<T> {}
// impl<T: TextOrNumber + Any + Sized> TextOrNumber for Option<T> {}
// impl<T: Scalar + Any + Sized> Scalar for Option<T> {}
// impl<T: Field + Any + Sized> Field for Option<T> {}
// impl<T: DateTimeParam + Any + Sized> DateTimeParam for Option<T> {}

// -----------------------------------------------------------------------

/// An expression in parentheses.
/// Use p() / parentheses() to create one.
#[derive(Debug)]
pub struct FParentheses<A>(A);
impl<A: Any> Any for FParentheses<A> {
    #[inline]
    fn formula(&self, buf: &mut String) {
        buf.push('(');
        self.0.formula(buf);
        buf.push(')');
    }
}
impl<A: Number> Number for FParentheses<A> {}
impl<A: Text> Text for FParentheses<A> {}
impl<A: Logical> Logical for FParentheses<A> {}
impl<A: Reference> Reference for FParentheses<A> {}
impl<A: Matrix> Matrix for FParentheses<A> {}
impl<A: Array> Array for FParentheses<A> {}
impl<A: Database> Database for FParentheses<A> {}
impl<A: Criteria> Criteria for FParentheses<A> {}
impl<A: Sequence> Sequence for FParentheses<A> {}
impl<A: TextOrNumber> TextOrNumber for FParentheses<A> {}
impl<A: Scalar> Scalar for FParentheses<A> {}
impl<A: Field> Field for FParentheses<A> {}
impl<A: DateTimeParam> DateTimeParam for FParentheses<A> {}

/// Creates an expression in parentheses.
#[inline]
pub fn p<A: Any>(a: A) -> FParentheses<A> {
    FParentheses(a)
}

// -----------------------------------------------------------------------

macro_rules! value_number {
    ($t:ty) => {
        impl Any for $t {
            #[inline]
            fn formula(&self, buf: &mut String) {
                let _ = write!(buf, "{}", self);
            }
        }
        impl Number for $t {}
        impl Logical for $t {}
        impl Sequence for $t {}
        impl TextOrNumber for $t {}
        impl Scalar for $t {}
        impl Field for $t {}
        impl DateTimeParam for $t {}
    };
}

value_number!(i8);
value_number!(i16);
value_number!(i32);
value_number!(i64);
value_number!(i128);
value_number!(isize);
value_number!(u8);
value_number!(u16);
value_number!(u32);
value_number!(u64);
value_number!(u128);
value_number!(usize);
value_number!(f32);
value_number!(f64);

#[inline]
pub fn num<A: Number>(n: A) -> ValNumber<A> {
    ValNumber(n)
}

impl Any for bool {
    #[inline]
    fn formula(&self, buf: &mut String) {
        buf.push_str(if *self { "TRUE()" } else { "FALSE()" });
    }
}
impl Number for bool {}
impl Logical for bool {}
impl Sequence for bool {}
impl Scalar for bool {}

impl Any for &str {
    #[inline]
    fn formula(&self, buf: &mut String) {
        if self.contains('"') {
            buf.push('"');
            for (i, s) in self.split('"').enumerate() {
                if i > 0 {
                    buf.push_str("\"\"");
                }
                buf.push_str(s);
            }
            buf.push('"');
        } else {
            buf.push('"');
            buf.push_str(self);
            buf.push('"');
        }
    }
}
impl Text for &str {}
impl Sequence for &str {}
impl TextOrNumber for &str {}
impl Scalar for &str {}
impl Field for &str {}
impl DateTimeParam for &str {}

impl<'a> Any for Cow<'a, str> {
    #[inline]
    fn formula(&self, buf: &mut String) {
        let s: &str = self.borrow();
        s.formula(buf)
    }
}
impl<'a> Text for Cow<'a, str> {}
impl<'a> Sequence for Cow<'a, str> {}
impl<'a> TextOrNumber for Cow<'a, str> {}
impl<'a> Scalar for Cow<'a, str> {}
impl<'a> Field for Cow<'a, str> {}
impl<'a> DateTimeParam for Cow<'a, str> {}

impl Any for String {
    #[inline]
    fn formula(&self, buf: &mut String) {
        self.as_str().formula(buf)
    }
}
impl Text for String {}
impl Sequence for String {}
impl TextOrNumber for String {}
impl Scalar for String {}
impl Field for String {}
impl DateTimeParam for String {}

impl Any for CellRef {
    #[inline]
    fn formula(&self, buf: &mut String) {
        buf.push_str(self.to_formula().as_str())
    }
}
impl Number for CellRef {}
impl Text for CellRef {}
impl Logical for CellRef {}
impl Reference for CellRef {}
impl Matrix for CellRef {}
impl Array for CellRef {}
impl Database for CellRef {}
impl Criteria for CellRef {}
impl Sequence for CellRef {}
impl TextOrNumber for CellRef {}
impl Scalar for CellRef {}
impl Field for CellRef {}
impl DateTimeParam for CellRef {}

impl Any for CellRange {
    #[inline]
    fn formula(&self, buf: &mut String) {
        buf.push_str(self.to_formula().as_str())
    }
}
impl Number for CellRange {}
impl Text for CellRange {}
impl Logical for CellRange {}
impl Reference for CellRange {}
impl Matrix for CellRange {}
impl Array for CellRange {}
impl Database for CellRange {}
impl Criteria for CellRange {}
impl Sequence for CellRange {}
impl TextOrNumber for CellRange {}
impl Scalar for CellRange {}
impl Field for CellRange {}
impl DateTimeParam for CellRange {}

// -----------------------------------------------------------------------

macro_rules! number_op {
    ($t:ident $(< $($l:lifetime $(,)? )? $($tname:ident $(,)?)* >)?) => {
        impl <$($($l,)? $($tname: Any,)*)? V: Number> Add<V> for $t $(< $($l,)? $($tname,)* >)? {
            type Output = OpNumber<Self, V>;

            #[inline]
            fn add(self, rhs: V) -> Self::Output {
                OpNumber(self, "+", rhs)
            }
        }

        impl <$($($l,)? $($tname: Any,)*)? V: Number> Sub<V> for $t $(< $($l,)? $($tname,)* >)? {
            type Output = OpNumber<Self, V>;

            #[inline]
            fn sub(self, rhs: V) -> Self::Output {
                OpNumber(self, "-", rhs)
            }
        }

        impl <$($($l,)? $($tname: Any,)*)? V: Number> Mul<V> for $t $(< $($l,)? $($tname,)* >)? {
            type Output = OpNumber<Self, V>;

            #[inline]
            fn mul(self, rhs: V) -> Self::Output {
                OpNumber(self, "*", rhs)
            }
        }

        impl <$($($l,)? $($tname: Any,)*)? V: Number> Div<V> for $t $(< $($l,)? $($tname,)* >)? {
            type Output = OpNumber<Self, V>;

            #[inline]
            fn div(self, rhs: V) -> Self::Output {
                OpNumber(self, "/", rhs)
            }
        }

        impl <$($($l,)? $($tname: Any,)*)? V: Number> BitXor<V> for $t $(< $($l,)? $($tname,)* >)? {
            type Output = OpNumber<Self, V>;

            #[inline]
            fn bitxor(self, rhs: V) -> Self::Output {
                OpNumber(self, "^", rhs)
            }
        }

        impl <$($($l,)? $($tname: Any,)*)?> Neg for $t $(< $($l,)? $($tname,)* >)? {
            type Output = OpNumber<(), Self>;

            #[inline]
            fn neg(self) -> Self::Output {
                OpNumber((), "-", self)
            }
        }

    };
}

number_op!(ValAny<A>);
number_op!(OpAny<A, B>);
number_op!(FnAnyVar);
number_op!(FnAny0);
number_op!(FnAny1<A>);
number_op!(FnAny2<A, B>);
number_op!(FnAny3<A, B, C>);
number_op!(FnAny4<A, B, C, D>);
number_op!(FnAny5<A, B, C, D, E>);
number_op!(FnAny6<A, B, C, D, E, F>);

number_op!(ValNumber<A>);
number_op!(OpNumber<A, B>);
number_op!(FnNumberVar);
number_op!(FnNumber0);
number_op!(FnNumber1<A>);
number_op!(FnNumber2<A, B>);
number_op!(FnNumber3<A, B, C>);
number_op!(FnNumber4<A, B, C, D>);
number_op!(FnNumber5<A, B, C, D, E>);
number_op!(FnNumber6<A, B, C, D, E, F>);
number_op!(FnNumber7<A, B, C, D, E, F, G>);

number_op!(ValLogical<A>);
number_op!(OpLogical<A, B>);
number_op!(FnLogicalVar);
number_op!(FnLogical0);
number_op!(FnLogical1<A>);
number_op!(FnLogical2<A, B>);
number_op!(FnLogical3<A, B, C>);
number_op!(FnLogical4<A, B, C, D>);
number_op!(FnLogical5<A, B, C, D, E>);
number_op!(FnLogical6<A, B, C, D, E, F>);

number_op!(ValReference<A>);
number_op!(OpReference<A, B>);
number_op!(FnReferenceVar);
number_op!(FnReference0);
number_op!(FnReference1<A>);
number_op!(FnReference2<A, B>);
number_op!(FnReference3<A, B, C>);
number_op!(FnReference4<A, B, C, D>);
number_op!(FnReference5<A, B, C, D, E>);
number_op!(FnReference6<A, B, C, D, E, F>);

number_op!(FParentheses<A>);

// -----------------------------------------------------------------------

macro_rules! text_op {
    ($t:ident $(< $($l:lifetime $(,)? )? $($tname:ident $(,)?)* >)?) => {
        impl <$($($l,)? $($tname: Any,)*)? V: Text> BitAnd<V> for $t $(< $($l,)? $($tname,)* >)? {
            type Output = OpText<Self, V>;

            #[inline]
            fn bitand(self, rhs: V) -> Self::Output {
                OpText(self, "&", rhs)
            }
        }
    }
}

text_op!(ValText<A>);
text_op!(OpText<A, B>);
text_op!(FnTextVar);
text_op!(FnText0);
text_op!(FnText1<A>);
text_op!(FnText2<A, B>);
text_op!(FnText3<A, B, C>);
text_op!(FnText4<A, B, C, D>);
text_op!(FnText5<A, B, C, D, E>);
text_op!(FnText6<A, B, C, D, E, F>);

// -----------------------------------------------------------------------

/// Creates a formula from any formula expression.
#[inline]
pub fn formula<T: Any>(f: T) -> String {
    let mut buf = String::new();
    buf.push_str("of:=");
    let _ = f.formula(&mut buf);
    buf
}
