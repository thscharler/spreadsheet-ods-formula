use crate::{Any, Criterion, FnNumberVar, Reference};

pub use crate::generated::stat::*;

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
    let mut param = Vec::new();

    param.push(Box::new(range) as Box<dyn Any>);
    for (r, c) in list {
        param.push(Box::new(r) as Box<dyn Any>);
        param.push(Box::new(c) as Box<dyn Any>);
    }

    FnNumberVar("SUMIFS", param)
}
