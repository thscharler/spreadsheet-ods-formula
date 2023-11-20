//!
//! Statistical functions.
//!

use crate::{Any, Criterion, FnNumberVar, Reference};

pub use crate::generated::stat::*;

/// Summary: Average the values of cells that meet multiple criteria in multiple ranges.
///
/// __Syntax__:
/// ```ods
///     AVERAGEIFS( Reference A ; Reference R1 ; Criterion C1 [ ; Reference R2 ; Criterion C2 ]... )
/// ```
/// __Returns__: Number
///
/// __Constraints__: Does not accept constant values as reference parameters.
///
/// __Semantics__: Averages the values of cells in the reference range A that meet the Criterion
/// C1 in the reference range R1 and the Criterion C2 in the reference range R2, and so on
/// (4.11.8). All reference ranges shall have the same dimension and size, else an Error is
/// returned. A logical AND is applied between each array result of each selection; a cell
/// of reference range A is evaluated only if the same position in each array is the result
/// of a Criterion match. If no numbers are in the result set to be averaged, an Error is returned.
///
/// The values returned may vary depending upon the HOST-USE-REGULAR-EXPRESSIONS or
/// HOST-USE-WILDCARDS or HOST-SEARCH-CRITERIA-MUST-APPLY-TO-WHOLE-CELL properties. 3.4
///
/// __See also__ AVERAGEIF 6.18.5, COUNTIFS 6.13.10, SUMIFS 6.16.63, Infix Operator "=" 6.4.7,
/// Infix Operator "<>" 6.4.8, Infix Operator Ordered Comparison ("<", "<=", ">", ">=") 6.4.9
#[inline]
pub fn averageifs<
    A: Reference + 'static,
    R: Reference + 'static,
    C: Criterion + 'static,
    const N: usize,
>(
    range: A,
    list: [(R, C); N],
) -> FnNumberVar {
    let mut param: Vec<Box<dyn Any>> = Vec::new();

    param.push(Box::new(range));
    for (r, c) in list {
        param.push(Box::new(r));
        param.push(Box::new(c));
    }

    FnNumberVar("SUMIFS", param)
}
