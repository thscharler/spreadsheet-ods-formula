
# ODS formulas

This library provides functions that map to ODS-functions.

```rust
use spreadsheet_ods::CellRef;
use spreadsheet_ods_formula::{cell, formula, of};

let f = formula(of::sin(cell!(0,0)));

assert_eq!(f, "of:=SIN([.A1])");
```
