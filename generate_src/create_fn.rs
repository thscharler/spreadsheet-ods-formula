#![allow(dead_code)]

use crate::error::DError;
use crate::mapp::{
    args, etc_fn, manual_fn, mod_file, mod_name, rectify_fn, ret_args, ret_type, returns, type_vars,
};
use crate::parse::{Func, Mod, Spec};
use std::fs::File;
use std::io::Read;
use std::io::Write;

fn main() -> Result<(), DError> {
    let mut txt = Vec::new();

    let mut f = File::open("generate_src/spec.txt")?;
    f.read_to_end(&mut txt)?;

    let txt = String::from_utf8_lossy(txt.as_ref());

    let mut it = txt.as_ref();
    let mut file = None;
    let mut mod_name_ = String::from("");
    loop {
        let (rest, spec) = parse::parse(&it).expect("mods");
        match spec {
            Spec::Mod(mod_) => {
                mod_name_ = mod_name(&mod_)?.into();

                println!("{:?}", mod_.name);

                file.replace(File::create(format!("generated/{}", mod_file(&mod_)?))?);
                let f = file.as_mut().expect("file");

                generate_mod(f, &mod_)?;
            }
            Spec::Func(mut fun) => {
                fun.mod_ = mod_name_.clone();

                // println!("{:?}", fun.name);
                if etc_fn(&fun) {
                    println!("Don't generate etc-fn {}.", fun.fun);
                } else if manual_fn(&fun) {
                    println!("Don't generate manual-fn {}.", fun.fun);
                } else {
                    let f = file.as_mut().expect("file");
                    rectify_fn(&mut fun)?;
                    generate_fn_family(f, &mut fun)?;
                }
            }
            Spec::Eof => {
                break;
            }
        }

        it = rest;
    }

    println!("fine.");

    Ok(())
}

fn generate_mod(f: &mut File, m: &Mod) -> Result<(), DError> {
    // for l in m.desc.lines() {
    //     writeln!(f, "//! {}", l)?;
    // }
    // writeln!(f)?;

    writeln!(f, "use crate::*;")?;
    writeln!(f, "#[allow(unused_imports)]")?;
    writeln!(f, "use crate::{}::*;", mod_name(&m)?)?;
    Ok(())
}

pub fn generate_fn_family(f: &mut File, fun: &mut Func) -> Result<(), DError> {
    let mut args = Vec::new();

    // pop trailing optionals.
    loop {
        if let Some(arg) = fun.args.pop() {
            if arg.vol {
                args.push(arg);
            } else {
                fun.args.push(arg);
                break;
            }
        } else {
            break;
        }
    }

    // reinstate one arg and generate.
    loop {
        generate_fn(f, fun)?;

        if let Some(arg) = args.pop() {
            fun.args.push(arg);
            fun.name.push('_');
        } else {
            break;
        }
    }

    Ok(())
}

pub fn generate_fn(f: &mut File, fun: &Func) -> Result<(), DError> {
    writeln!(f)?;

    // if fun.summary.len() > 0 {
    //     for l in fun.summary.lines() {
    //         writeln!(f, "/// {}", l)?;
    //     }
    // }
    //
    // if let Some(extra0) = &fun.extra0 {
    //     if extra0.len() > 0 {
    //         writeln!(f, "///")?;
    //         writeln!(f, "/// Arguments:")?;
    //         for l in extra0.lines() {
    //             writeln!(f, "/// {}", l)?;
    //         }
    //     }
    // }
    //
    // if let Some(constraints) = &fun.constraints {
    //     if constraints.len() > 0 {
    //         writeln!(f, "///")?;
    //         writeln!(f, "/// Constraints:")?;
    //         for l in constraints.lines() {
    //             writeln!(f, "/// {}", l)?;
    //         }
    //     }
    // }
    //
    // if let Some(extra1) = &fun.extra1 {
    //     if extra1.len() > 0 {
    //         writeln!(f, "///")?;
    //         writeln!(f, "/// Info2:")?;
    //         for l in extra1.lines() {
    //             writeln!(f, "/// {}", l)?;
    //         }
    //     }
    // }
    //
    // if fun.semantics.len() > 0 {
    //     writeln!(f, "///")?;
    //     writeln!(f, "/// Semantics:")?;
    //     for l in fun.semantics.lines() {
    //         writeln!(f, "/// {}", l)?;
    //     }
    // }
    //
    // if let Some(note) = &fun.note {
    //     if note.len() > 0 {
    //         writeln!(f, "///")?;
    //         writeln!(f, "/// Note:")?;
    //         for l in note.lines() {
    //             writeln!(f, "/// {}", l)?;
    //         }
    //     }
    // }
    //
    // if !fun.see_also.fnname.is_empty() {
    //     writeln!(f, "///")?;
    //     write!(f, "/// See also: ")?;
    //     for l in &fun.see_also.fnname {
    //         write!(f, "{:?}, ", l)?;
    //     }
    // }

    writeln!(f, "#[inline]")?;
    writeln!(
        f,
        "pub fn {}{}({}) -> {} {{",
        fun.name,
        type_vars(fun)?,
        args(fun)?,
        returns(fun)?
    )?;
    writeln!(
        f,
        "    {}(\"{}\", {})",
        ret_type(fun)?,
        fun.fun,
        ret_args(fun)?,
    )?;
    writeln!(f, "}}")?;

    Ok(())
}

mod mapp {
    use crate::error::{DError, DErrorString};
    use crate::parse::{Arg, Func, Mod};
    use std::fmt::Display;
    use std::fmt::Write;

    const TYPE_VARS: [&str; 8] = ["A", "B", "C", "D", "E", "F", "G", "H"];

    pub fn mod_file(mod_: &Mod) -> Result<String, DError> {
        let file = format!("{}.rs", mod_name(mod_)?);
        Ok(file)
    }

    pub fn type_(a: &Arg) -> Result<String, DError> {
        match a.type_.as_str() {
            _ => panic!("no type for {:?}", a.type_),
        }
    }

    pub fn type_vars(fun: &Func) -> Result<impl Display, DError> {
        let mut buf = String::new();

        if !fun.args.is_empty() {
            write!(buf, "<")?;
        }

        let mut t_idx = 0usize;
        for a in fun.args.iter() {
            if let Some((v, t)) = type_var(t_idx, fun, a)? {
                if t_idx > 0 {
                    write!(buf, ", ")?;
                }
                write!(buf, "{}: {}", v, t)?;
                t_idx += 1;
            }
        }

        if !fun.args.is_empty() {
            write!(buf, ">")?;
        }

        Ok(buf)
    }

    pub fn args(f: &Func) -> Result<String, DError> {
        let mut buf = String::new();

        let mut t_idx = 0usize;
        for (idx, arg) in f.args.iter().enumerate() {
            let aname = arg_name(f, arg)?;
            let (atype, atypevar) = arg_type(t_idx, f, arg)?;

            if idx > 0 {
                write!(buf, ", ")?;
            }
            if arg.opt {
                write!(buf, "{}: Option<{}>", aname, atype)?;
            } else {
                write!(buf, "{}: {}", aname, atype)?;
            }

            if atypevar {
                t_idx += 1;
            }
        }

        Ok(buf)
    }

    pub fn returns(f: &Func) -> Result<String, DError> {
        let mut buf = String::new();

        write!(buf, "{}", ret_type(f)?)?;

        if !f.args.is_empty() {
            write!(buf, "<")?;
        }

        let mut t_idx = 0usize;
        for (idx, arg) in f.args.iter().enumerate() {
            let (atype, atype_var) = arg_type(t_idx, f, arg)?;

            if idx > 0 {
                write!(buf, ", ")?;
            }
            if arg.opt {
                write!(buf, "Option<{}>", atype)?;
            } else {
                write!(buf, "{}", atype)?;
            }
            if atype_var {
                t_idx += 1;
            }
        }

        if !f.args.is_empty() {
            write!(buf, ">")?;
        }

        Ok(buf)
    }

    pub fn ret_args(f: &Func) -> Result<String, DError> {
        let mut buf = String::new();

        let mut t_idx = 0usize;
        for (_idx, a) in f.args.iter().enumerate() {
            if t_idx > 0 {
                write!(buf, ", ")?;
            }
            write!(buf, "{}", arg_name(f, a)?)?;
            t_idx += 1;
        }

        Ok(buf)
    }

    pub fn etc_fn(fun: &Func) -> bool {
        fun.etc
    }

    pub fn manual_fn(fun: &Func) -> bool {
        match fun.name.as_str() {
            "IF" => true,
            _ => false,
        }
    }

    pub fn rectify_fn(fun: &mut Func) -> Result<(), DError> {
        fun.name = fun.fun.to_lowercase().replace('.', "_");

        // fn name
        if fun.fun == "MATCH" {
            fun.name = "match_".into();
        } else if fun.fun == "MOD" {
            fun.name = "mod_".into();
        } else if fun.fun == "FALSE" {
            fun.name = "false_".into();
        } else if fun.fun == "TRUE" {
            fun.name = "true_".into();
        } else if fun.fun == "TYPE" {
            fun.name = "type_".into();
        } else if fun.fun == "YIELD" {
            fun.name = "yield_".into();
        }

        // set vol/opt
        for arg in fun.args.iter_mut().rev() {
            if arg.opt {
                arg.vol = true;
                arg.opt = false;
            } else {
                break;
            }
        }

        Ok(())
    }

    // type args + arg-trait
    pub fn type_var(idx: usize, fun: &Func, arg: &Arg) -> Result<Option<(String, String)>, DError> {
        let v = match (
            fun.mod_.as_str(),
            fun.fun.as_str(),
            arg.type_.as_str(),
            arg.ident.as_str(),
        ) {
            (_, "DAYS360", "Logical", "Method") => None,
            (_, "WEEKDAY", "Integer", "Type") => None,
            (_, "WEEKNUM", "Number", "Mode") => None,
            (_, "YEARFRAC", "Basis", "B") => None,
            (_, "ROMAN", "Integer", "Format") => None,
            (_, "ADDRESS", "Integer", "Abs") => None,
            (_, "MATCH", "Integer", "MatchType") => None,
            (_, "SUBTOTAL", "Integer", "Function") => None,
            (_, "CEILING", "Number", "Mode") => None,
            (_, "FLOOR", "Number", "Mode") => None,
            (_, "CELL", "Text", "Info_Type") => None,
            (_, "INFO", "Text", "Category") => None,
            (_, "NPER", "Number", "PayType") => None,

            ("fin", _, "Basis", "B") => None,
            ("fin", _, "Basis", "Basis") => None,
            ("fin", _, "Basis", "Bas") => None,
            ("fin", _, "Integer", "Frequency") => None,
            ("fin", _, "Number", "Frequency") => None,
            ("fin", _, "Integer", "Type") => None,

            ("matrix", _, "Array", "A") => Some((TYPE_VARS[idx], "Matrix")),
            ("matrix", _, "Array", "B") => Some((TYPE_VARS[idx], "Matrix")),

            (_, _, "Any", _) if !arg.rep => Some((TYPE_VARS[idx], "Any")),
            (_, _, "Array", _) if !arg.rep => Some((TYPE_VARS[idx], "Array")),
            (_, _, "ByteLength", _) => Some((TYPE_VARS[idx], "Number")),
            (_, _, "BytePosition", _) => Some((TYPE_VARS[idx], "Number")),
            (_, _, "Complex", _) => Some((TYPE_VARS[idx], "Number")),
            (_, _, "Criteria", _) => Some((TYPE_VARS[idx], "Criteria")),
            (_, _, "Criterion", _) => Some((TYPE_VARS[idx], "Criterion")),
            (_, _, "Database", _) => Some((TYPE_VARS[idx], "Database")),
            (_, _, "Date", _) => Some((TYPE_VARS[idx], "DateTime")),
            (_, _, "DateParam", _) => Some((TYPE_VARS[idx], "DateTime")),
            (_, _, "Error", _) => Some((TYPE_VARS[idx], "Any")),
            (_, _, "Field", _) => Some((TYPE_VARS[idx], "Field")),
            (_, _, "Integer", _) => Some((TYPE_VARS[idx], "Number")), // todo
            (_, _, "Logical", _) if !arg.rep => Some((TYPE_VARS[idx], "Logical")),
            (_, _, "Number", _) => Some((TYPE_VARS[idx], "Number")),
            (_, _, "Reference", _) => Some((TYPE_VARS[idx], "Reference")),
            (_, _, "ReferenceList", _) => Some((TYPE_VARS[idx], "Reference")),
            (_, _, "Scalar", _) => Some((TYPE_VARS[idx], "Scalar")),
            (_, _, "Text", _) if !arg.rep => Some((TYPE_VARS[idx], "Text")),
            (_, _, "TimeParam", _) => Some((TYPE_VARS[idx], "DateTime")),

            (_, _, "Any", _) if arg.rep => Some((TYPE_VARS[idx], "Sequence")),
            (_, _, "Array", _) if arg.rep => Some((TYPE_VARS[idx], "Sequence")),
            (_, _, "Logical", _) if arg.rep => Some((TYPE_VARS[idx], "Sequence")),
            (_, _, "Text", _) if arg.rep => Some((TYPE_VARS[idx], "Sequence")),
            (_, _, "ComplexSequence", _) => Some((TYPE_VARS[idx], "Sequence")),
            (_, _, "DateSequence", _) => Some((TYPE_VARS[idx], "Sequence")),
            (_, _, "LogicalSequence", _) => Some((TYPE_VARS[idx], "Sequence")),
            (_, _, "NumberSequence", _) => Some((TYPE_VARS[idx], "Sequence")),
            (_, _, "NumberSequenceList", _) => Some((TYPE_VARS[idx], "Sequence")),
            (_, _, "Logical|NumberSequenceList", _) => Some((TYPE_VARS[idx], "Sequence")),

            (_, _, "Text|Number", _) => Some((TYPE_VARS[idx], "TextOrNumber")),
            (_, _, "TextOrNumber", _) => Some((TYPE_VARS[idx], "TextOrNumber")),
            (_, _, "Text|Reference", _) => Some((TYPE_VARS[idx], "TextOrReference")),
            (_, _, "ReferenceList|Reference", _) => Some((TYPE_VARS[idx], "Reference")),
            (_, _, "Reference|Array", _) => Some((TYPE_VARS[idx], "ReferenceOrArray")),
            (_, _, "ReferenceList|Array", _) => Some((TYPE_VARS[idx], "ReferenceOrArray")),
            (_, _, "Number|Array", _) => Some((TYPE_VARS[idx], "NumberOrArray")),
            (_, _, "Number or Array", _) => Some((TYPE_VARS[idx], "NumberOrArray")),
            (_, _, "Integer|Array", _) => Some((TYPE_VARS[idx], "NumberOrArray")),

            _ => {
                return Err(DErrorString(format!(
                    "no type_var for {:?} {:?} -- {:?}",
                    arg.ident, arg.type_, fun
                ))
                .into());
            }
        };

        if let Some((_v, t)) = v {
            if t != "Sequence" && arg.rep {
                return Err(DErrorString(format!(
                    "repeat is set for non-sequence {:?} {:?} -- {:?}",
                    arg.ident, arg.type_, fun
                ))
                .into());
            }
        }

        return Ok(v.map(|(v, t)| (v.into(), t.into())));
    }

    // argument type
    pub fn arg_type(idx: usize, fun: &Func, arg: &Arg) -> Result<(String, bool), DError> {
        let v = match (
            fun.mod_.as_str(),
            fun.fun.as_str(),
            arg.type_.as_str(),
            arg.ident.as_str(),
        ) {
            (_, "DAYS360", "Logical", "Method") => ("Days360Method", false),
            (_, "WEEKDAY", "Integer", "Type") => ("WeekdayMethod", false),
            (_, "WEEKNUM", "Number", "Mode") => ("WeeknumMethod", false),
            (_, "YEARFRAC", "Basis", "B") => ("YearFracMethod", false),
            (_, "ROMAN", "Integer", "Format") => ("RomanStyle", false),
            (_, "ADDRESS", "Integer", "Abs") => ("AddressAbs", false),
            (_, "MATCH", "Integer", "MatchType") => ("MatchType", false),
            (_, "SUBTOTAL", "Integer", "Function") => ("SubtotalFunction", false),
            (_, "CEILING", "Number", "Mode") => ("RoundingMode", false),
            (_, "FLOOR", "Number", "Mode") => ("RoundingMode", false),
            (_, "CELL", "Text", "Info_Type") => ("CellInfo", false),
            (_, "INFO", "Text", "Category") => ("InfoInfo", false),
            (_, "NPER", "Number", "PayType") => ("PayType", false),

            ("fin", _, "Basis", "B") => ("YearFracMethod", false),
            ("fin", _, "Basis", "Basis") => ("YearFracMethod", false),
            ("fin", _, "Basis", "Bas") => ("YearFracMethod", false),
            ("fin", _, "Integer", "Frequency") => ("Frequency", false),
            ("fin", _, "Number", "Frequency") => ("Frequency", false),
            ("fin", _, "Integer", "Type") => ("MaturityDate", false),

            ("matrix", _, "Array", "A") => (TYPE_VARS[idx], true),
            ("matrix", _, "Array", "B") => (TYPE_VARS[idx], true),

            (_, _, "Any", _) => (TYPE_VARS[idx], true),
            (_, _, "Array", _) => (TYPE_VARS[idx], true),
            (_, _, "ByteLength", _) => (TYPE_VARS[idx], true),
            (_, _, "BytePosition", _) => (TYPE_VARS[idx], true),
            (_, _, "Complex", _) => (TYPE_VARS[idx], true),
            (_, _, "Criteria", _) => (TYPE_VARS[idx], true),
            (_, _, "Criterion", _) => (TYPE_VARS[idx], true),
            (_, _, "Database", _) => (TYPE_VARS[idx], true),
            (_, _, "Date", _) => (TYPE_VARS[idx], true),
            (_, _, "DateParam", _) => (TYPE_VARS[idx], true),
            (_, _, "Error", _) => (TYPE_VARS[idx], true),
            (_, _, "Field", _) => (TYPE_VARS[idx], true),
            (_, _, "Integer", _) => (TYPE_VARS[idx], true),
            (_, _, "Logical", _) => (TYPE_VARS[idx], true),
            (_, _, "Number", _) => (TYPE_VARS[idx], true),
            (_, _, "Reference", _) => (TYPE_VARS[idx], true),
            (_, _, "ReferenceList", _) => (TYPE_VARS[idx], true),
            (_, _, "Scalar", _) => (TYPE_VARS[idx], true),
            (_, _, "Text", _) => (TYPE_VARS[idx], true),
            (_, _, "TimeParam", _) => (TYPE_VARS[idx], true),

            (_, _, "ComplexSequence", _) => (TYPE_VARS[idx], true),
            (_, _, "DateSequence", _) => (TYPE_VARS[idx], true),
            (_, _, "LogicalSequence", _) => (TYPE_VARS[idx], true),
            (_, _, "NumberSequence", _) => (TYPE_VARS[idx], true),
            (_, _, "NumberSequenceList", _) => (TYPE_VARS[idx], true),
            (_, _, "Logical|NumberSequenceList", _) => (TYPE_VARS[idx], true),

            (_, _, "Text|Number", _) => (TYPE_VARS[idx], true),
            (_, _, "TextOrNumber", _) => (TYPE_VARS[idx], true),
            (_, _, "Text|Reference", _) => (TYPE_VARS[idx], true),
            (_, _, "ReferenceList|Reference", _) => (TYPE_VARS[idx], true),
            (_, _, "Reference|Array", _) => (TYPE_VARS[idx], true),
            (_, _, "ReferenceList|Array", _) => (TYPE_VARS[idx], true),
            (_, _, "Number|Array", _) => (TYPE_VARS[idx], true),
            (_, _, "Number or Array", _) => (TYPE_VARS[idx], true),
            (_, _, "Integer|Array", _) => (TYPE_VARS[idx], true),

            _ => {
                return Err(
                    DErrorString(format!("no type for {:?} -- {:?}", arg.type_, fun)).into(),
                )
            }
        };

        if arg.opt {}

        Ok((v.0.into(), v.1))
    }

    // argument name
    #[allow(unreachable_code)]
    pub fn arg_name(_fun: &Func, arg: &Arg) -> Result<String, DError> {
        let mut buf = String::new();

        match arg.ident.as_str() {
            "Ref" => write!(buf, "ref_")?,
            "Const" => write!(buf, "const_")?,
            "Type" => write!(buf, "type_")?,
            "Yield" => write!(buf, "yield_")?,
            "α" => write!(buf, "alpha")?,
            "β" => write!(buf, "beta")?,
            _ => {
                let mut last_c = ' ';
                for (i, c) in arg.ident.chars().enumerate() {
                    if c.is_uppercase() {
                        if i == 0 {
                            write!(buf, "{}", c.to_lowercase())?;
                        } else if last_c == '_' {
                            write!(buf, "{}", c.to_lowercase())?;
                        } else {
                            write!(buf, "_{}", c.to_lowercase())?;
                        }
                    } else {
                        write!(buf, "{}", c)?;
                    }
                    last_c = c;
                }
            }
        }

        Ok(buf)
    }

    pub fn ret_type(fun: &Func) -> Result<String, DError> {
        let Some(ret) = &fun.ret else {
            return Ok(format!("FnNumber{}", fun.args.len()));
        };

        let ty = match (fun.mod_.as_str(), ret.as_str()) {
            ("matrix", "Array") => format!("FnMatrix{}", fun.args.len()),

            (_, "Any") => format!("FnAny{}", fun.args.len()),
            (_, "Array") => format!("FnArray{}", fun.args.len()),
            (_, "ByteLength") => format!("FnNumber{}", fun.args.len()),
            (_, "BytePosition") => format!("FnNumber{}", fun.args.len()),
            (_, "Complex") => format!("FnNumber{}", fun.args.len()),
            (_, "Currency") => format!("FnNumber{}", fun.args.len()),
            (_, "Database") => format!("FnNumber{}", fun.args.len()),
            (_, "Date") => format!("FnNumber{}", fun.args.len()),
            (_, "DateTime") => format!("FnNumber{}", fun.args.len()),
            (_, "Error") => format!("FnAny{}", fun.args.len()),
            (_, "Integer") => format!("FnNumber{}", fun.args.len()),
            (_, "Logical") => format!("FnLogical{}", fun.args.len()),
            (_, "Matrix") => format!("FnMatrix{}", fun.args.len()),
            (_, "Number") => format!("FnNumber{}", fun.args.len()),
            (_, "Percentage") => format!("FnNumber{}", fun.args.len()),
            (_, "Reference") => format!("FnReference{}", fun.args.len()),
            (_, "String") => format!("FnText{}", fun.args.len()),
            (_, "Text") => format!("FnText{}", fun.args.len()),
            (_, "Time") => format!("FnNumber{}", fun.args.len()),

            (_, "ComplexSequence") => format!("FnNumber{}", fun.args.len()),

            (_, "Any (see below)") => format!("FnAny{}", fun.args.len()),
            (_, "Number|Text") => format!("FnText{}", fun.args.len()),
            (_, "Text or Number") => format!("FnText{}", fun.args.len()),
            (_, "Information about position, formatting properties or content") => {
                format!("FnAny{}", fun.args.len())
            }
            (_, "Number ≥ 1") => format!("FnNumber{}", fun.args.len()),
            (_, "Number or Array") => format!("FnArray{}", fun.args.len()),

            _ => {
                return Err(
                    DErrorString(format!("no return type for {:?} -- {:?}", ret, fun)).into(),
                )
            }
        };

        Ok(ty)
    }

    pub fn mod_name(mod_: &Mod) -> Result<&'static str, DError> {
        let v = match mod_.name.as_str() {
            "Matrix Functions" => "matrix",
            "Bit operation functions" => "bit",
            "Byte-position text functions" => "textb",
            "Complex Number Functions" => "complex",
            "Database Functions" => "db",
            "Date and Time Functions" => "date",
            "External Access Functions" => "ext",
            "Financial Functions" => "fin",
            "Information Functions" => "info",
            "Lookup Functions" => "lookup",
            "Logical Functions" => "logic",
            "Mathematical Functions" => "math",
            "Rounding Functions" => "round",
            "Statistical Functions" => "stat",
            "Number Representation Conversion Functions" => "conv",
            "Text Functions" => "text",
            _ => return Err(DErrorString(format!("no match for {:?}", mod_.name)).into()),
        };

        Ok(v)
    }
}

mod parse {
    use kparse::combinators::{pchar, track};
    use kparse::prelude::*;
    use kparse::KParseError;
    #[cfg(debug_assertions)]
    use kparse::ParseSpan;
    use kparse::{define_span, Code};
    use nom::branch::alt;
    use nom::bytes::complete::{tag, take_while, take_while1};
    use nom::combinator::{opt, peek, recognize};
    use nom::error::ParseError;
    use nom::sequence::{delimited, tuple};
    use nom::{AsChar, IResult, InputIter, InputLength, InputTake, Offset, Parser, Slice};
    use std::fmt::{Debug, Display, Formatter};
    use std::mem;
    use std::ops::{RangeFrom, RangeTo};

    #[derive(Debug, PartialEq, Clone, Copy, Eq)]
    pub enum SpecCode {
        NomError,

        General,
        Header2,
        Header3,
        FnName,
        Summary,
        Syntax,
        Returns,
        Constraints,
        Note,
        Semantics,
        SeeAlso,

        Sem0,
        Sem1,

        TypeMod,
        Type,
        Ident,
        Default,
        Etc,

        Word,

        CarriageReturn,
        NewLine,
        Blank,
        WhiteSpace,
        Bracket,
        Brace,
        Paren,
        Semicolon,
        Eof,
    }

    impl Display for SpecCode {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self)
        }
    }

    impl Code for SpecCode {
        const NOM_ERROR: Self = Self::NomError;
    }

    #[derive(Debug, Clone)]
    pub enum Spec {
        Mod(Mod),
        Func(Func),
        Eof,
    }

    #[derive(Debug, Clone)]
    pub struct Mod {
        pub name: String,
        pub desc: Text,
    }

    #[derive(Debug, Clone)]
    pub struct Func {
        pub mod_: String,

        pub name: String,
        pub fun: String,
        pub args: Vec<Arg>,
        pub etc: bool,
        pub ret: Option<String>,

        pub summary: Text,
        pub extra0: Option<Text>,
        pub constraints: Option<Text>,
        pub extra1: Option<Text>,
        pub semantics: Option<Text>,
        pub note: Option<Text>,
        pub see_also: SeeAlso,
    }

    #[derive(Debug, Clone)]
    pub struct Arg {
        pub type_mod: Option<String>,
        pub type_: String,
        pub ident: String,
        pub default: Option<String>,
        pub opt: bool,
        pub vol: bool,
        pub rep: bool,
    }

    #[derive(Debug, Clone)]
    pub struct Returns {
        pub type_: String,
    }

    #[derive(Debug, Clone)]
    pub struct SeeAlso {
        pub fnname: Vec<String>,
    }

    #[derive(Debug, Clone)]
    pub struct Text {
        pub txt: Vec<String>,
    }

    impl PartialEq for Spec {
        fn eq(&self, other: &Self) -> bool {
            mem::discriminant(self) == mem::discriminant(other)
        }
    }

    impl Mod {
        pub fn from(name: &str, desc: Text) -> Self {
            Self {
                name: name.trim().into(),
                desc,
            }
        }
    }

    impl Text {
        pub fn from(str: &str) -> Self {
            let tracker = Track::new_tracker::<SpecCode, _>();
            let str = Track::new_span(&tracker, str.as_ref());

            let mut lines = Vec::new();
            let mut line = String::new();

            let mut rest_loop = str;
            loop {
                let rest2 = match word(rest_loop) {
                    Ok((rest, v)) => {
                        if line.len() + v.len() > 75 {
                            lines.push(line);
                            line = String::new();
                        }
                        line.push_str(*v.fragment());

                        rest
                    }
                    Err(_) => rest_loop,
                };
                let rest2 = match whitespace1(rest2) {
                    Ok((rest, _v)) => {
                        if line.len() > 0 {
                            line.push(' ');
                        }
                        rest
                    }
                    Err(_) => rest2,
                };
                let rest2 = match carriage_return(rest2) {
                    Ok((rest, _)) => rest,
                    Err(_) => rest2,
                };
                let rest2 = match newline(rest2) {
                    Ok((rest, _)) => {
                        if let Some(l) = lines.last() {
                            if l.len() > 0 || line.len() > 0 {
                                lines.push(line);
                            }
                        } else {
                            lines.push(line);
                        }
                        line = String::new();

                        rest
                    }
                    Err(_) => rest2,
                };
                if rest2.len() == 0 {
                    break;
                }

                rest_loop = rest2;
            }
            if line.len() > 0 {
                lines.push(line);
            }

            if let Some(last) = lines.last() {
                if last.len() == 0 {
                    lines.pop();
                }
            }

            Self { txt: lines }
        }

        pub fn len(&self) -> usize {
            self.txt.len()
        }

        pub fn lines(&self) -> impl Iterator<Item = &String> {
            self.txt.iter()
        }
    }

    define_span!(pub Span = SpecCode, str);
    // pub type Span<'a> = &'a str;
    pub type ParserResult<'s, O> = kparse::ParserResult<SpecCode, Span<'s>, O>;
    pub type TokenizerResult<'s> = kparse::TokenizerResult<SpecCode, Span<'s>, Span<'s>>;
    pub type NomResult<'s> = kparse::ParserResult<SpecCode, Span<'s>, Span<'s>>;
    pub type ParserError<'s> = kparse::ParserError<SpecCode, Span<'s>>;

    pub fn parse(txt: &str) -> Result<(&str, Spec), bool> {
        let tracker = Track::new_tracker::<SpecCode, _>();
        let input = Track::new_span(&tracker, txt.as_ref());

        match parse_spec(input) {
            Ok((rest, v)) => {
                let offset = input.offset(&rest);
                Ok((&txt[offset..], v))
            }
            Err(nom::Err::Error(e)) => {
                println!("{:#?}", e);
                let r = tracker.results();
                println!("{:#?}", r);
                Err(false)
            }
            Err(nom::Err::Incomplete(_)) => {
                unreachable!()
            }
            Err(nom::Err::Failure(_)) => {
                unreachable!()
            }
        }
    }

    #[inline]
    fn parse_spec(input: Span<'_>) -> ParserResult<'_, Spec> {
        match parse_eof(input) {
            Ok((rest, _)) => {
                return Ok((rest, Spec::Eof));
            }
            Err(_) => {}
        };

        match parse_header2(input) {
            Ok((rest, mod_name)) => {
                let (rest2, header3) = parser_header3(rest)?;
                if *header3.fragment() == "General\r" {
                    let (rest2, desc) = parse_general(rest2)?;
                    return Ok((rest2, Spec::Mod(Mod::from(mod_name.fragment(), desc))));
                } else {
                    return Ok((
                        rest,
                        Spec::Mod(Mod::from(mod_name.fragment(), Text::from(""))),
                    ));
                }
            }
            Err(_) => {
                let (rest2, _fnname) = parser_header3(input)?;
                let (rest2, (syntax1, extra0_1)) = parse_syntax(rest2)?;
                let (rest2, summary) = parse_summary(rest2)?;
                let (rest2, (syntax2, extra0_2)) = parse_syntax(rest2)?;
                let (rest2, returns) = opt(parse_returns)(rest2)?;
                let (rest2, constraints) = opt(parse_constraints)(rest2)?;
                let (rest2, semantics) = opt(parse_semantics)(rest2)?;
                let (rest2, note) = opt(parse_note)(rest2)?;
                let (rest2, see_also) = opt(parse_see_also)(rest2)?;

                let syntax = syntax1.or(syntax2).expect("syntax");
                let extra0 = extra0_1.or(extra0_2);
                let (extra1, semantics) = if let Some((extra1, semantics)) = semantics {
                    (extra1, Some(semantics))
                } else {
                    (None, None)
                };
                let see_also = see_also.unwrap_or(SeeAlso { fnname: vec![] });

                let fun = Func {
                    mod_: "".to_string(),
                    name: syntax.0.into(),
                    fun: syntax.0.into(),
                    args: syntax.1,
                    etc: syntax.2,
                    ret: returns.map(|v| v.fragment().trim().into()),
                    summary,
                    extra0,
                    constraints,
                    extra1,
                    semantics,
                    note,
                    see_also,
                };

                return Ok((rest2, Spec::Func(fun)));
            }
        };
    }

    #[inline]
    fn parse_header2(input: Span<'_>) -> ParserResult<'_, Span<'_>> {
        let (rest, (_, _, _, _, _, v)) = track(
            SpecCode::Header2,
            tuple((
                opt(pchar('\n')),
                take_while1(|c: char| c.is_ascii_digit()),
                pchar('.'),
                take_while1(|c: char| c.is_ascii_digit()),
                pchar(' '),
                take_while(|c: char| c != '\n'),
            )),
        )(input)
        .with_code(SpecCode::Header2)?;

        Ok((rest, v))
    }

    #[inline]
    fn tok_header2_num(input: Span<'_>) -> ParserResult<'_, Span<'_>> {
        let (rest, v) = recognize(tuple((
            take_while1(|c: char| c.is_ascii_digit()),
            pchar('.'),
            take_while1(|c: char| c.is_ascii_digit()),
            pchar(' '),
        )))(input)
        .with_code(SpecCode::Header2)?;

        Ok((rest, v))
    }

    #[inline]
    fn parser_header3(input: Span<'_>) -> ParserResult<'_, Span<'_>> {
        let (rest, (_, _, _, _, _, _, _, v)) = track(
            SpecCode::Header3,
            tuple((
                blank,
                take_while1(|c: char| c.is_ascii_digit()),
                pchar('.'),
                take_while1(|c: char| c.is_ascii_digit()),
                pchar('.'),
                take_while1(|c: char| c.is_ascii_digit()),
                pchar(' '),
                take_while(|c: char| c != '\n'),
            )),
        )(input)
        .with_code(SpecCode::Header3)?;

        Ok((rest, v))
    }

    #[inline]
    fn tok_header3_num(input: Span<'_>) -> ParserResult<'_, Span<'_>> {
        let (rest, v) = recognize(tuple((
            take_while1(|c: char| c.is_ascii_digit()),
            pchar('.'),
            take_while1(|c: char| c.is_ascii_digit()),
            pchar('.'),
            take_while1(|c: char| c.is_ascii_digit()),
            opt(pchar(' ')),
        )))(input)
        .with_code(SpecCode::Header3)?;

        Ok((rest, v))
    }

    #[inline]
    fn parse_general(input: Span<'_>) -> ParserResult<'_, Text> {
        let (rest, v) = track(SpecCode::General, fparse_until('\n', tok_header3_num))(input)
            .with_code(SpecCode::General)?;

        Ok((rest, Text::from(v.fragment())))
    }

    #[inline]
    fn parse_summary(input: Span<'_>) -> ParserResult<'_, Text> {
        let (rest, (_, _, v)) = track(
            SpecCode::Summary,
            tuple((
                blank,
                tag("Summary:"),
                fparse_until(
                    '\n',
                    alt((
                        tag("Syntax:"),
                        tag("Returns:"),
                        tag("Constraints:"),
                        tag("Semantics:"),
                        tag("Note:"),
                        tag("See also"),
                    )),
                ),
            )),
        )(input)
        .with_code(SpecCode::Summary)?;

        Ok((rest, Text::from(v.fragment())))
    }

    #[inline]
    fn parse_syntax(
        input: Span<'_>,
    ) -> ParserResult<'_, (Option<(&'_ str, Vec<Arg>, bool)>, Option<Text>)> {
        match peek(tuple((blank, tag("Syntax:"))))(input) {
            Ok((_, _)) => {}
            Err(_) => {
                return Ok((input, (None, None)));
            }
        }

        let (rest, (_, _, fun, safety, x)) = track(
            SpecCode::Syntax,
            tuple((
                blank,
                tag("Syntax:"),
                syntax_syntax,
                take_while(|c: char| c != '\n'),
                fparse_until(
                    '\n',
                    alt((
                        tag("Summary:"),
                        tag("Returns:"),
                        tag("Constraints:"),
                        tag("Semantics:"),
                        tag("Note:"),
                        tag("See also"),
                    )),
                ),
            )),
        )(input)
        .with_code(SpecCode::Syntax)?;

        if safety.len() > 0 {
            return Err(nom::Err::Error(ParserError::new(SpecCode::Syntax, safety)));
        }

        Ok((rest, (Some(fun), Some(Text::from(x.fragment())))))
    }

    #[inline]
    fn syntax_syntax(input: Span<'_>) -> ParserResult<'_, (&'_ str, Vec<Arg>, bool)> {
        Track.enter(SpecCode::Syntax, input);

        let rest = input;
        let (rest, (_, fnname, _, _, _)) = tuple((
            whitespace,
            syntax_fnname,
            whitespace,
            syntax_paren,
            whitespace,
        ))(rest)
        .with_code(SpecCode::Syntax)
        .track()?;

        let mut args = Vec::new();
        let mut etc = false;

        let mut opt_stack = 0;
        let mut rep_stack = 0;

        let mut rest_loop = rest;
        let rest = 'args: loop {
            let rest2 = rest_loop;

            let (rest2, v) = syntax_bracket(rest2)?;
            opt_stack += v;

            let (rest2, v) = syntax_brace(rest2)?;
            rep_stack += v;

            let (rest2, (type_mod, _, type_, _, ident, _, default)) = tuple((
                opt(syntax_type_mod),
                whitespace,
                syntax_type,
                whitespace,
                syntax_ident,
                whitespace,
                syntax_default,
            ))(rest2)
            .with_code(SpecCode::Syntax)
            .track()?;

            if !ident.fragment().is_empty() {
                args.push(Arg {
                    type_mod: type_mod.map(|v| (*v.fragment()).into()),
                    type_: (*type_.fragment()).into(),
                    ident: (*ident.fragment()).into(),
                    default: default.map(|v| (*v.fragment()).into()),
                    opt: opt_stack > 0,
                    vol: false,
                    rep: rep_stack > 0,
                });
            }

            let (rest2, v) = syntax_bracket(rest2)?;
            opt_stack += v;

            let (rest2, v) = syntax_bracket_c(rest2)?;
            opt_stack -= v;

            let (rest2, v) = syntax_bracket(rest2)?;
            opt_stack += v;

            let (rest2, v) = syntax_brace(rest2)?;
            rep_stack += v;

            let (rest2, v) = syntax_brace_plus_c(rest2)?;
            rep_stack -= v;

            let rest2 = match syntax_etc(rest2) {
                Ok((rest, _)) => {
                    etc = true;
                    rest
                }
                Err(_) => rest2,
            };

            let rest2 = match tok_semicolon(rest2) {
                Ok((r, _v)) => r,
                Err(_) => break 'args rest2,
            };

            rest_loop = rest2;
        };

        let (rest, _) = syntax_parenc(rest).track()?;

        // alt syntax
        let rest = match track(
            SpecCode::Syntax,
            tuple::<_, _, ParserError<'_>, _>((tag("or "), take_while(|c: char| c != '\r'))),
        )(rest)
        {
            Ok((r, _)) => r,
            Err(_) => rest,
        };

        let (rest, _) = carriage_return(rest).track()?;

        Track.ok(rest, input, (*fnname.fragment(), args, etc))
    }

    #[inline]
    fn syntax_fnname(input: Span<'_>) -> ParserResult<'_, Span<'_>> {
        let (rest, v) = track(
            SpecCode::FnName,
            take_while(|c: char| c.is_ascii_alphanumeric() || c == '.'),
        )(input)
        .with_code(SpecCode::FnName)?;

        Ok((rest, v))
    }

    #[inline]
    fn syntax_type_mod(input: Span<'_>) -> ParserResult<'_, Span<'_>> {
        let (rest, v) =
            track(SpecCode::TypeMod, tag("ForceArray"))(input).with_code(SpecCode::TypeMod)?;

        Ok((rest, v))
    }

    #[inline]
    fn syntax_type(input: Span<'_>) -> ParserResult<'_, Span<'_>> {
        let (rest, v) = track(
            SpecCode::Type,
            take_while(|c: char| c.is_ascii_alphabetic() || c == '|'),
        )(input)
        .with_code(SpecCode::Type)?;

        Ok((rest, v))
    }

    #[inline]
    fn syntax_ident(input: Span<'_>) -> ParserResult<'_, Span<'_>> {
        let (rest, v) = track(
            SpecCode::Ident,
            take_while(|c: char| {
                c.is_ascii_alphanumeric()
                    || c == '_'
                    || c == 'α'
                    || c == 'β'
                    || c == 'λ'
                    || c == 'μ'
                    || c == 'σ'
            }),
        )(input)
        .with_code(SpecCode::Ident)?;

        Ok((rest, v))
    }

    #[inline]
    fn syntax_default(input: Span<'_>) -> ParserResult<'_, Option<Span<'_>>> {
        let (rest, v) = track(
            SpecCode::Default,
            opt(delimited(
                tuple((pchar('='), whitespace)),
                alt((
                    tag("FALSE"),
                    tag("TRUE()"),
                    tag("TRUE"),
                    tag("12"),
                    tag("10"),
                    tag("0.1"),
                    tag("0"),
                    tag("1"),
                    tag("2"),
                    tag("3"),
                )),
                whitespace,
            )),
        )(input)
        .with_code(SpecCode::Default)?;

        Ok((rest, v))
    }

    #[inline]
    fn syntax_paren(input: Span<'_>) -> ParserResult<'_, Span<'_>> {
        let (rest, (v, _)) = tuple((pchar('('), whitespace))(input).with_code(SpecCode::Paren)?;
        Ok((rest, v))
    }

    #[inline]
    fn syntax_parenc(input: Span<'_>) -> ParserResult<'_, Span<'_>> {
        let (rest, (v, _)) = tuple((pchar(')'), whitespace))(input).with_code(SpecCode::Paren)?;
        Ok((rest, v))
    }

    #[inline]
    fn syntax_brace(input: Span<'_>) -> ParserResult<'_, i32> {
        let mut count = 0;

        let rest = match track(SpecCode::Brace, tuple((pchar('{'), whitespace)))(input)
            .with_code(SpecCode::Brace)
        {
            Ok((r, _v)) => {
                count += 1;
                r
            }
            Err(_) => input,
        };

        Ok((rest, count))
    }

    #[inline]
    fn syntax_brace_plus_c(input: Span<'_>) -> ParserResult<'_, i32> {
        let mut count = 0;

        let mut rest_loop = input;
        let rest = 'opt: loop {
            let rest2 = match track(SpecCode::Brace, tuple((pchar('}'), pchar('+'), whitespace)))(
                rest_loop,
            )
            .with_code(SpecCode::Brace)
            {
                Ok((r, _v)) => {
                    count += 1;
                    r
                }
                Err(_) => break 'opt rest_loop,
            };

            rest_loop = rest2;
        };

        Ok((rest, count))
    }

    #[inline]
    fn syntax_bracket(input: Span<'_>) -> ParserResult<'_, i32> {
        let mut count = 0;

        let rest = match track(SpecCode::Bracket, tuple((pchar('['), whitespace)))(input)
            .with_code(SpecCode::Bracket)
        {
            Ok((r, _v)) => {
                count += 1;
                r
            }
            Err(_) => input,
        };

        Ok((rest, count))
    }

    #[inline]
    fn syntax_bracket_c(input: Span<'_>) -> ParserResult<'_, i32> {
        let mut count = 0;

        let mut rest_loop = input;
        let rest = 'opt: loop {
            let rest2 = match track(SpecCode::Bracket, tuple((pchar(']'), whitespace)))(rest_loop)
                .with_code(SpecCode::Bracket)
            {
                Ok((r, _v)) => {
                    count += 1;
                    r
                }
                Err(_) => break 'opt rest_loop,
            };

            rest_loop = rest2;
        };

        Ok((rest, count))
    }

    fn syntax_etc(input: Span<'_>) -> ParserResult<'_, Span<'_>> {
        match track(SpecCode::Etc, tuple((tag("..."), whitespace)))(input).with_code(SpecCode::Etc)
        {
            Ok((rest, _)) => Ok((rest, rest.take(0))),
            Err(e) => Err(e),
        }
    }

    fn tok_semicolon(input: Span<'_>) -> ParserResult<'_, Span<'_>> {
        let (rest, (v, _)) = track(SpecCode::Semicolon, tuple((pchar(';'), whitespace)))(input)
            .with_code(SpecCode::Semicolon)?;
        Ok((rest, v))
    }

    #[inline]
    fn parse_returns(input: Span<'_>) -> ParserResult<'_, Span<'_>> {
        let (rest, (_, _, v)) = track(
            SpecCode::Returns,
            tuple((blank, tag("Returns: "), take_while1(|c: char| c != '\n'))),
        )(input)
        .with_code(SpecCode::Returns)?;

        Ok((rest, v))
    }

    #[inline]
    fn parse_constraints(input: Span<'_>) -> ParserResult<'_, Text> {
        let (rest, (_, _, v)) = track(
            SpecCode::Constraints,
            tuple((
                blank,
                tag("Constraints:"),
                fparse_until(
                    '\n',
                    alt((
                        tag("Semantics:"),
                        tag("Note:"),
                        tag("See also"),
                        tok_header3_num,
                        tok_header2_num,
                    )),
                ),
            )),
        )(input)
        .with_code(SpecCode::Constraints)?;

        Ok((rest, Text::from(v.fragment())))
    }

    #[inline]
    fn parse_semantics(input: Span<'_>) -> ParserResult<'_, (Option<Text>, Text)> {
        let (rest, (_, u, _, v)) = track(
            SpecCode::Semantics,
            tuple((
                blank,
                track(
                    SpecCode::Sem0,
                    opt(recognize(tuple((
                        fparse_until(
                            '\n',
                            alt((
                                tag("Semantics:"),
                                tag("Note:"),
                                tag("See also"),
                                tok_header3_num,
                                tok_header2_num,
                            )),
                        ),
                        pchar('\n'),
                    )))),
                ),
                tag("Semantics:"),
                track(
                    SpecCode::Sem1,
                    fparse_until(
                        '\n',
                        alt((
                            tag("Note:"),
                            tag("See also"),
                            tok_header3_num,
                            tok_header2_num,
                        )),
                    ),
                ),
            )),
        )(input)
        .with_code(SpecCode::Semantics)?;

        Ok((
            rest,
            (
                u.map(|v| Text::from(v.fragment())),
                Text::from(v.fragment()),
            ),
        ))
    }

    #[inline]
    fn parse_note(input: Span<'_>) -> ParserResult<'_, Text> {
        let (rest, (_, _, v)) = track(
            SpecCode::Note,
            tuple((
                blank,
                tag("Note:"),
                fparse_until(
                    '\n',
                    alt((tag("See also"), tok_header3_num, tok_header2_num)),
                ),
            )),
        )(input)
        .with_code(SpecCode::Note)?;

        Ok((rest, Text::from(v.fragment())))
    }

    #[inline]
    fn parse_see_also(input: Span<'_>) -> ParserResult<'_, SeeAlso> {
        Track.enter(SpecCode::SeeAlso, input);

        let mut see = SeeAlso { fnname: vec![] };

        let rest = input;

        let (rest, _) = track(
            SpecCode::SeeAlso,
            tuple((blank, alt((tag("See also "), tag("See also: "))))),
        )(rest)
        .track()?;

        let mut loop_rest = rest;
        let rest = 'l: loop {
            let (rest2, (fnname, _, _)) = tuple((
                fparse_until(' ', tok_header3_num),
                whitespace,
                tok_header3_num,
            ))(loop_rest)
            .track()?;

            see.fnname.push((*fnname.fragment()).into());

            let rest2 = match tuple((pchar(','), whitespace))(rest2) {
                Ok((rest, _)) => rest,
                Err(_) => rest2,
            };

            let rest2 = match pchar::<_, ParserError>('\r')(rest2) {
                Ok((rest, _)) => break 'l rest,
                Err(_) => rest2,
            };

            let rest2 = match parse_eof(rest2) {
                Ok((rest, _)) => break 'l rest,
                Err(_) => rest2,
            };

            loop_rest = rest2;
        };

        Track.ok(rest, input, see)
    }

    #[inline]
    fn blank(input: Span<'_>) -> ParserResult<'_, Span<'_>> {
        let (rest, v) = track(
            SpecCode::Blank,
            take_while1(|c: char| c == '\n' || c == '\r' || c == ' ' || c == '\t'),
        )(input)
        .with_code(SpecCode::Blank)?;

        let last = v.slice(v.input_len() - 1..);
        if *last.fragment() != "\n" {
            return Err(nom::Err::Error(ParserError::new(SpecCode::Blank, last)));
        }

        Ok((rest, v))
    }

    #[inline]
    pub fn word(input: Span<'_>) -> ParserResult<'_, Span<'_>> {
        let (rest, v) = track(
            SpecCode::Word,
            take_while1(|c: char| c != ' ' && c != '\t' && c != '\r' && c != '\n'),
        )(input)
        .with_code(SpecCode::Word)?;

        Ok((rest, v))
    }

    #[inline]
    fn whitespace(input: Span<'_>) -> ParserResult<'_, Span<'_>> {
        let (rest, v) =
            take_while(|c: char| c == ' ' || c == '\t')(input).with_code(SpecCode::WhiteSpace)?;

        Ok((rest, v))
    }

    #[inline]
    fn whitespace1(input: Span<'_>) -> ParserResult<'_, Span<'_>> {
        let (rest, v) =
            take_while1(|c: char| c == ' ' || c == '\t')(input).with_code(SpecCode::WhiteSpace)?;

        Ok((rest, v))
    }

    #[inline]
    fn carriage_return(input: Span<'_>) -> ParserResult<'_, Span<'_>> {
        let (rest, v) = track(SpecCode::CarriageReturn, pchar('\r'))(input)
            .with_code(SpecCode::CarriageReturn)?;
        Ok((rest, v))
    }

    #[inline]
    fn newline(input: Span<'_>) -> ParserResult<'_, Span<'_>> {
        let (rest, v) =
            track(SpecCode::NewLine, pchar('\n'))(input).with_code(SpecCode::NewLine)?;
        Ok((rest, v))
    }

    #[inline]
    fn parse_eof(input: Span<'_>) -> ParserResult<'_, Span<'_>> {
        Track.enter(SpecCode::Eof, input);
        if input.len() == 0 {
            Track.ok(input, input, input.take(0))
        } else {
            Track.err(ParserError::new(SpecCode::Eof, input))
        }
    }

    #[inline]
    fn fparse_until<PA, I, O, E: ParseError<I>>(
        hook: char,
        mut end: PA,
    ) -> impl FnMut(I) -> IResult<I, I, E>
    where
        PA: Parser<I, O, E>,
        I: Slice<RangeTo<usize>> + Slice<RangeFrom<usize>> + InputIter + InputTake + Clone + Debug,
        <I as InputIter>::Item: AsChar,
    {
        move |i: I| {
            let tmp = i.clone();
            match end.parse(tmp) {
                Ok((_rest, _v)) => {
                    return Ok((i.clone(), i.take(0)));
                }
                Err(_) => {}
            }

            let mut check_next = false;
            for (idx, v) in i.clone().iter_indices() {
                if check_next {
                    let tmp = i.slice(idx..);
                    match end.parse(tmp) {
                        Ok((_rest, _v)) => {
                            return Ok((i.slice(idx - 1..), i.slice(..idx - 1)));
                        }
                        Err(_) => {}
                    }

                    check_next = false;
                } else {
                    let cc = v.as_char();
                    check_next = hook == cc;
                }
            }

            Err(nom::Err::Error(E::from_error_kind(
                i,
                nom::error::ErrorKind::TakeUntil,
            )))
        }
    }
}

mod error {
    use std::error::Error;
    use std::fmt::{Debug, Display, Formatter};

    pub struct DError(pub Box<dyn Error>);

    impl Debug for DError {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }

    impl<T: Error + 'static> From<T> for DError {
        fn from(value: T) -> Self {
            Self(Box::new(value))
        }
    }

    pub struct DErrorString(pub String);

    impl Debug for DErrorString {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }

    impl Display for DErrorString {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            write!(f, "{}", self.0)
        }
    }

    impl Error for DErrorString {}
}
