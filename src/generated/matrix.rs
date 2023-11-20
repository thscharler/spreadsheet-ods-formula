use crate::*;
#[allow(unused_imports)]
use crate::matrix::*;

#[inline]
pub fn mdeterm<A: Matrix>(a: A) -> FnNumber1<A> {
    FnNumber1("MDETERM", a)
}

#[inline]
pub fn minverse<A: Matrix>(a: A) -> FnMatrix1<A> {
    FnMatrix1("MINVERSE", a)
}

#[inline]
pub fn mmult<A: Matrix, B: Matrix>(a: A, b: B) -> FnMatrix2<A, B> {
    FnMatrix2("MMULT", a, b)
}

#[inline]
pub fn munit<A: Number>(n: A) -> FnMatrix1<A> {
    FnMatrix1("MUNIT", n)
}

#[inline]
pub fn transpose<A: Matrix>(a: A) -> FnMatrix1<A> {
    FnMatrix1("TRANSPOSE", a)
}
