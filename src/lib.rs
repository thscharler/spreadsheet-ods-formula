use spreadsheet_ods::{CellRange, CellRef};
use std::borrow::Cow;
use std::fmt::{Display, Formatter, Write};
use std::io::BufRead;
use std::mem::MaybeUninit;
use std::{alloc, mem, slice};

pub trait Any {
    fn formula(&self, buf: &mut String);
}

pub trait Number: Any {}
pub trait Text: Any {}
pub trait Logical: Any {}
pub trait Reference: Any {}
pub trait Matrix: Any {}
pub trait Criterion: Any {}
pub trait Sequence: Any {}
pub trait TextOrNumber: Any {}
pub trait Scalar: Any {}
pub trait Field: Any {}
pub trait DateTimeParam: Any {}

trait Param {
    type ParamType<'a>
    where
        Self: 'a;

    fn as_param(&self) -> Self::ParamType<'_>;
}

// -----------------------------------------------------------------------

// Comparision operators
pub trait AnyOp<T: Any> {
    /// equal
    fn eq<U: Any>(&self, other: U) -> OpLogical<T, U>;
    /// not equal
    fn ne<U: Any>(&self, other: U) -> OpLogical<T, U>;
    /// less than
    fn lt<U: Any>(&self, other: U) -> OpLogical<T, U>;
    /// less than or equal
    fn le<U: Any>(&self, other: U) -> OpLogical<T, U>;
    /// greater than
    fn gt<U: Any>(&self, other: U) -> OpLogical<T, U>;
    /// greater than or equal
    fn ge<U: Any>(&self, other: U) -> OpLogical<T, U>;
}

/// Operations on number-like values.
pub trait NumberOp<T: Any> {
    /// add
    fn add<U: Number>(&self, other: U) -> OpNumber<T, U>;
    /// subtract
    fn sub<U: Number>(&self, other: U) -> OpNumber<T, U>;
    /// multiply
    fn mul<U: Number>(&self, other: U) -> OpNumber<T, U>;
    /// divide
    fn div<U: Number>(&self, other: U) -> OpNumber<T, U>;
    /// exponential
    fn pow<U: Number>(&self, other: U) -> OpNumber<T, U>;
    /// as percentage
    fn percent(&self) -> OpNumber<T, ()>;
}

/// Operations on text-like values.
pub trait TextOp<T: Any> {
    /// concat text
    fn concat<U: Text>(&self, other: U) -> OpText<T, U>;
}

/// Operations on boolean-like values.
pub trait LogicalOp<T: Any> {
    /// and
    fn and<U: Logical>(&self, other: U) -> OpLogical<T, U>;
    /// or
    fn or<U: Logical>(&self, other: U) -> OpLogical<T, U>;
    /// xor
    fn xor<U: Logical>(&self, other: U) -> OpLogical<T, U>;
}

/// Operations on references.
pub trait ReferenceOp<T: Any> {
    /// intersection of references
    fn intersect<U: Reference>(&self, other: U) -> OpReference<T, U>;
    /// concatenation of references
    fn refcat<U: Reference>(&self, other: U) -> OpReference<T, U>;
}

// -----------------------------------------------------------------------

impl<T: Any> AnyOp<T> for T {
    fn eq<U: Any>(&self, other: U) -> OpLogical<T, U> {
        eq(self, other)
    }

    fn ne<U: Any>(&self, other: U) -> OpLogical<T, U> {
        ne(self, other)
    }

    fn lt<U: Any>(&self, other: U) -> OpLogical<T, U> {
        lt(self, other)
    }

    fn le<U: Any>(&self, other: U) -> OpLogical<T, U> {
        le(self, other)
    }

    fn gt<U: Any>(&self, other: U) -> OpLogical<T, U> {
        gt(self, other)
    }

    fn ge<U: Any>(&self, other: U) -> OpLogical<T, U> {
        ge(self, other)
    }
}

impl<T: Number> NumberOp<T> for T {
    fn add<U: Number>(&self, other: U) -> OpNumber<T, U> {
        add(self, other)
    }

    fn sub<U: Number>(&self, other: U) -> OpNumber<T, U> {
        sub(self, other)
    }

    fn mul<U: Number>(&self, other: U) -> OpNumber<T, U> {
        mul(self, other)
    }

    fn div<U: Number>(&self, other: U) -> OpNumber<T, U> {
        div(self, other)
    }

    fn pow<U: Number>(&self, other: U) -> OpNumber<T, U> {
        pow(self, other)
    }

    fn percent(&self) -> OpNumber<T, ()> {
        percent(self)
    }
}

impl<T: Text> TextOp<T> for T {
    fn concat<U: Text>(&self, other: U) -> OpText<T, U> {
        concat(self, other)
    }
}

impl<T: Logical> LogicalOp<T> for T {
    fn and<U: Logical>(&self, other: U) -> OpLogical<T, U> {
        and((self, other))
    }

    fn or<U: Logical>(&self, other: U) -> OpLogical<T, U> {
        or((self, other))
    }

    fn xor<U: Logical>(&self, other: U) -> OpLogical<T, U> {
        xor((self, other))
    }
}

impl<T: Reference> ReferenceOp<T> for T {
    fn intersect<U: Reference>(&self, other: U) -> OpReference<T, U> {
        intersect(self, other)
    }
    fn refcat<U: Reference>(&self, other: U) -> OpReference<T, U> {
        refcat(self, other)
    }
}

// -----------------------------------------------------------------------

macro_rules! any_struct {
    (OP $t:ident) => {

        pub struct $t<A:Any, B:Any>(
            A,
            &'static str,
            B
        );

        impl<A:Any, B: Any> Any for $t<A,B> {
            fn formula(&self, buf: &mut String) {
                self.0.formula(buf);
                buf.push_str(self.1.as_ref());
                self.1.formula(buf);
            }
        }
    };
    (VAR $t:ident) => {

        pub struct $t(
            &'static str,
            Vec<Box<dyn Any>>
        );

        impl Any for $t {
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

        pub struct $t(
            &'static str
        );

        impl Any for $t {
            fn formula(&self, buf: &mut String) {
                buf.push_str(self.0.as_ref());
                buf.push('(');
                buf.push(')');
            }
        }
    };
    ($t:ident : $tname0:tt $($tname:tt $tidx:tt)*) => {

        pub struct $t<$tname0: Any $(,$tname: Any)*>(
            &'static str,
            $tname0
            $(,$tname)*
        );

        impl<$tname0: Any $(,$tname: Any)*> Any for $t<$tname0 $(,$tname)*> {
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
    (OP $t:ident) => {
        any_struct!(OP $t);
        fn_any!(__IMPL $t: A B);
    };
    (VAR $t:ident) => {
        any_struct!(VAR $t);
        fn_any!(__IMPL $t);
    };
    ($t:ident) => {
        any_struct!($t);
        fn_any!(__IMPL $t);
    };
    ($t:ident : $tname0:tt $($tname:tt $tidx:tt)*) => {
        any_struct!($t: $tname0 $($tname $tidx)*);
        fn_any!(__IMPL $t: $tname0 $($tname)*);
    };
    (__IMPL $t:ident) => {
        impl Number for $t {}
        impl Text for $t {}
        impl Logical for $t {}
        impl Sequence for $t {}
        impl TextOrNumber for $t {}
        impl Field for $t {}
        impl Scalar for $t {}
        impl DateTimeParam for $t {}
    };
    (__IMPL $t:ident : $tname0:tt $($tname:tt)*) => {
        impl <$tname0: Any $(,$tname: Any)*> Number for $t<$tname0 $(,$tname)*> {}
        impl <$tname0: Any $(,$tname: Any)*> Text for $t<$tname0 $(,$tname)*> {}
        impl <$tname0: Any $(,$tname: Any)*> Logical for $t<$tname0 $(,$tname)*> {}
        impl <$tname0: Any $(,$tname: Any)*> Sequence for $t<$tname0 $(,$tname)*> {}
        impl <$tname0: Any $(,$tname: Any)*> TextOrNumber for $t<$tname0 $(,$tname)*> {}
        impl <$tname0: Any $(,$tname: Any)*> Field for $t<$tname0 $(,$tname)*> {}
        impl <$tname0: Any $(,$tname: Any)*> Scalar for $t<$tname0 $(,$tname)*> {}
        impl <$tname0: Any $(,$tname: Any)*> DateTimeParam for $t<$tname0 $(,$tname)*> {}
    };
}

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
    (OP $t:ident) => {
        any_struct!(OP $t);
        fn_number!(__IMPL $t: A B);
    };
    (VAR $t:ident) => {
        any_struct!(VAR $t);
        fn_number!(__IMPL $t);
    };
    ($t:ident) => {
        any_struct!($t);
        fn_number!(__IMPL $t);
    };
    ($t:ident : $tname0:tt $($tname:tt $tidx:tt)*) => {
        any_struct!($t: $tname0 $($tname $tidx)*);
        fn_number!(__IMPL $t: $tname0 $($tname)*);
    };
    (__IMPL $t:ident) => {
        impl Number for $t {}
        impl Logical for $t {}
        impl Sequence for $t {}
        impl TextOrNumber for $t {}
        impl Field for $t {}
        impl Scalar for $t {}
        impl DateTimeParam for $t {}
    };
    (__IMPL $t:ident : $tname0:tt $($tname:tt)*) => {
        impl <$tname0: Any $(,$tname: Any)*> Number for $t<$tname0 $(,$tname)*> {}
        impl <$tname0: Any $(,$tname: Any)*> Logical for $t<$tname0 $(,$tname)*> {}
        impl <$tname0: Any $(,$tname: Any)*> Sequence for $t<$tname0 $(,$tname)*> {}
        impl <$tname0: Any $(,$tname: Any)*> TextOrNumber for $t<$tname0 $(,$tname)*> {}
        impl <$tname0: Any $(,$tname: Any)*> Field for $t<$tname0 $(,$tname)*> {}
        impl <$tname0: Any $(,$tname: Any)*> Scalar for $t<$tname0 $(,$tname)*> {}
        impl <$tname0: Any $(,$tname: Any)*> DateTimeParam for $t<$tname0 $(,$tname)*> {}
    };
}

fn_number!(OP OpNumber);
fn_number!(VAR FnNumberVar);
fn_number!(FnNumber0);
fn_number!(FnNumber1: A);
fn_number!(FnNumber2: A B 2);
fn_number!(FnNumber3: A B 2 C 3);
fn_number!(FnNumber4: A B 2 C 3 D 4);
fn_number!(FnNumber5: A B 2 C 3 D 4 E 5);
fn_number!(FnNumber6: A B 2 C 3 D 4 E 5 F 6);

macro_rules! fn_text {
    (OP $t:ident) => {
        any_struct!(OP $t);
        fn_text!(__IMPL $t: A B);
    };
    (VAR $t:ident) => {
        any_struct!(VAR $t);
        fn_text!(__IMPL $t);
    };
    ($t:ident) => {
        any_struct!($t);
        fn_text!(__IMPL $t);
    };
    ($t:ident : $tname0:tt $($tname:tt $tidx:tt)*) => {
        any_struct!($t: $tname0 $($tname $tidx)*);
        fn_text!(__IMPL $t: $tname0 $($tname)*);
    };
    (__IMPL $t:ident) => {
        impl Text for $t {}
        impl Sequence for $t {}
        impl TextOrNumber for $t {}
        impl Field for $t {}
        impl Scalar for $t {}
        impl DateTimeParam for $t {}
    };
    (__IMPL $t:ident : $tname0:tt $($tname:tt)*) => {
        impl <$tname0: Any $(,$tname: Any)*> Text for $t<$tname0 $(,$tname)*> {}
        impl <$tname0: Any $(,$tname: Any)*> Sequence for $t<$tname0 $(,$tname)*> {}
        impl <$tname0: Any $(,$tname: Any)*> TextOrNumber for $t<$tname0 $(,$tname)*> {}
        impl <$tname0: Any $(,$tname: Any)*> Field for $t<$tname0 $(,$tname)*> {}
        impl <$tname0: Any $(,$tname: Any)*> Scalar for $t<$tname0 $(,$tname)*> {}
        impl <$tname0: Any $(,$tname: Any)*> DateTimeParam for $t<$tname0 $(,$tname)*> {}
    };
}

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
    (OP $t:ident) => {
        any_struct!(OP $t);
        fn_logical!(__IMPL $t: A B);
    };
    (VAR $t:ident) => {
        any_struct!(VAR $t);
        fn_logical!(__IMPL $t);
    };
    ($t:ident) => {
        any_struct!($t);
        fn_logical!(__IMPL $t);
    };
    ($t:ident : $tname0:tt $($tname:tt $tidx:tt)*) => {
        any_struct!($t: $tname0 $($tname $tidx)*);
        fn_logical!(__IMPL $t: $tname0 $($tname)*);
    };
    (__IMPL $t:ident) => {
        impl Logical for $t {}
        impl Number for $t {}
        impl Sequence for $t {}
        impl Scalar for $t {}
        impl TextOrNumber for $t {}
    };
    (__IMPL $t:ident : $tname0:tt $($tname:tt)*) => {
        impl <$tname0: Any $(,$tname: Any)*> Logical for $t<$tname0 $(,$tname)*> {}
        impl <$tname0: Any $(,$tname: Any)*> Number for $t<$tname0 $(,$tname)*> {}
        impl <$tname0: Any $(,$tname: Any)*> Sequence for $t<$tname0 $(,$tname)*> {}
        impl <$tname0: Any $(,$tname: Any)*> Scalar for $t<$tname0 $(,$tname)*> {}
        impl <$tname0: Any $(,$tname: Any)*> TextOrNumber for $t<$tname0 $(,$tname)*> {}
    };
}

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
    (OP $t:ident) => {
        any_struct!(OP $t);
        fn_matrix!(__IMPL $t: A B);
    };
    (VAR $t:ident) => {
        any_struct!(VAR $t);
        fn_matrix!(__IMPL $t);
    };
    ($t:ident) => {
        any_struct!($t);
        fn_matrix!(__IMPL $t);
    };
    ($t:ident : $tname0:tt $($tname:tt $tidx:tt)*) => {
        any_struct!($t: $tname0 $($tname $tidx)*);
        fn_matrix!(__IMPL $t: $tname0 $($tname)*);
    };
    (__IMPL $t:ident) => {
        impl Matrix for $t {}
    };
    (__IMPL $t:ident : $tname0:tt $($tname:tt)*) => {
        impl <$tname0: Any $(,$tname: Any)*> Matrix for $t<$tname0 $(,$tname)*> {}
    };
}

fn_matrix!(OP OpMatrix);
fn_matrix!(VAR FnMatrixVar);
fn_matrix!(FnMatrix0);
fn_matrix!(FnMatrix1: A);
fn_matrix!(FnMatrix2: A B 2);
fn_matrix!(FnMatrix3: A B 2 C 3);
fn_matrix!(FnMatrix4: A B 2 C 3 D 4);
fn_matrix!(FnMatrix5: A B 2 C 3 D 4 E 5);
fn_matrix!(FnMatrix6: A B 2 C 3 D 4 E 5 F 6);

macro_rules! fn_reference {
    (OP $t:ident) => {
        any_struct!(OP $t);
        fn_reference!(__IMPL $t: A B);
    };
    (VAR $t:ident) => {
        any_struct!(VAR $t);
        fn_reference!(__IMPL $t);
    };
    ($t:ident) => {
        any_struct!($t);
        fn_reference!(__IMPL $t);
    };
    ($t:ident : $tname0:tt $($tname:tt $tidx:tt)*) => {
        any_struct!($t: $tname0 $($tname $tidx)*);
        fn_reference!(__IMPL $t: $tname0 $($tname)*);
    };
    (__IMPL $t:ident) => {
        impl Reference for $t {}
        impl Number for $t {}
        impl Text for $t {}
        impl Logical for $t {}
        impl Matrix for $t {}
        impl Sequence for $t {}
        impl TextOrNumber for $t {}
        impl Field for $t {}
        impl Scalar for $t {}
        impl DateTimeParam for $t {}
    };
    (__IMPL $t:ident : $tname0:tt $($tname:tt)*) => {
        impl <$tname0: Any $(,$tname: Any)*> Reference for $t<$tname0 $(,$tname)*> {}
        impl <$tname0: Any $(,$tname: Any)*> Number for $t<$tname0 $(,$tname)*> {}
        impl <$tname0: Any $(,$tname: Any)*> Text for $t<$tname0 $(,$tname)*> {}
        impl <$tname0: Any $(,$tname: Any)*> Logical for $t<$tname0 $(,$tname)*> {}
        impl <$tname0: Any $(,$tname: Any)*> Matrix for $t<$tname0 $(,$tname)*> {}
        impl <$tname0: Any $(,$tname: Any)*> Sequence for $t<$tname0 $(,$tname)*> {}
        impl <$tname0: Any $(,$tname: Any)*> TextOrNumber for $t<$tname0 $(,$tname)*> {}
        impl <$tname0: Any $(,$tname: Any)*> Field for $t<$tname0 $(,$tname)*> {}
        impl <$tname0: Any $(,$tname: Any)*> Scalar for $t<$tname0 $(,$tname)*> {}
        impl <$tname0: Any $(,$tname: Any)*> DateTimeParam for $t<$tname0 $(,$tname)*> {}
    };
}

fn_reference!(OP OpReference);
fn_reference!(VAR FnReferenceVar);
fn_reference!(FnReference0);
fn_reference!(FnReference1: A);
fn_reference!(FnReference2: A B 2);
fn_reference!(FnReference3: A B 2 C 3);
fn_reference!(FnReference4: A B 2 C 3 D 4);
fn_reference!(FnReference5: A B 2 C 3 D 4 E 5);
fn_reference!(FnReference6: A B 2 C 3 D 4 E 5 F 6);

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
    pub fn new(op: CriterionCmp, f: A) -> Self {
        Self(op, f)
    }
}

impl<A: Any> Any for FCriterion<A> {
    fn formula(&self, buf: &mut String) {
        let _ = write!(buf, "\"{}\"", self.0);
        buf.push('&');
        self.1.formula(buf);
    }
}
impl<A: Any> Criterion for FCriterion<A> {}

impl<A: Any> Any for (CriterionCmp, A) {
    fn formula(&self, buf: &mut String) {
        let _ = write!(buf, "\"{}\"", self.0);
        buf.push('&');
        self.1.formula(buf);
    }
}
impl<A: Any> Criterion for (CriterionCmp, A) {}

// -----------------------------------------------------------------------

impl<T: Any + ?Sized> Any for &T {
    fn formula(&self, buf: &mut String) {
        (*self).formula(buf);
    }
}
impl<T: Number + Any + ?Sized> Number for &T {}
impl<T: Text + Any + ?Sized> Text for &T {}
impl<T: Logical + Any + ?Sized> Logical for &T {}
impl<T: Reference + Any + ?Sized> Reference for &T {}
impl<T: Criterion + Any + ?Sized> Criterion for &T {}
impl<T: Sequence + Any + ?Sized> Sequence for &T {}
impl<T: Matrix + Any + ?Sized> Matrix for &T {}
impl<T: TextOrNumber + Any + ?Sized> TextOrNumber for &T {}
impl<T: Field + Any + ?Sized> Field for &T {}
impl<T: DateTimeParam + Any + ?Sized> DateTimeParam for &T {}

impl<T: Any + Sized> Any for Option<T> {
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
impl<T: Criterion + Any + Sized> Criterion for Option<T> {}
impl<T: Sequence + Any + Sized> Sequence for Option<T> {}
impl<T: Matrix + Any + Sized> Matrix for Option<T> {}
impl<T: TextOrNumber + Any + Sized> TextOrNumber for Option<T> {}
impl<T: Field + Any + Sized> Field for Option<T> {}
impl<T: DateTimeParam + Any + Sized> DateTimeParam for Option<T> {}

impl<T: Any, const N: usize, const M: usize> Any for [[T; M]; N] {
    fn formula(&self, buf: &mut String) {
        buf.push('{');
        for (i, r) in self.iter().enumerate() {
            if i > 0 {
                buf.push('|');
            }
            for (j, c) in r.iter().enumerate() {
                if j > 0 {
                    buf.push(';');
                }
                c.formula(buf);
            }
        }
        buf.push('}');
    }
}
impl<T: Any, const N: usize, const M: usize> Matrix for [[T; M]; N] {}
impl<T: Any, const N: usize, const M: usize> Sequence for [[T; M]; N] {}

// -----------------------------------------------------------------------

impl Any for () {
    fn formula(&self, _buf: &mut String) {}
}

// -----------------------------------------------------------------------

/// An expression in parentheses.
/// Use p() / parentheses() to create one.
pub struct FParentheses<A>(A);
impl<A: Any> Any for FParentheses<A> {
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
impl<A: Sequence> Sequence for FParentheses<A> {}
impl<A: TextOrNumber> TextOrNumber for FParentheses<A> {}
impl<A: Field> Field for FParentheses<A> {}
impl<A: DateTimeParam> DateTimeParam for FParentheses<A> {}

/// Creates an expression in parentheses. Aliased as p().
pub fn parentheses<A: Any>(a: A) -> FParentheses<A> {
    FParentheses(a)
}

// -----------------------------------------------------------------------

macro_rules! value_number {
    ($t:ty) => {
        impl Any for $t {
            fn formula(&self, buf: &mut String) {
                let _ = write!(buf, "{}", self);
            }
        }
        impl Number for $t {}
        impl Logical for $t {}
        impl Sequence for $t {}
        impl TextOrNumber for $t {}
        impl Field for $t {}
        impl Scalar for $t {}
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

impl Any for bool {
    fn formula(&self, buf: &mut String) {
        buf.push_str(if *self { "TRUE()" } else { "FALSE()" });
    }
}
impl Logical for bool {}
impl Number for bool {}
impl Scalar for bool {}
impl Sequence for bool {}

impl Any for &str {
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
impl Field for &str {}
impl Scalar for &str {}
impl DateTimeParam for &str {}

impl<'a> Any for Cow<'a, str> {
    fn formula(&self, buf: &mut String) {
        let str = self.as_ref();
        if str.contains('"') {
            buf.push('"');
            for (i, s) in str.split('"').enumerate() {
                if i > 0 {
                    buf.push_str("\"\"");
                }
                buf.push_str(s);
            }
            buf.push('"');
        } else {
            buf.push('"');
            buf.push_str(str);
            buf.push('"');
        }
    }
}
impl<'a> Text for Cow<'a, str> {}
impl<'a> Sequence for Cow<'a, str> {}
impl<'a> TextOrNumber for Cow<'a, str> {}
impl<'a> Field for Cow<'a, str> {}
impl<'a> Scalar for Cow<'a, str> {}
impl<'a> DateTimeParam for Cow<'a, str> {}

impl Any for String {
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
impl Text for String {}
impl Sequence for String {}
impl TextOrNumber for String {}
impl Field for String {}
impl Scalar for String {}
impl DateTimeParam for String {}

impl Any for CellRef {
    fn formula(&self, buf: &mut String) {
        buf.push_str(self.to_formula().as_str())
    }
}
impl Reference for CellRef {}
impl Number for CellRef {}
impl Text for CellRef {}
impl Logical for CellRef {}
impl Sequence for CellRef {}
impl TextOrNumber for CellRef {}
impl Field for CellRef {}
impl Scalar for CellRef {}
impl Matrix for CellRef {}
impl DateTimeParam for CellRef {}

impl Any for CellRange {
    fn formula(&self, buf: &mut String) {
        buf.push_str(self.to_formula().as_str())
    }
}
impl Reference for CellRange {}
impl Number for CellRange {}
impl Text for CellRange {}
impl Logical for CellRange {}
impl Sequence for CellRange {}
impl TextOrNumber for CellRange {}
impl Field for CellRange {}
impl Scalar for CellRange {}
impl Matrix for CellRange {}
impl DateTimeParam for CellRange {}

// -----------------------------------------------------------------------

// -----------------------------------------------------------------------

/// Creates a formula from any formula expression.
pub fn formula<T: Any>(f: T) -> String {
    let mut buf = String::new();
    buf.push_str("of=");
    let _ = f.formula(&mut buf);
    buf
}

// create a array for the parameters.
fn create_param<'a>(n: usize) -> Box<[MaybeUninit<&'a dyn Any>]> {
    let layout = alloc::Layout::array::<MaybeUninit<&dyn Any>>(n).unwrap();

    assert!(layout.size() > 0);

    let sl = unsafe {
        let ptr = std::alloc::alloc(layout) as *mut MaybeUninit<&dyn Any>;
        if ptr.is_null() {
            alloc::handle_alloc_error(layout);
        }

        slice::from_raw_parts(ptr, n)
    };

    Box::from(sl)
}

// assume init
#[inline]
unsafe fn param_assume_init(p: Box<[MaybeUninit<&dyn Any>]>) -> Box<[&dyn Any]> {
    mem::transmute(p)
}

#[inline(never)]
fn func(name: &str, args: &[&dyn Any]) -> String {
    let mut buf = String::new();
    buf.push_str(name);
    buf.push('(');
    for (i, v) in args.iter().enumerate() {
        if i > 0 {
            buf.push(';');
        }
        let _ = v.formula(&mut buf);
    }
    buf.push(')');
    buf
}

#[inline]
fn infix<'a, A: Any, B: Any>(a: A, op: &str, b: B) -> String {
    let mut buf = String::new();
    a.formula(&mut buf);
    buf.push_str(op);
    b.formula(&mut buf);
    buf
}

#[inline]
fn prefix<'a, A: Any>(op: &str, a: A) -> String {
    let mut buf = String::new();
    buf.push_str(op);
    a.formula(&mut buf);
    buf
}

#[inline]
fn postfix<'a, A: Any>(a: A, op: &str) -> String {
    let mut buf = String::new();
    a.formula(&mut buf);
    buf.push_str(op);
    buf
}

// -----------------------------------------------------------------------
