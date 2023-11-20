//! 
//! Matrix functions operate on matrices.
//! 
//! A matrix with M rows and N columns is defined by
//! 
//! The dimension subscript may be omitted, if the context allows it, i.e.
//! . Matrices are represented by upper-case letters. The elements of a matrix 
//! are denoted by the corresponding lower case letter and subscripts, which 
//! defines the row and column number.
//! Square matrices have the same number of rows and columns, i.e.
//! .

use crate::*;
#[allow(unused_imports)]
use crate::matrix::*;

/// Calculates the determinant of a matrix.
///
/// __Syntax__: 
/// ```ods
///     MDETERM( A Array )
/// ```
///
/// __Constraints__:
/// Only square matrices are allowed.
///
/// __Semantics__:
/// Returns the determinant of matrix A. The determinant is defined by
/// 
/// where P denotes a permutation of the numbers 1, 2, ..., n and
/// is the sign of the permutation, which is +1 for an even amount of 
/// permutations (i.e., permutations that can be written as the composition of 
/// an even number of transpositions), -1 otherwise. A transposition on 1, ..., 
/// n is a permutation of 1, ..., n with exactly (n - 2) numbers fixed.
///
/// __See also__: "MINVERSE", 
#[inline]
pub fn mdeterm<A: Matrix>(a: A) -> FnNumber1<A> {
    FnNumber1("MDETERM", a)
}

/// Returns the inverse of a matrix.
///
/// __Syntax__: 
/// ```ods
///     MINVERSE( A Array )
/// ```
///
/// __Constraints__:
/// Only square matrices are allowed.
///
/// __Semantics__:
/// Calculates the inverse
/// of matrix A. The matrix A multiplied with its inverse
/// results in the unity matrix of the same dimension as A:
/// 
/// Invertible matrices have a non-zero determinant. If the matrix is not 
/// invertible, this function should return an Error value.
///
/// __See also__: "MDETERM", 
#[inline]
pub fn minverse<A: Matrix>(a: A) -> FnMatrix1<A> {
    FnMatrix1("MINVERSE", a)
}

/// Multiplies the matrices A and B.
///
/// __Syntax__: 
/// ```ods
///     MMULT( A Array; B Array )
/// ```
///
/// __Constraints__:
/// COLUMNS(A) = ROWS(B)
///
/// __Semantics__:
/// Returns the matrix product of the two matrices. The elements
/// of the resulting matrix
/// , are defined by:
///
/// __See also__: "COLUMNS", "ROWS", 
#[inline]
pub fn mmult<A: Matrix, B: Matrix>(a: A, b: B) -> FnMatrix2<A, B> {
    FnMatrix2("MMULT", a, b)
}

/// Creates a unit matrix of a specified dimension N.
///
/// __Syntax__: 
/// ```ods
///     MUNIT( N Integer )
/// ```
///
/// __Constraints__:
/// The dimension has to be greater than zero.
///
/// __Semantics__:
/// Creates the unit matrix (identity matrix) of dimension N.
#[inline]
pub fn munit<A: Number>(n: A) -> FnMatrix1<A> {
    FnMatrix1("MUNIT", n)
}

/// Returns the transpose of a matrix.
///
/// __Syntax__: 
/// ```ods
///     TRANSPOSE( A Array )
/// ```
///
/// __Constraints__:
/// None
///
/// __Semantics__:
/// Returns the transpose AT of a matrix A, i.e. rows and columns of the matrix 
/// are exchanged.
#[inline]
pub fn transpose<A: Matrix>(a: A) -> FnMatrix1<A> {
    FnMatrix1("TRANSPOSE", a)
}
