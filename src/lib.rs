use spreadsheet_ods::{CellRange, CellRef};
use std::borrow::Cow;
use std::fmt::{Display, Formatter, Write};
use std::ops::{Add, BitAnd, BitXor, Div, Mul, Neg, Sub};

pub trait Any {
    fn formula(&self, buf: &mut String);
}

pub trait Number: Any {}
pub trait Text: Any {}
pub trait Logical: Any {}
pub trait Reference: Any {}
pub trait Matrix: Any {}
pub trait Criterion: Any {}
pub trait Sequence: Any {
    fn into_vec(self) -> Vec<Box<dyn Any>>;
}
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
    fn eq<U: Any>(self, other: U) -> OpLogical<T, U> {
        eq(self, other)
    }

    fn ne<U: Any>(self, other: U) -> OpLogical<T, U> {
        ne(self, other)
    }

    fn lt<U: Any>(self, other: U) -> OpLogical<T, U> {
        lt(self, other)
    }

    fn le<U: Any>(self, other: U) -> OpLogical<T, U> {
        le(self, other)
    }

    fn gt<U: Any>(self, other: U) -> OpLogical<T, U> {
        gt(self, other)
    }

    fn ge<U: Any>(self, other: U) -> OpLogical<T, U> {
        ge(self, other)
    }
}

impl<T: Number> NumberOp<T> for T {
    fn add<U: Number>(self, other: U) -> OpNumber<T, U> {
        add(self, other)
    }

    fn sub<U: Number>(self, other: U) -> OpNumber<T, U> {
        sub(self, other)
    }

    fn mul<U: Number>(self, other: U) -> OpNumber<T, U> {
        mul(self, other)
    }

    fn div<U: Number>(self, other: U) -> OpNumber<T, U> {
        div(self, other)
    }

    fn pow<U: Number>(self, other: U) -> OpNumber<T, U> {
        pow(self, other)
    }

    fn percent(self) -> OpNumber<T, ()> {
        percent(self)
    }
}

impl<T: Text> TextOp<T> for T {
    fn concat<U: Text>(self, other: U) -> OpText<T, U> {
        concat(self, other)
    }
}

impl<T: Logical> LogicalOp<T> for T {
    fn and<U: Logical>(self, other: U) -> FnLogical2<T, U> {
        FnLogical2("AND", self, other)
    }

    fn or<U: Logical>(self, other: U) -> FnLogical2<T, U> {
        FnLogical2("OR", self, other)
    }

    fn xor<U: Logical>(self, other: U) -> FnLogical2<T, U> {
        FnLogical2("XOR", self, other)
    }
}

impl<T: Reference> ReferenceOp<T> for T {
    fn intersect<U: Reference>(self, other: U) -> OpReference<T, U> {
        intersect(self, other)
    }
    fn refcat<U: Reference>(self, other: U) -> OpReference<T, U> {
        refcat(self, other)
    }
}

// -----------------------------------------------------------------------

impl Any for Vec<Box<dyn Any>> {
    fn formula(&self, buf: &mut String) {
        for (i, v) in self.iter().enumerate() {
            if i > 0 {
                buf.push(';');
            }
            let _ = v.formula(buf);
        }
    }
}

impl Sequence for Vec<Box<dyn Any>> {
    fn into_vec(self) -> Vec<Box<dyn Any>> {
        self
    }
}

// -----------------------------------------------------------------------

macro_rules! any_struct {
    (OP $t:ident) => {

        pub struct $t<A:Any, B:Any>(
            pub A,
            pub &'static str,
            pub B
        );

        impl<A:Any, B: Any> Any for $t<A,B> {
            fn formula(&self, buf: &mut String) {
                self.0.formula(buf);
                buf.push_str(self.1.as_ref());
                self.1.formula(buf);
            }
        }

        impl <A:Any+'static, B:Any+'static> Sequence for $t<A, B> {
            fn into_vec(self) -> Vec<Box<dyn Any>> {
                vec![Box::new(self)]
            }
        }
    };
    (VAR $t:ident) => {

        pub struct $t(
            pub &'static str,
            pub Vec<Box<dyn Any>>
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

        impl Sequence for $t {
            fn into_vec(self) -> Vec<Box<dyn Any>> {
                vec![Box::new(self)]
            }
        }

    };
    ($t:ident) => {

        pub struct $t(
            pub &'static str
        );

        impl Any for $t {
            fn formula(&self, buf: &mut String) {
                buf.push_str(self.0.as_ref());
                buf.push('(');
                buf.push(')');
            }
        }

        impl Sequence for $t {
            fn into_vec(self) -> Vec<Box<dyn Any>> {
                vec![Box::new(self)]
            }
        }

    };
    ($t:ident : $tname0:tt $($tname:tt $tidx:tt)*) => {

        pub struct $t<$tname0: Any+'static $(,$tname: Any+'static)*>(
            pub &'static str,
            pub $tname0
            $(, pub $tname)*
        );

        impl <$tname0: Any+'static $(,$tname: Any+'static)*> Any for $t<$tname0 $(,$tname)*> {
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

        impl <$tname0: Any+'static $(,$tname: Any+'static)*> Sequence for $t<$tname0 $(,$tname)*> {
            fn into_vec(self) -> Vec<Box<dyn Any>> {
                vec![Box::new(self)]
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
        impl $(<$l>)? TextOrNumber for $t$(<$l>)?  {}
        impl $(<$l>)? Field for $t$(<$l>)?  {}
        impl $(<$l>)? Scalar for $t$(<$l>)?  {}
        impl $(<$l>)? DateTimeParam for $t$(<$l>)?  {}
    };
    (__IMPL $t:ident : $($l:lifetime)? $tname0:tt $($tname:tt)*) => {
        impl <$($l, )?$tname0: Any $(,$tname: Any)*> Number for $t<$($l, )?$tname0 $(,$tname)*> {}
        impl <$($l, )?$tname0: Any $(,$tname: Any)*> Text for $t<$($l, )?$tname0 $(,$tname)*> {}
        impl <$($l, )?$tname0: Any $(,$tname: Any)*> Logical for $t<$($l, )?$tname0 $(,$tname)*> {}
        impl <$($l, )?$tname0: Any $(,$tname: Any)*> TextOrNumber for $t<$($l, )?$tname0 $(,$tname)*> {}
        impl <$($l, )?$tname0: Any $(,$tname: Any)*> Field for $t<$($l, )?$tname0 $(,$tname)*> {}
        impl <$($l, )?$tname0: Any $(,$tname: Any)*> Scalar for $t<$($l, )?$tname0 $(,$tname)*> {}
        impl <$($l, )?$tname0: Any $(,$tname: Any)*> DateTimeParam for $t<$($l, )?$tname0 $(,$tname)*> {}
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
        impl $(<$l>)? TextOrNumber for $t$(<$l>)?  {}
        impl $(<$l>)? Field for $t$(<$l>)?  {}
        impl $(<$l>)? Scalar for $t$(<$l>)?  {}
        impl $(<$l>)? DateTimeParam for $t$(<$l>)?  {}
    };
    (__IMPL $t:ident : $($l:lifetime)? $tname0:tt $($tname:tt)*) => {
        impl <$($l, )?$tname0: Any $(,$tname: Any)*> Number for $t<$($l, )?$tname0 $(,$tname)*> {}
        impl <$($l, )?$tname0: Any $(,$tname: Any)*> Logical for $t<$($l, )?$tname0 $(,$tname)*> {}
        impl <$($l, )?$tname0: Any $(,$tname: Any)*> TextOrNumber for $t<$($l, )?$tname0 $(,$tname)*> {}
        impl <$($l, )?$tname0: Any $(,$tname: Any)*> Field for $t<$($l, )?$tname0 $(,$tname)*> {}
        impl <$($l, )?$tname0: Any $(,$tname: Any)*> Scalar for $t<$($l, )?$tname0 $(,$tname)*> {}
        impl <$($l, )?$tname0: Any $(,$tname: Any)*> DateTimeParam for $t<$($l, )?$tname0 $(,$tname)*> {}
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
        impl $(<$l>)? TextOrNumber for $t$(<$l>)?  {}
        impl $(<$l>)? Field for $t$(<$l>)?  {}
        impl $(<$l>)? Scalar for $t$(<$l>)?  {}
        impl $(<$l>)? DateTimeParam for $t$(<$l>)?  {}
    };
    (__IMPL $t:ident : $($l:lifetime)? $tname0:tt $($tname:tt)*) => {
        impl <$($l, )?$tname0: Any $(,$tname: Any)*> Text for $t<$($l, )?$tname0 $(,$tname)*> {}
        impl <$($l, )?$tname0: Any $(,$tname: Any)*> TextOrNumber for $t<$($l, )?$tname0 $(,$tname)*> {}
        impl <$($l, )?$tname0: Any $(,$tname: Any)*> Field for $t<$($l, )?$tname0 $(,$tname)*> {}
        impl <$($l, )?$tname0: Any $(,$tname: Any)*> Scalar for $t<$($l, )?$tname0 $(,$tname)*> {}
        impl <$($l, )?$tname0: Any $(,$tname: Any)*> DateTimeParam for $t<$($l, )?$tname0 $(,$tname)*> {}
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
        impl $(<$l>)? Logical for $t$(<$l>)?  {}
        impl $(<$l>)? Number for $t$(<$l>)?  {}
        impl $(<$l>)? Scalar for $t$(<$l>)?  {}
        impl $(<$l>)? TextOrNumber for $t$(<$l>)?  {}
    };
    (__IMPL $t:ident : $($l:lifetime)? $tname0:tt $($tname:tt)*) => {
        impl <$($l, )?$tname0: Any $(,$tname: Any)*> Logical for $t<$($l, )?$tname0 $(,$tname)*> {}
        impl <$($l, )?$tname0: Any $(,$tname: Any)*> Number for $t<$($l, )?$tname0 $(,$tname)*> {}
        impl <$($l, )?$tname0: Any $(,$tname: Any)*> Scalar for $t<$($l, )?$tname0 $(,$tname)*> {}
        impl <$($l, )?$tname0: Any $(,$tname: Any)*> TextOrNumber for $t<$($l, )?$tname0 $(,$tname)*> {}
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
        impl $(<$l>)? Reference for $t$(<$l>)?  {}
        impl $(<$l>)? Number for $t$(<$l>)?  {}
        impl $(<$l>)? Text for $t$(<$l>)?  {}
        impl $(<$l>)? Logical for $t$(<$l>)?  {}
        impl $(<$l>)? Matrix for $t$(<$l>)?  {}
        impl $(<$l>)? TextOrNumber for $t$(<$l>)?  {}
        impl $(<$l>)? Field for $t$(<$l>)?  {}
        impl $(<$l>)? Scalar for $t$(<$l>)?  {}
        impl $(<$l>)? DateTimeParam for $t$(<$l>)?  {}
    };
    (__IMPL $t:ident : $($l:lifetime)? $tname0:tt $($tname:tt)*) => {
        impl <$($l, )?$tname0: Any $(,$tname: Any)*> Reference for $t<$($l, )?$tname0 $(,$tname)*> {}
        impl <$($l, )?$tname0: Any $(,$tname: Any)*> Number for $t<$($l, )?$tname0 $(,$tname)*> {}
        impl <$($l, )?$tname0: Any $(,$tname: Any)*> Text for $t<$($l, )?$tname0 $(,$tname)*> {}
        impl <$($l, )?$tname0: Any $(,$tname: Any)*> Logical for $t<$($l, )?$tname0 $(,$tname)*> {}
        impl <$($l, )?$tname0: Any $(,$tname: Any)*> Matrix for $t<$($l, )?$tname0 $(,$tname)*> {}
        impl <$($l, )?$tname0: Any $(,$tname: Any)*> TextOrNumber for $t<$($l, )?$tname0 $(,$tname)*> {}
        impl <$($l, )?$tname0: Any $(,$tname: Any)*> Field for $t<$($l, )?$tname0 $(,$tname)*> {}
        impl <$($l, )?$tname0: Any $(,$tname: Any)*> Scalar for $t<$($l, )?$tname0 $(,$tname)*> {}
        impl <$($l, )?$tname0: Any $(,$tname: Any)*> DateTimeParam for $t<$($l, )?$tname0 $(,$tname)*> {}
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

impl<T: Any + ?Sized> Any for Box<T> {
    fn formula(&self, buf: &mut String) {
        self.as_ref().formula(buf);
    }
}
impl<T: Number + Any + ?Sized> Number for Box<T> {}
impl<T: Text + Any + ?Sized> Text for Box<T> {}
impl<T: Logical + Any + ?Sized> Logical for Box<T> {}
impl<T: Reference + Any + ?Sized> Reference for Box<T> {}
impl<T: Criterion + Any + ?Sized> Criterion for Box<T> {}
impl<T: Sequence + Any + ?Sized + 'static> Sequence for Box<T> {
    fn into_vec(self) -> Vec<Box<dyn Any>> {
        vec![Box::new(self)]
    }
}
impl<T: Matrix + Any + ?Sized> Matrix for Box<T> {}
impl<T: TextOrNumber + Any + ?Sized> TextOrNumber for Box<T> {}
impl<T: Field + Any + ?Sized> Field for Box<T> {}
impl<T: DateTimeParam + Any + ?Sized> DateTimeParam for Box<T> {}

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
impl<T: Sequence + Any + 'static> Sequence for Option<T> {
    fn into_vec(self) -> Vec<Box<dyn Any>> {
        vec![Box::new(self)]
    }
}
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
// todo: sequence?

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
impl<A: Sequence + Any + 'static> Sequence for FParentheses<A> {
    fn into_vec(self) -> Vec<Box<dyn Any>> {
        vec![Box::new(self)]
    }
}
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
        impl Sequence for $t {
            fn into_vec(self) -> Vec<Box<dyn Any>> {
                vec![Box::new(self)]
            }
        }
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
impl Sequence for bool {
    fn into_vec(self) -> Vec<Box<dyn Any>> {
        vec![Box::new(self)]
    }
}

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
impl Sequence for &'static str {
    fn into_vec(self) -> Vec<Box<dyn Any>> {
        vec![Box::new(self)]
    }
}
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
impl Sequence for Cow<'static, str> {
    fn into_vec(self) -> Vec<Box<dyn Any>> {
        vec![Box::new(self)]
    }
}
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
impl Sequence for String {
    fn into_vec(self) -> Vec<Box<dyn Any>> {
        vec![Box::new(self)]
    }
}
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
impl Sequence for CellRef {
    fn into_vec(self) -> Vec<Box<dyn Any>> {
        vec![Box::new(self)]
    }
}
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
impl Sequence for CellRange {
    fn into_vec(self) -> Vec<Box<dyn Any>> {
        vec![Box::new(self)]
    }
}
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

// -----------------------------------------------------------------------

/// Adds two numbers. Also available as postfix add() and as operator +.
pub fn add<'a, A: Number, B: Number>(a: A, b: B) -> OpNumber<A, B> {
    OpNumber(a, "+", b)
}

/// Subtracts two numbers. Also available as postfix sub() and as operator -.
pub fn sub<'a, A: Number, B: Number>(a: A, b: B) -> OpNumber<A, B> {
    OpNumber(a, "-", b)
}

/// Multiplies to numbers. Also available as postfix mul() and as operator *;
pub fn mul<'a, A: Number, B: Number>(a: A, b: B) -> OpNumber<A, B> {
    OpNumber(a, "*", b)
}

/// Divides to numbers. Also available as postfix div() and as operator /.
pub fn div<'a, A: Number, B: Number>(a: A, b: B) -> OpNumber<A, B> {
    OpNumber(a, "/", b)
}

/// Exponential function. Also available as postfix pow() and as operator ^.
pub fn pow<'a, A: Number, B: Number>(a: A, b: B) -> OpNumber<A, B> {
    OpNumber(a, "^", b)
}

/// Negates as number. Also available as prefix operator -.
pub fn neg<'a, A: Number>(a: A) -> OpNumber<(), A> {
    OpNumber((), "-", a)
}

/// percentage. Also available as postfix percent()
pub fn percent<'a, A: Number>(a: A) -> OpNumber<A, ()> {
    OpNumber(a, "%", ())
}

macro_rules! number_op {
    ($t:ident $(< $($l:lifetime $(,)? )? $($tname:ident $(,)?)* >)?) => {
        impl <$($($l,)? $($tname: Any,)*)? V: Number> Add<V> for $t $(< $($l,)? $($tname,)* >)? {
            type Output = OpNumber<Self, V>;

            fn add(self, rhs: V) -> Self::Output {
                OpNumber(self, "+", rhs)
            }
        }

        impl <$($($l,)? $($tname: Any,)*)? V: Number> Sub<V> for $t $(< $($l,)? $($tname,)* >)? {
            type Output = OpNumber<Self, V>;

            fn sub(self, rhs: V) -> Self::Output {
                OpNumber(self, "-", rhs)
            }
        }

        impl <$($($l,)? $($tname: Any,)*)? V: Number> Mul<V> for $t $(< $($l,)? $($tname,)* >)? {
            type Output = OpNumber<Self, V>;

            fn mul(self, rhs: V) -> Self::Output {
                OpNumber(self, "*", rhs)
            }
        }

        impl <$($($l,)? $($tname: Any,)*)? V: Number> Div<V> for $t $(< $($l,)? $($tname,)* >)? {
            type Output = OpNumber<Self, V>;

            fn div(self, rhs: V) -> Self::Output {
                OpNumber(self, "*", rhs)
            }
        }

        impl <$($($l,)? $($tname: Any,)*)? V: Number> BitXor<V> for $t $(< $($l,)? $($tname,)* >)? {
            type Output = OpNumber<Self, V>;

            fn bitxor(self, rhs: V) -> Self::Output {
                OpNumber(self, "^", rhs)
            }
        }

        impl <$($($l,)? $($tname: Any,)*)?> Neg for $t $(< $($l,)? $($tname,)* >)? {
            type Output = OpNumber<(), Self>;

            fn neg(self) -> Self::Output {
                OpNumber((), "-", self)
            }
        }

    };
}

number_op!(OpAny<A, B>);
number_op!(FnAnyVar);
number_op!(FnAny0);
number_op!(FnAny1<A>);
number_op!(FnAny2<A, B>);
number_op!(FnAny3<A, B, C>);
number_op!(FnAny4<A, B, C, D>);
number_op!(FnAny5<A, B, C, D, E>);
number_op!(FnAny6<A, B, C, D, E, F>);

number_op!(OpNumber<A, B>);
number_op!(FnNumberVar);
number_op!(FnNumber0);
number_op!(FnNumber1<A>);
number_op!(FnNumber2<A, B>);
number_op!(FnNumber3<A, B, C>);
number_op!(FnNumber4<A, B, C, D>);
number_op!(FnNumber5<A, B, C, D, E>);
number_op!(FnNumber6<A, B, C, D, E, F>);

number_op!(OpLogical<A, B>);
number_op!(FnLogicalVar);
number_op!(FnLogical0);
number_op!(FnLogical1<A>);
number_op!(FnLogical2<A, B>);
number_op!(FnLogical3<A, B, C>);
number_op!(FnLogical4<A, B, C, D>);
number_op!(FnLogical5<A, B, C, D, E>);
number_op!(FnLogical6<A, B, C, D, E, F>);

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

/// concatenates two strings. Also available as postfix concat() and as operator &.
pub fn concat<'a, A: Text, B: Text>(a: A, b: B) -> OpText<A, B> {
    OpText(a, "&", b)
}

macro_rules! text_op {
    ($t:ident $(< $($l:lifetime $(,)? )? $($tname:ident $(,)?)* >)?) => {
        impl <$($($l,)? $($tname: Any,)*)? V: Text> BitAnd<V> for $t $(< $($l,)? $($tname,)* >)? {
            type Output = OpText<Self, V>;

            fn bitand(self, rhs: V) -> Self::Output {
                OpText(self, "&", rhs)
            }
        }
    }
}

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

pub fn intersect<'a, A: Reference, B: Reference>(a: A, b: B) -> OpReference<A, B> {
    OpReference(a, "!", b)
}
pub fn refcat<'a, A: Reference, B: Reference>(a: A, b: B) -> OpReference<A, B> {
    OpReference(a, "~", b)
}

// -----------------------------------------------------------------------

/// equal
pub fn eq<'a, A: Any, B: Any>(a: A, b: B) -> OpLogical<A, B> {
    OpLogical(a, "=", b)
}

/// inequal
pub fn ne<'a, A: Any, B: Any>(a: A, b: B) -> OpLogical<A, B> {
    OpLogical(a, "<>", b)
}

/// less than
pub fn lt<'a, A: Any, B: Any>(a: A, b: B) -> OpLogical<A, B> {
    OpLogical(a, "<", b)
}

/// less than or equal
pub fn le<'a, A: Any, B: Any>(a: A, b: B) -> OpLogical<A, B> {
    OpLogical(a, "<=", b)
}

/// greater than
pub fn gt<'a, A: Any, B: Any>(a: A, b: B) -> OpLogical<A, B> {
    OpLogical(a, ">", b)
}

/// greater than or equal
pub fn ge<'a, A: Any, B: Any>(a: A, b: B) -> OpLogical<A, B> {
    OpLogical(a, ">=", b)
}

// -----------------------------------------------------------------------

///  Compute logical AND of all parameters.
#[inline]
pub fn and(list: impl Sequence) -> FnLogicalVar {
    FnLogicalVar("AND", list.into_vec())
}

/// Compute logical OR of all parameters.
#[inline]
pub fn or(list: impl Sequence) -> FnLogicalVar {
    FnLogicalVar("OR", list.into_vec())
}

/// Compute logical OR of all parameters.
#[inline]
pub fn xor(list: impl Sequence) -> FnLogicalVar {
    FnLogicalVar("XOR", list.into_vec())
}

// -----------------------------------------------------------------------

#[macro_export]
macro_rules! cell {
    ($row:expr, $col:expr) => {
        CellRef::local($row, $col)
    };
    ($table:expr => $row:expr, $col:expr) => {
        CellRef::remote($table, $row, $col)
    };
}

#[macro_export]
macro_rules! range {
    ($row:expr, $col:expr, $row_to:expr, $col_to:expr) => {
        CellRange::local($row, $col, $row_to, $col_to)
    };
    ($row:expr, $col:expr; + $row_delta:expr, $col_delta:expr) => {
        CellRange::origin_span($row, $col, ($row_delta, $col_delta))
    };
    ($table:expr => $row:expr, $col:expr, $row_to:expr, $col_to:expr) => {
        CellRange::remote($table, $row, $col, $row_to, $col_to)
    };
    ($table:expr => $row:expr, $col:expr; + $row_delta:expr, $col_delta:expr) => {
        CellRange::remote($table, $row, $col, $row + $row_delta, $col + $col_delta)
    };
}
