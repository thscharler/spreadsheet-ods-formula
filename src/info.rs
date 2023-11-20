//!
//! Information functions.
//!

pub use crate::generated::info::*;

use crate::{Any, Criterion, FnNumberVar, Reference};
use std::fmt::Write;

/// Parameter for CELL()
pub enum CellInfo {
    Address,
    Col,
    Colored,
    Contents,
    Filename,
    Format,
    Formula,
    Parentheses,
    Prefix,
    Protect,
    Row,
    Sheet,
    Type,
    Width,
}

impl Any for CellInfo {
    fn formula(&self, buf: &mut String) {
        let _ = write!(
            buf,
            "{}",
            match self {
                CellInfo::Address => "ADDRESS",
                CellInfo::Col => "COL",
                CellInfo::Colored => "COLORED",
                CellInfo::Contents => "CONTENTS",
                CellInfo::Filename => "FILENAME",
                CellInfo::Format => "FORMAT",
                CellInfo::Formula => "FORMULA",
                CellInfo::Parentheses => "PARENTHESES",
                CellInfo::Prefix => "PREFIX",
                CellInfo::Protect => "PROTECT",
                CellInfo::Row => "ROW",
                CellInfo::Sheet => "SHEET",
                CellInfo::Type => "TYPE",
                CellInfo::Width => "WIDTH",
            }
        );
    }
}

/// Parameter for INFO()
pub enum InfoInfo {
    Directory,
    MemAvail,
    MemUsed,
    NumFile,
    OSVersion,
    Origin,
    ReCalc,
    Release,
    System,
    TotMem,
}

impl Any for InfoInfo {
    fn formula(&self, buf: &mut String) {
        let _ = write!(
            buf,
            "{}",
            match self {
                InfoInfo::Directory => "directory",
                InfoInfo::MemAvail => "memavail",
                InfoInfo::MemUsed => "memused",
                InfoInfo::NumFile => "numfile",
                InfoInfo::OSVersion => "osversion",
                InfoInfo::Origin => "origin",
                InfoInfo::ReCalc => "recalc",
                InfoInfo::Release => "release",
                InfoInfo::System => "system",
                InfoInfo::TotMem => "totmem",
            }
        );
    }
}

/// Count the number of cells that meet multiple criteria in multiple ranges.
#[inline]
pub fn countifs<R: Reference + 'static, C: Criterion + 'static, const N: usize>(
    list: [(R, C); N],
) -> FnNumberVar {
    let mut param = Vec::new();

    for (r, c) in list {
        param.push(Box::new(r) as Box<dyn Any>);
        param.push(Box::new(c) as Box<dyn Any>);
    }

    FnNumberVar("COUNTIFS", param)
}
