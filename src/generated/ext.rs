//! 
//! OpenFormula defines two functions, DDE and HYPERLINK, for accessing 
//! external data.

use crate::*;
#[allow(unused_imports)]
use crate::ext::*;

/// Returns data from a DDE request
///
/// __Syntax__: 
/// ```ods
///     DDE( Server Text; Topic Text; Item Text )
/// ```
///
/// __Constraints__:
/// None
///
/// __Semantics__:
/// Performs a DDE request and returns its result. The request invokes the 
/// service Server on the topic named as Topic, requesting that it reply with 
/// the information on Item.
/// 
/// Evaluators may choose to not perform this function on every recalculation, 
/// but instead cache an answer and require a separate action to re-perform 
/// these requests. Evaluators shall perform this request on initial load when 
/// their security policies permit it.
/// 
/// Mode is an optional parameter that determines how the results are returned:
/// 
/// Data retrieved as text (not converted to number)
/// 
/// In an OpenDocument spreadsheet document the default table cell style is 
/// specified with table:default-cell-style-name. Its number:number-style 
/// specified by style:data-style-name specifies the locale to use in the 
/// conversion.
/// 
/// The DDE function is non-portable because it depends on availability of 
/// external programs (server parameter) and their interpretation of the topic 
/// and item parameters.
#[inline]
pub fn dde<A: Text, B: Text, C: Text>(server: A, topic: B, item: C) -> FnText3<A, B, C> {
    FnText3("DDE", server, topic, item)
}

/// Returns data from a DDE request
///
/// __Syntax__: 
/// ```ods
///     DDE( Server Text; Topic Text; Item Text; Mode Integer )
/// ```
///
/// __Constraints__:
/// None
///
/// __Semantics__:
/// Performs a DDE request and returns its result. The request invokes the 
/// service Server on the topic named as Topic, requesting that it reply with 
/// the information on Item.
/// 
/// Evaluators may choose to not perform this function on every recalculation, 
/// but instead cache an answer and require a separate action to re-perform 
/// these requests. Evaluators shall perform this request on initial load when 
/// their security policies permit it.
/// 
/// Mode is an optional parameter that determines how the results are returned:
/// 
/// Data retrieved as text (not converted to number)
/// 
/// In an OpenDocument spreadsheet document the default table cell style is 
/// specified with table:default-cell-style-name. Its number:number-style 
/// specified by style:data-style-name specifies the locale to use in the 
/// conversion.
/// 
/// The DDE function is non-portable because it depends on availability of 
/// external programs (server parameter) and their interpretation of the topic 
/// and item parameters.
#[inline]
pub fn dde_<A: Text, B: Text, C: Text, D: Number>(server: A, topic: B, item: C, mode: D) -> FnText4<A, B, C, D> {
    FnText4("DDE", server, topic, item, mode)
}

/// Creation of a hyperlink involving an evaluated expression.
///
/// __Syntax__: 
/// ```ods
///     HYPERLINK( IRI Text )
/// ```
///
/// __Constraints__:
/// None
///
/// __Semantics__:
/// The default for the second argument is the value of the first argument. The 
/// second argument value is returned.
/// 
/// In addition, hosting environments may interpret expressions containing 
/// HYPERLINK function calls as calling for an implementation-dependent 
/// creation of a hypertext link based on the expression containing the 
/// HYPERLINK function calls.
#[inline]
pub fn hyperlink<A: Text>(i_r_i: A) -> FnText1<A> {
    FnText1("HYPERLINK", i_r_i)
}

/// Creation of a hyperlink involving an evaluated expression.
///
/// __Syntax__: 
/// ```ods
///     HYPERLINK( IRI Text; FunctionResult Text|Number )
/// ```
///
/// __Constraints__:
/// None
///
/// __Semantics__:
/// The default for the second argument is the value of the first argument. The 
/// second argument value is returned.
/// 
/// In addition, hosting environments may interpret expressions containing 
/// HYPERLINK function calls as calling for an implementation-dependent 
/// creation of a hypertext link based on the expression containing the 
/// HYPERLINK function calls.
#[inline]
pub fn hyperlink_<A: Text, B: TextOrNumber>(i_r_i: A, function_result: B) -> FnText2<A, B> {
    FnText2("HYPERLINK", i_r_i, function_result)
}
