use crate::FCriterion;

// /// Count the number of cells that meet multiple criteria in multiple ranges.
// #[inline]
// pub fn countifs(list: &[(FReference, FCriterion<A>)]) -> FNumber {
//     let mut param = create_param(list.len() * 2);
//     for (i, (r, c)) in list.iter().enumerate() {
//         param[2 * i].write(r);
//         param[2 * i + 1].write(c);
//     }
//     let param = unsafe { param_assume_init(param) };
//
//     FNumber(func("COUNTIFS", param.as_ref()))
// }
