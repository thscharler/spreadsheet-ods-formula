//!
//! This library provides functions that map to ODS-functions.
//!
//! ```
//! use spreadsheet_ods::CellRef;
//! use spreadsheet_ods_formula::{cell, formula, of};
//!
//! let f = formula(of::sin(cell!(0,0)));
//!
//! assert_eq!(f, "of:=SIN([.A1])");
//! ```
//!
//! All functions use traits to somewhat constrain the allowed parameter-types. There are quite a
//! lot of traits to map the possible parameter types of ods-functions. By the nature of
//! spreadsheets this is a rather loose mapping anyway.
//!
//! * Basic types ixx, uxx, bool, str, String and `Cow<str>` have the appropriate traits.
//!   The common operators are overloaded too. With the caveat that the first parameter
//!   must be created via the num() function.
//!
//! * Expressions in parentheses must use the p()-function.
//!
//! ```
//! use spreadsheet_ods_formula::{formula, num, of, p};
//!
//! let f = formula(num(33) * p(of::sqrt(2) + of::sqrt(3)));
//! ```
//!
//! * CellRef and CellRange are imported from spreadsheet-ods. There is also a cell!() macro.
//!
//! ```
//! use spreadsheet_ods_formula::{cell, formula};
//!
//! let f = formula(cell!(0, 0));
//! println!("{}", f);
//! let f = formula(cell!("Table1" => 4, 4));
//! println!("{}", f);
//! let f = formula(cell!(0, 0, 4, 4));
//! println!("{}", f);
//! ```
//!
//! * ODS operators are mapped with the AnyOp, TextOp, NumberOp, LogicalOp and ReferenceOp traits.
//!
//! ```
//! use spreadsheet_ods_formula::{cell, formula};
//! use spreadsheet_ods_formula::{ReferenceOp};
//!
//! let f = formula(cell!(0, 0, 4, 4).refcat(cell!(8, 8, 12, 12)));
//! println!("{}", f);
//! ```
//!
//! * Tuples are used for Sequences. They are formatted as inline-arrays.
//!
//! ```
//! use spreadsheet_ods_formula::{cell, formula, of};
//!
//! let f = formula(of::networkdays__(
//!      cell!(5, 5),
//!      cell!(9, 9),
//!      (9, 9, 9),
//!      (0, 0, 0, 0, 0, 1, 0),
//! ));
//! ```
//!
//! * FMatrix and FArray for inline arrays and matrizes.
//!
//! ```
//! use spreadsheet_ods_formula::{cell, FArray, formula, of};
//!
//! let f = formula(of::networkdays__(
//!      cell!(5, 5),
//!      cell!(9, 9),
//!      FArray([9, 9, 9]),
//!      FArray([0, 0, 0, 0, 0, 1, 0]),
//! ));
//! ```

#![allow(clippy::too_many_arguments)]
#![warn(absolute_paths_not_starting_with_crate)]
// NO #![warn(box_pointers)]
#![warn(elided_lifetimes_in_paths)]
#![warn(explicit_outlives_requirements)]
#![warn(keyword_idents)]
#![warn(macro_use_extern_crate)]
#![warn(meta_variable_misuse)]
#![warn(missing_abi)]
// NOT_ACCURATE #![warn(missing_copy_implementations)]
// #![warn(missing_debug_implementations)]
// #![warn(missing_docs)]
// NO #![warn(non_ascii_idents)]
#![warn(noop_method_call)]
// NO #![warn(or_patterns_back_compat)]
#![warn(pointer_structural_match)]
#![warn(semicolon_in_expressions_from_macros)]
// NOT_ACCURATE #![warn(single_use_lifetimes)]
#![warn(trivial_casts)]
#![warn(trivial_numeric_casts)]
#![warn(unreachable_pub)]
// #![warn(unsafe_code)]
#![warn(unsafe_op_in_unsafe_fn)]
#![warn(unstable_features)]
// NO #![warn(unused_crate_dependencies)]
// NO #![warn(unused_extern_crates)]
#![warn(unused_import_braces)]
#![warn(unused_lifetimes)]
#![warn(unused_qualifications)]
// NO #![warn(unused_results)]
#![warn(variant_size_differences)]

use spreadsheet_ods::{CellRange, CellRef};
use std::borrow::Borrow;
use std::borrow::Cow;
use std::fmt::{Display, Formatter, Write};
use std::ops::{Add, BitAnd, BitXor, Div, Mul, Neg, Sub};

mod generated;

pub mod cmp;
pub mod op;

pub mod bit;
pub mod complex;
pub mod conv;
pub mod date;
pub mod db;
pub mod ext;
pub mod fin;
pub mod info;
pub mod logic;
pub mod lookup;
pub mod math;
pub mod matrix;
pub mod round;
pub mod stat;
pub mod text;
pub mod textb;

/// The traits for this crate.
/// And the function p() for parentheses.
pub mod prelude {
    pub use crate::Any;
    pub use crate::{AnyOp, LogicalOp, NumberOp, ReferenceOp, TextOp};
}

/// All functions in one module, and also all families of functions.
pub mod of {
    #[doc(inline)]
    pub use crate::cmp::*;
    #[doc(inline)]
    pub use crate::op::*;

    #[doc(inline)]
    pub use crate::bit::*;
    #[doc(inline)]
    pub use crate::complex::*;
    #[doc(inline)]
    pub use crate::conv::*;
    #[doc(inline)]
    pub use crate::date::*;
    #[doc(inline)]
    pub use crate::db::*;
    #[doc(inline)]
    pub use crate::ext::*;
    #[doc(inline)]
    pub use crate::fin::*;
    #[doc(inline)]
    pub use crate::info::*;
    #[doc(inline)]
    pub use crate::logic::*;
    #[doc(inline)]
    pub use crate::lookup::*;
    #[doc(inline)]
    pub use crate::math::*;
    #[doc(inline)]
    pub use crate::matrix::*;
    #[doc(inline)]
    pub use crate::round::*;
    #[doc(inline)]
    pub use crate::stat::*;
    #[doc(inline)]
    pub use crate::text::*;
    #[doc(inline)]
    pub use crate::textb::*;

    #[doc(inline)]
    pub use crate::{
        bit, complex, conv, date, db, ext, fin, info, logic, lookup, math, matrix, round, stat,
        text, textb,
    };
    #[doc(inline)]
    pub use crate::{cmp, op};
}

// -----------------------------------------------------------------------

/// Trait for any part of the formula.
///
/// This trait is used to create the string-repr of the formula.
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
/// A date/time parameter.
pub trait DateTime: Any {}
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
/// A single scalar value.
pub trait Scalar: Any {}
/// A field denominator for a db.
pub trait Field: Any {}

/// Text or Number parameter.
pub trait TextOrNumber: Any {}
/// Reference or Array parameter.
pub trait ReferenceOrArray: Any {}
/// Reference or Text parameter.
pub trait TextOrReference: Any {}
/// Number or Array parameter.
pub trait NumberOrArray: Any {}

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
        #[doc(hidden)]
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
        #[doc(hidden)]
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
        #[doc(hidden)]
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
        #[doc(hidden)]
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
        #[doc(hidden)]
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
        impl $(<$l>)? Scalar for $t$(<$l>)?  {}
        impl $(<$l>)? Field for $t$(<$l>)?  {}
        impl $(<$l>)? DateTime for $t$(<$l>)?  {}
        impl $(<$l>)? TextOrNumber for $t$(<$l>)?  {}
        impl $(<$l>)? TextOrReference for $t$(<$l>)?  {}
        impl $(<$l>)? NumberOrArray for $t$(<$l>)?  {}
    };
    (__IMPL $t:ident : $($l:lifetime)? $tname0:tt $($tname:tt)*) => {
        impl <$($l, )?$tname0: Any $(,$tname: Any)*> Number for $t<$($l, )?$tname0 $(,$tname)*> {}
        impl <$($l, )?$tname0: Any $(,$tname: Any)*> Text for $t<$($l, )?$tname0 $(,$tname)*> {}
        impl <$($l, )?$tname0: Any $(,$tname: Any)*> Logical for $t<$($l, )?$tname0 $(,$tname)*> {}
        impl <$($l, )?$tname0: Any $(,$tname: Any)*> Sequence for $t<$($l, )?$tname0 $(,$tname)*> {}
        impl <$($l, )?$tname0: Any $(,$tname: Any)*> Scalar for $t<$($l, )?$tname0 $(,$tname)*> {}
        impl <$($l, )?$tname0: Any $(,$tname: Any)*> Field for $t<$($l, )?$tname0 $(,$tname)*> {}
        impl <$($l, )?$tname0: Any $(,$tname: Any)*> DateTime for $t<$($l, )?$tname0 $(,$tname)*> {}
        impl <$($l, )?$tname0: Any $(,$tname: Any)*> TextOrNumber for $t<$($l, )?$tname0 $(,$tname)*> {}
        impl <$($l, )?$tname0: Any $(,$tname: Any)*> TextOrReference for $t<$($l, )?$tname0 $(,$tname)*> {}
        impl <$($l, )?$tname0: Any $(,$tname: Any)*> NumberOrArray for $t<$($l, )?$tname0 $(,$tname)*> {}
    };
}

// fn_any!(VAL ValAny);
// fn_any!(OP OpAny);
fn_any!(VAR FnAnyVar);
fn_any!(FnAny0);
fn_any!(FnAny1: A);
fn_any!(FnAny2: A B 2);
fn_any!(FnAny3: A B 2 C 3);
fn_any!(FnAny4: A B 2 C 3 D 4);
fn_any!(FnAny5: A B 2 C 3 D 4 E 5);

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
        impl $(<$l>)? Scalar for $t$(<$l>)?  {}
        impl $(<$l>)? Field for $t$(<$l>)?  {}
        impl $(<$l>)? DateTime for $t$(<$l>)?  {}
        impl $(<$l>)? TextOrNumber for $t$(<$l>)?  {}
        impl $(<$l>)? NumberOrArray for $t$(<$l>)?  {}
    };
    (__IMPL $t:ident : $($l:lifetime)? $tname0:tt $($tname:tt)*) => {
        impl <$($l, )?$tname0: Any $(,$tname: Any)*> Number for $t<$($l, )?$tname0 $(,$tname)*> {}
        impl <$($l, )?$tname0: Any $(,$tname: Any)*> Logical for $t<$($l, )?$tname0 $(,$tname)*> {}
        impl <$($l, )?$tname0: Any $(,$tname: Any)*> Sequence for $t<$($l, )?$tname0 $(,$tname)*> {}
        impl <$($l, )?$tname0: Any $(,$tname: Any)*> Scalar for $t<$($l, )?$tname0 $(,$tname)*> {}
        impl <$($l, )?$tname0: Any $(,$tname: Any)*> Field for $t<$($l, )?$tname0 $(,$tname)*> {}
        impl <$($l, )?$tname0: Any $(,$tname: Any)*> DateTime for $t<$($l, )?$tname0 $(,$tname)*> {}
        impl <$($l, )?$tname0: Any $(,$tname: Any)*> TextOrNumber for $t<$($l, )?$tname0 $(,$tname)*> {}
        impl <$($l, )?$tname0: Any $(,$tname: Any)*> NumberOrArray for $t<$($l, )?$tname0 $(,$tname)*> {}
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
fn_number!(FnNumber8: A B 2 C 3 D 4 E 5 F 6 G 7 H 8);
fn_number!(FnNumber9: A B 2 C 3 D 4 E 5 F 6 G 7 H 8 I 9);

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
        impl $(<$l>)? Scalar for $t$(<$l>)?  {}
        impl $(<$l>)? Field for $t$(<$l>)?  {}
        impl $(<$l>)? DateTime for $t$(<$l>)?  {}
        impl $(<$l>)? TextOrNumber for $t$(<$l>)?  {}
        impl $(<$l>)? TextOrReference for $t$(<$l>)?  {}
    };
    (__IMPL $t:ident : $($l:lifetime)? $tname0:tt $($tname:tt)*) => {
        impl <$($l, )?$tname0: Any $(,$tname: Any)*> Text for $t<$($l, )?$tname0 $(,$tname)*> {}
        impl <$($l, )?$tname0: Any $(,$tname: Any)*> Sequence for $t<$($l, )?$tname0 $(,$tname)*> {}
        impl <$($l, )?$tname0: Any $(,$tname: Any)*> Scalar for $t<$($l, )?$tname0 $(,$tname)*> {}
        impl <$($l, )?$tname0: Any $(,$tname: Any)*> Field for $t<$($l, )?$tname0 $(,$tname)*> {}
        impl <$($l, )?$tname0: Any $(,$tname: Any)*> DateTime for $t<$($l, )?$tname0 $(,$tname)*> {}
        impl <$($l, )?$tname0: Any $(,$tname: Any)*> TextOrNumber for $t<$($l, )?$tname0 $(,$tname)*> {}
        impl <$($l, )?$tname0: Any $(,$tname: Any)*> TextOrReference for $t<$($l, )?$tname0 $(,$tname)*> {}
    };
}

// fn_text!(VAL ValText);
fn_text!(OP OpText);
fn_text!(VAR FnTextVar);
// fn_text!(FnText0);
fn_text!(FnText1: A);
fn_text!(FnText2: A B 2);
fn_text!(FnText3: A B 2 C 3);
fn_text!(FnText4: A B 2 C 3 D 4);
fn_text!(FnText5: A B 2 C 3 D 4 E 5);

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
        impl $(<$l>)? Scalar for $t$(<$l>)?  {}
        impl $(<$l>)? TextOrNumber for $t$(<$l>)?  {}
        impl $(<$l>)? NumberOrArray for $t$(<$l>)?  {}
    };
    (__IMPL $t:ident : $($l:lifetime)? $tname0:tt $($tname:tt)*) => {
        impl <$($l, )?$tname0: Any $(,$tname: Any)*> Number for $t<$($l, )?$tname0 $(,$tname)*> {}
        impl <$($l, )?$tname0: Any $(,$tname: Any)*> Logical for $t<$($l, )?$tname0 $(,$tname)*> {}
        impl <$($l, )?$tname0: Any $(,$tname: Any)*> Sequence for $t<$($l, )?$tname0 $(,$tname)*> {}
        impl <$($l, )?$tname0: Any $(,$tname: Any)*> Scalar for $t<$($l, )?$tname0 $(,$tname)*> {}
        impl <$($l, )?$tname0: Any $(,$tname: Any)*> TextOrNumber for $t<$($l, )?$tname0 $(,$tname)*> {}
        impl <$($l, )?$tname0: Any $(,$tname: Any)*> NumberOrArray for $t<$($l, )?$tname0 $(,$tname)*> {}
    };
}

// fn_logical!(VAL ValLogical);
fn_logical!(OP OpLogical);
// fn_logical!(VAR FnLogicalVar);
fn_logical!(FnLogical0);
fn_logical!(FnLogical1: A);
fn_logical!(FnLogical2: A B 2);

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

// fn_matrix!(VAL ValMatrix);
// fn_matrix!(OP OpMatrix);
// fn_matrix!(VAR FnMatrixVar);
// fn_matrix!(FnMatrix0);
fn_matrix!(FnMatrix1: A);
fn_matrix!(FnMatrix2: A B 2);

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
        impl $(<$l>)? Scalar for $t$(<$l>)?  {}
        impl $(<$l>)? Field for $t$(<$l>)?  {}
        impl $(<$l>)? DateTime for $t$(<$l>)?  {}
        impl $(<$l>)? TextOrNumber for $t$(<$l>)?  {}
        impl $(<$l>)? ReferenceOrArray for $t$(<$l>)?  {}
        impl $(<$l>)? TextOrReference for $t$(<$l>)?  {}
        impl $(<$l>)? NumberOrArray for $t$(<$l>)?  {}
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
        impl <$($l, )?$tname0: Any $(,$tname: Any)*> Scalar for $t<$($l, )?$tname0 $(,$tname)*> {}
        impl <$($l, )?$tname0: Any $(,$tname: Any)*> Field for $t<$($l, )?$tname0 $(,$tname)*> {}
        impl <$($l, )?$tname0: Any $(,$tname: Any)*> DateTime for $t<$($l, )?$tname0 $(,$tname)*> {}
        impl <$($l, )?$tname0: Any $(,$tname: Any)*> TextOrNumber for $t<$($l, )?$tname0 $(,$tname)*> {}
        impl <$($l, )?$tname0: Any $(,$tname: Any)*> ReferenceOrArray for $t<$($l, )?$tname0 $(,$tname)*> {}
        impl <$($l, )?$tname0: Any $(,$tname: Any)*> TextOrReference for $t<$($l, )?$tname0 $(,$tname)*> {}
        impl <$($l, )?$tname0: Any $(,$tname: Any)*> NumberOrArray for $t<$($l, )?$tname0 $(,$tname)*> {}
    };
}

// fn_reference!(VAL ValReference);
fn_reference!(OP OpReference);
// fn_reference!(VAR FnReferenceVar);
// fn_reference!(FnReference0);
fn_reference!(FnReference1: A);
fn_reference!(FnReference2: A B 2);
fn_reference!(FnReference3: A B 2 C 3);
fn_reference!(FnReference4: A B 2 C 3 D 4);
fn_reference!(FnReference5: A B 2 C 3 D 4 E 5);

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
        impl $(<$l>)? ReferenceOrArray for $t$(<$l>)?  {}
        impl $(<$l>)? NumberOrArray for $t$(<$l>)?  {}
    };
    (__IMPL $t:ident : $($l:lifetime)? $tname0:tt $($tname:tt)*) => {
        impl <$($l, )?$tname0: Any $(,$tname: Any)*> Array for $t<$($l, )?$tname0 $(,$tname)*> {}
        impl <$($l, )?$tname0: Any $(,$tname: Any)*> ReferenceOrArray for $t<$($l, )?$tname0 $(,$tname)*> {}
        impl <$($l, )?$tname0: Any $(,$tname: Any)*> NumberOrArray for $t<$($l, )?$tname0 $(,$tname)*> {}
    };
}

// fn_array!(VAL ValArray);
// fn_array!(OP OpArray);
// fn_array!(VAR FnArrayVar);
// fn_array!(FnArray0);
fn_array!(FnArray1: A);
fn_array!(FnArray2: A B 2);
fn_array!(FnArray3: A B 2 C 3);
fn_array!(FnArray4: A B 2 C 3 D 4);

// -----------------------------------------------------------------------

macro_rules! tup {
    ( $tname0:ident $($tname:tt $tnum:tt)* ) => {

        impl<$tname0: Any, $($tname: Any,)*> Any for ($tname0, $($tname,)*) {
            #[inline]
            fn formula(&self, buf: &mut String) {
                buf.push('{');
                self.0.formula(buf);
                $(
                    buf.push(';');
                    self.$tnum.formula(buf);
                )*
                buf.push('}');
            }
        }

        impl<$tname0: Any + 'static, $($tname: Any + 'static,)*> Sequence for ($tname0, $($tname,)*) {}

    }
}

impl Any for () {
    #[inline]
    fn formula(&self, buf: &mut String) {
        buf.push('{');
        buf.push('}');
    }
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

/// Matrix value.
#[derive(Debug)]
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

/// Array.
#[derive(Debug)]
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
impl<T: Any, const N: usize> Sequence for FArray<T, N> {}
impl<T: Any, const N: usize> ReferenceOrArray for FArray<T, N> {}
impl<T: Any, const N: usize> NumberOrArray for FArray<T, N> {}

// -----------------------------------------------------------------------

impl<T: Any + Sized> Any for Option<T> {
    #[inline]
    fn formula(&self, buf: &mut String) {
        if let Some(v) = self {
            v.formula(buf);
        }
    }
}

impl<T: Number + Any + Sized> Number for Option<T> {}
impl<T: Text + Any + Sized> Text for Option<T> {}
impl<T: Logical + Any + Sized> Logical for Option<T> {}
impl<T: Reference + Any + Sized> Reference for Option<T> {}
impl<T: Matrix + Any + Sized> Matrix for Option<T> {}
impl<T: Array + Any + Sized> Array for Option<T> {}
impl<T: Database + Any + Sized> Database for Option<T> {}
impl<T: Criterion + Any + Sized> Criterion for Option<T> {}
impl<T: Criteria + Any + Sized> Criteria for Option<T> {}
impl<T: Sequence + Any + Sized> Sequence for Option<T> {}
impl<T: Scalar + Any + Sized> Scalar for Option<T> {}
impl<T: Field + Any + Sized> Field for Option<T> {}
impl<T: DateTime + Any + Sized> DateTime for Option<T> {}
impl<T: TextOrNumber + Any + Sized> TextOrNumber for Option<T> {}
impl<T: ReferenceOrArray + Any + Sized> ReferenceOrArray for Option<T> {}
impl<T: TextOrReference + Any + Sized> TextOrReference for Option<T> {}
impl<T: NumberOrArray + Any + Sized> NumberOrArray for Option<T> {}

// -----------------------------------------------------------------------

/// An expression in parentheses. Use p() to create one.
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
impl<A: Scalar> Scalar for FParentheses<A> {}
impl<A: Field> Field for FParentheses<A> {}
impl<A: DateTime> DateTime for FParentheses<A> {}
impl<A: TextOrNumber> TextOrNumber for FParentheses<A> {}
impl<A: ReferenceOrArray> ReferenceOrArray for FParentheses<A> {}
impl<A: TextOrReference> TextOrReference for FParentheses<A> {}
impl<A: NumberOrArray> NumberOrArray for FParentheses<A> {}

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
        impl Scalar for $t {}
        impl Field for $t {}
        impl DateTime for $t {}
        impl TextOrNumber for $t {}
        impl NumberOrArray for $t {}
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

/// Creates a formula-number from a rust number literal.
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
impl NumberOrArray for bool {}

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
impl Scalar for &str {}
impl Field for &str {}
impl DateTime for &str {}
impl TextOrNumber for &str {}
impl TextOrReference for &str {}

impl<'a> Any for Cow<'a, str> {
    #[inline]
    fn formula(&self, buf: &mut String) {
        let s: &str = self.borrow();
        s.formula(buf)
    }
}
impl<'a> Text for Cow<'a, str> {}
impl<'a> Sequence for Cow<'a, str> {}
impl<'a> Scalar for Cow<'a, str> {}
impl<'a> Field for Cow<'a, str> {}
impl<'a> DateTime for Cow<'a, str> {}
impl<'a> TextOrNumber for Cow<'a, str> {}
impl<'a> TextOrReference for Cow<'a, str> {}

impl Any for String {
    #[inline]
    fn formula(&self, buf: &mut String) {
        self.as_str().formula(buf)
    }
}
impl Text for String {}
impl Sequence for String {}
impl Scalar for String {}
impl Field for String {}
impl DateTime for String {}
impl TextOrNumber for String {}
impl TextOrReference for String {}

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
impl Scalar for CellRef {}
impl Field for CellRef {}
impl DateTime for CellRef {}
impl TextOrNumber for CellRef {}
impl ReferenceOrArray for CellRef {}
impl TextOrReference for CellRef {}
impl NumberOrArray for CellRef {}

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
impl Scalar for CellRange {}
impl Field for CellRange {}
impl DateTime for CellRange {}
impl TextOrNumber for CellRange {}
impl ReferenceOrArray for CellRange {}
impl TextOrReference for CellRange {}
impl NumberOrArray for CellRange {}

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

// number_op!(ValAny<A>);
// number_op!(OpAny<A, B>);
number_op!(FnAnyVar);
number_op!(FnAny0);
number_op!(FnAny1<A>);
number_op!(FnAny2<A, B>);
number_op!(FnAny3<A, B, C>);
number_op!(FnAny4<A, B, C, D>);
number_op!(FnAny5<A, B, C, D, E>);

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
number_op!(FnNumber8<A, B, C, D, E, F, G, H>);
number_op!(FnNumber9<A, B, C, D, E, F, G, H, I>);

// number_op!(ValLogical<A>);
number_op!(OpLogical<A, B>);
// number_op!(FnLogicalVar);
number_op!(FnLogical0);
number_op!(FnLogical1<A>);
number_op!(FnLogical2<A, B>);

// number_op!(ValReference<A>);
number_op!(OpReference<A, B>);
// number_op!(FnReferenceVar);
// number_op!(FnReference0);
number_op!(FnReference1<A>);
number_op!(FnReference2<A, B>);
number_op!(FnReference3<A, B, C>);
number_op!(FnReference4<A, B, C, D>);
number_op!(FnReference5<A, B, C, D, E>);

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

// text_op!(ValText<A>);
text_op!(OpText<A, B>);
text_op!(FnTextVar);
// text_op!(FnText0);
text_op!(FnText1<A>);
text_op!(FnText2<A, B>);
text_op!(FnText3<A, B, C>);
text_op!(FnText4<A, B, C, D>);
text_op!(FnText5<A, B, C, D, E>);

// -----------------------------------------------------------------------

/// Creates a formula from any formula expression.
#[inline]
pub fn formula<T: Any>(f: T) -> String {
    let mut buf = String::new();
    buf.push_str("of:=");
    f.formula(&mut buf);
    buf
}

// -----------------------------------------------------------------------

/// Macro for cell-references.
#[macro_export]
macro_rules! cell {
    ($row:expr, $col:expr) => {
        spreadsheet_ods::CellRef::local($row, $col)
    };
    ($row:expr, $col:expr, $row2:expr, $col2:expr) => {
        spreadsheet_ods::CellRange::local($row, $col, $row2, $col2)
    };
    ($table:expr => $row:expr, $col:expr) => {
        spreadsheet_ods::CellRef::remote($table, $row, $col)
    };
    ($table:expr => $row:expr, $col:expr, $row2:expr, $col2:expr) => {
        spreadsheet_ods::CellRange::remote($table, $row, $col, $row2, $col2)
    };
}
