//! 
//! Evaluators shall support unsigned integer values and results of at least 48 
//! bits (values from 0 to 2^48-1 inclusive). Operations that receive or result 
//! in a value that cannot be represented within 48 bits are 
//! implementation-defined.

use crate::*;
#[allow(unused_imports)]
use crate::bit::*;

/// Returns bitwise “and” of its parameters
///
/// [documentfoundation->BITAND](https://wiki.documentfoundation.org/Documentation/Calc_Functions/BITAND)
///
/// __Syntax__: 
/// ```ods
///     BITAND( X: Integer; Y: Integer )
/// ```
///
/// __Constraints__:
/// X ≥ 0, Y ≥ 0
///
/// __Semantics__:
/// Returns bitwise “and” of its parameters. In the result, each bit 
/// position is 1 if and only if all parameters' bits at that position are also 
/// 1; else it is 0.
///
/// __See also__: [crate::of::bitor()], [crate::of::bitxor()], [crate::of::and()], 
#[inline]
pub fn bitand<A: Number, B: Number>(x: A, y: B) -> FnNumber2<A, B> {
    FnNumber2("BITAND", x, y)
}

/// Returns left shift of value X by N bits (“<<”)
///
/// [documentfoundation->BITLSHIFT](https://wiki.documentfoundation.org/Documentation/Calc_Functions/BITLSHIFT)
///
/// __Syntax__: 
/// ```ods
///     BITLSHIFT( X: Integer; N: Integer )
/// ```
///
/// __Constraints__:
/// X ≥ 0
///
/// __Semantics__:
/// Returns left shift of value X by N bit positions:
/// 
/// •If N < 0, return BITRSHIFT(X,-N)
/// 
/// •if N = 0, return X
/// 
/// •if N > 0, return X * 2^N
///
/// __See also__: [crate::of::bitand()], [crate::of::bitxor()], [crate::of::bitrshift()], 
#[inline]
pub fn bitlshift<A: Number, B: Number>(x: A, n: B) -> FnNumber2<A, B> {
    FnNumber2("BITLSHIFT", x, n)
}

/// Returns bitwise “or” of its parameters
///
/// [documentfoundation->BITOR](https://wiki.documentfoundation.org/Documentation/Calc_Functions/BITOR)
///
/// __Syntax__: 
/// ```ods
///     BITOR( X: Integer; Y: Integer )
/// ```
///
/// __Constraints__:
/// X ≥ 0, Y ≥ 0
///
/// __Semantics__:
/// Returns bitwise “or” of its parameters. In the result, each bit 
/// position is 1 if any of its parameters' bits at that position are also 1; 
/// else it is 0.
///
/// __See also__: [crate::of::bitand()], [crate::of::bitxor()], [crate::of::and()], 
#[inline]
pub fn bitor<A: Number, B: Number>(x: A, y: B) -> FnNumber2<A, B> {
    FnNumber2("BITOR", x, y)
}

/// Returns right shift of value X by N bits (“>>”)
///
/// [documentfoundation->BITRSHIFT](https://wiki.documentfoundation.org/Documentation/Calc_Functions/BITRSHIFT)
///
/// __Syntax__: 
/// ```ods
///     BITRSHIFT( X: Integer; N: Integer )
/// ```
///
/// __Constraints__:
/// X ≥ 0
///
/// __Semantics__:
/// Returns right shift of value X by N bit positions:
/// 
/// •If N < 0, return BITLSHIFT(X,-N)
/// 
/// •if N = 0, return X
/// 
/// •if N > 0, return INT(X / 2^N)
///
/// __See also__: [crate::of::bitand()], [crate::of::bitxor()], [crate::of::bitlshift()], [crate::of::int()], 
#[inline]
pub fn bitrshift<A: Number, B: Number>(x: A, n: B) -> FnNumber2<A, B> {
    FnNumber2("BITRSHIFT", x, n)
}

/// Returns bitwise “exclusive or” of its parameters
///
/// [documentfoundation->BITXOR](https://wiki.documentfoundation.org/Documentation/Calc_Functions/BITXOR)
///
/// __Syntax__: 
/// ```ods
///     BITXOR( X: Integer; Y: Integer )
/// ```
///
/// __Constraints__:
/// X ≥ 0, Y ≥ 0
///
/// __Semantics__:
/// Returns bitwise “exclusive or” (xor) of its parameters. In the result, 
/// each bit position is 1 if one or the other parameters' bits at that 
/// position are 1; else it is 0.
///
/// __See also__: [crate::of::bitand()], [crate::of::bitor()], [crate::of::or()], 
#[inline]
pub fn bitxor<A: Number, B: Number>(x: A, y: B) -> FnNumber2<A, B> {
    FnNumber2("BITXOR", x, y)
}
