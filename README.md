
[![crates.io](https://img.shields.io/crates/v/spreadsheet-ods-formula.svg)](https://crates.io/crates/spreadsheet-ods-formula)
[![Documentation](https://docs.rs/spreadsheet-ods-formula/badge.svg)](https://docs.rs/spreadsheet_ods-formula)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](https://opensource.org/licenses/MIT)
[![License](https://img.shields.io/badge/license-APACHE-blue.svg)](https://www.apache.org/licenses/LICENSE-2.0)
![](https://tokei.rs/b1/github/thscharler/spreadsheet-ods-formula)

# ODS formulas

This library provides functions that map to ODS-functions.

```rust
use spreadsheet_ods::CellRef;
use spreadsheet_ods_formula::{cell, formula, of};

let f = formula(of::sin(cell!(0,0)));

assert_eq!(f, "of:=SIN([.A1])");
```

## License

This project is licensed under either of

* [Apache License, Version 2.0](https://www.apache.org/licenses/LICENSE-2.0)
  ([LICENSE-APACHE](LICENSE-APACHE))

* [MIT License](https://opensource.org/licenses/MIT)
  ([LICENSE-MIT](LICENSE-MIT))

at your option.

## Changes

[changes.md](https://github.com/thscharler/spreadsheet-ods-formula/blob/master/changes.md)