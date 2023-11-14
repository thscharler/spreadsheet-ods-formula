use crate::*;
#[allow(unused_imports)]
use crate::matrix::*;

///  Calculates the determinant of a matrix.
#[inline]
pub fn mdeterm<A: Matrix>(a: A) -> FnNumber1<A> {
    FnNumber1("MDETERM", a)
}

///  Returns the inverse of a matrix.
#[inline]
pub fn minverse<A: Matrix>(a: A) -> FnMatrix1<A> {
    FnMatrix1("MINVERSE", a)
}

///  Multiplies the matrices A and B.
#[inline]
pub fn mmult<A: Matrix>(a: A) -> FnMatrix1<A> {
    FnMatrix1("MMULT", a)
}

///  Creates a unit matrix of a specified dimension N
#[inline]
pub fn munit<A: Number>(n: A) -> FnMatrix1<A> {
    FnMatrix1("MUNIT", n)
}

///  Returns the transpose of a matrix.
#[inline]
pub fn transpose<A: Matrix>(a: A) -> FnMatrix1<A> {
    FnMatrix1("TRANSPOSE", a)
}
