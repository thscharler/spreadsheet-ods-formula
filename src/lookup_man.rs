use crate::{FnAny2, FnAnyVar, FnText4, FnText5, Number, Param, Reference, Scalar, Text};

pub enum AddressAbs {
    RowAbsColAbs,
    RowAbsColRel,
    RowRelColAbs,
    RowRelColRel,
}

impl Param for AddressAbs {
    type Type = u32;

    fn as_param(&self) -> Self::Type {
        match self {
            AddressAbs::RowAbsColAbs => 1,
            AddressAbs::RowAbsColRel => 2,
            AddressAbs::RowRelColAbs => 3,
            AddressAbs::RowRelColRel => 4,
        }
    }
}

/// Returns a cell address (reference) as text.
#[inline]
pub fn address<A: Number, B: Number>(
    row: A,
    col: B,
    abs: AddressAbs,
) -> FnText4<A, B, <AddressAbs as Param>::Type, bool> {
    FnText4("ADDRESS", row, col, abs.as_param(), true)
}

/// Returns a cell address (reference) as text.
#[inline]
pub fn address_in_sheet<A: Number, B: Number, C: Text>(
    row: A,
    col: B,
    abs: AddressAbs,
    sheet: C,
) -> FnText5<A, B, <AddressAbs as Param>::Type, bool, C> {
    FnText5("ADDRESS", row, col, abs.as_param(), true, sheet)
}

/// Returns a cell address (reference) as text.
#[inline]
pub fn address_rc<A: Number, B: Number>(
    row: A,
    col: B,
    abs: AddressAbs,
) -> FnText4<A, B, <AddressAbs as Param>::Type, bool> {
    FnText4("ADDRESS", row, col, abs.as_param(), false)
}

/// Returns a cell address (reference) as text.
#[inline]
pub fn address_rc_in_sheet<A: Number, B: Number, C: Text>(
    row: A,
    col: B,
    abs: AddressAbs,
    sheet: C,
) -> FnText5<A, B, <AddressAbs as Param>::Type, bool, C> {
    FnText5("ADDRESS", row, col, abs.as_param(), false, sheet)
}

/// Return a value from a data pilot table.
#[inline]
pub fn getpivotdata<A: Text, B: Reference>(datafield: A, table: B) -> FnAny2<A, B> {
    FnAny2("GETPIVOTDATA", datafield, table)
}

/// Return a value from a data pilot table.
#[inline]
pub fn getpivotdata_fields<A: Text, B: Reference, F: Text, S: Scalar, const N: usize>(
    datafield: A,
    table: B,
    fields: [(F, S); N],
) -> FnAnyVar {
    let mut param = create_param(2 + fields.len() * 2);
    param[0].write(&datafield);
    param[1].write(&table);
    for (i, (n, sc)) in fields.iter().enumerate() {
        param[2 + 2 * i].write(n);
        param[2 + 2 * i + 1].write(sc);
    }
    let param = unsafe { param_assume_init(param) };

    FAny(func("GETPIVOTDATA", param.as_ref()))
}
