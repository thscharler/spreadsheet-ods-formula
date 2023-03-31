use csv::StringRecord;
use std::collections::HashMap;
use std::error::Error;
use std::fmt::Write;
use std::fmt::{Debug, Display, Formatter};
use std::fs::File;
use std::io::Write as IOWrite;
use std::path::PathBuf;

#[test]
fn run_build() {
    dbg!(build_from_csv());
}

struct DError(Box<dyn Error>);

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

struct DErrorString(String);

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

struct Module {
    pub mod_file: PathBuf,
    pub gen: String,
}

impl<'a> TryFrom<&'a OdsFn<'a>> for Module {
    type Error = DError;

    fn try_from(fnr: &'a OdsFn<'a>) -> Result<Self, Self::Error> {
        let mut fname = String::from("src/");
        fname.push_str(fnr.module);
        fname.push_str(".rs");

        let mut fmodule = Module {
            mod_file: fname.into(),
            gen: String::new(),
        };

        init_module(&mut fmodule)?;

        Ok(fmodule)
    }
}

struct OdsArg<'a> {
    arg: &'a str,
    typ: &'a str,
}

struct OdsFn<'a> {
    module: &'a str,
    func: &'a str,
    doc: &'a str,
    result: &'a str,
    args: Vec<OdsArg<'a>>,
}

impl<'a> From<&'a StringRecord> for OdsFn<'a> {
    fn from(r: &'a StringRecord) -> Self {
        let mut args = Vec::new();
        for n in 0..5 {
            let arg = &r[4 + 2 * n];
            let typ = &r[4 + 2 * n + 1];

            if !arg.is_empty() && !typ.is_empty() {
                args.push(OdsArg { arg, typ })
            } else if arg.is_empty() && typ.is_empty() {
                // fine
            } else {
                panic!("invalid args for {}", &r[1]);
            }
        }

        Self {
            module: &r[0],
            func: &r[1],
            doc: &r[2],
            result: &r[3],
            args,
        }
    }
}

fn build_from_csv() -> Result<(), DError> {
    let mut csv = csv::Reader::from_path("fn.csv")?;

    let mut mods = HashMap::new();
    for r in csv.records() {
        let r = r?;

        let fnr: OdsFn = (&r).into();

        if !mods.contains_key(fnr.module) {
            mods.insert(fnr.module.to_string(), (&fnr).try_into()?);
        }
        let m = mods.get_mut(fnr.module).expect("module");

        generate_fn(&fnr, m)?;
    }

    for v in mods.values() {
        let mut ff = File::create(&v.mod_file)?;
        ff.write_all(v.gen.as_ref())?;
    }

    Ok(())
}

fn init_module(m: &mut Module) -> Result<(), DError> {
    writeln!(m.gen, "use crate::*;")?;

    Ok(())
}

const TNAME: [&str; 5] = ["A", "B", "C", "D", "E"];

fn generate_fn(fnr: &OdsFn, m: &mut Module) -> Result<(), DError> {
    writeln!(m.gen, "")?;
    writeln!(m.gen, "/// {}", fnr.doc)?;
    writeln!(m.gen, "#[inline]")?;
    writeln!(
        m.gen,
        "pub fn {}{}({}) -> {} {{",
        gen_fn_name(fnr)?,
        gen_type_arg(fnr)?,
        gen_arg(fnr)?,
        gen_return(fnr)?
    )?;
    writeln!(
        m.gen,
        "    {}(\"{}\", {})",
        gen_struct(fnr)?,
        fnr.func,
        gen_param(fnr)?,
    )?;
    writeln!(m.gen, "}}")?;

    Ok(())
}

fn gen_param(fnr: &OdsFn) -> Result<String, DError> {
    let mut buf = String::new();

    for (i, a) in fnr.args.iter().enumerate() {
        if i > 0 {
            write!(buf, ", ")?;
        }
        write!(buf, "{}", a.arg)?;
    }

    Ok(buf)
}

fn gen_return(fnr: &OdsFn) -> Result<String, DError> {
    let mut buf = String::new();

    write!(buf, "{}", gen_struct(fnr)?)?;
    if fnr.args.len() > 0 {
        write!(buf, "<")?;
    }
    for (i, a) in fnr.args.iter().enumerate() {
        if i > 0 {
            write!(buf, ", ")?;
        }
        write!(buf, "{}", TNAME[i])?;
    }
    if fnr.args.len() > 0 {
        write!(buf, ">")?;
    }

    Ok(buf)
}

fn gen_arg(fnr: &OdsFn) -> Result<String, DError> {
    let mut buf = String::new();

    for (i, a) in fnr.args.iter().enumerate() {
        if i > 0 {
            write!(buf, ", ")?;
        }
        write!(buf, "{}: {}", a.arg, TNAME[i])?;
    }

    Ok(buf)
}

fn gen_type_arg(fnr: &OdsFn) -> Result<String, DError> {
    let mut buf = String::new();

    if fnr.args.len() > 0 {
        write!(buf, "<")?;
    }
    for (i, a) in fnr.args.iter().enumerate() {
        if i > 0 {
            write!(buf, ", ")?;
        }
        write!(buf, "{}: {}", TNAME[i], a.typ)?;
    }
    if fnr.args.len() > 0 {
        write!(buf, ">")?;
    }

    Ok(buf)
}

fn gen_struct(fnr: &OdsFn) -> Result<String, DError> {
    match fnr.result {
        "Number" => match fnr.args.len() {
            0 => Ok("FnNumber0".into()),
            1 => Ok("FnNumber1".into()),
            2 => Ok("FnNumber2".into()),
            3 => Ok("FnNumber3".into()),
            4 => Ok("FnNumber4".into()),
            5 => Ok("FnNumber5".into()),
            _ => Err(DErrorString(format!("Number args > 5 for {}", fnr.func)).into()),
        },
        _ => Err(DErrorString(format!("Unknown result for {}", fnr.func)).into()),
    }
}

fn gen_fn_name(fnr: &OdsFn) -> Result<String, DError> {
    let fname = fnr.func.to_lowercase().replace('.', "_");
    Ok(fname)
}
