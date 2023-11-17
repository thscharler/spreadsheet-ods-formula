#![allow(dead_code)]

use crate::mapp::{mod_file, mod_name};
use crate::parse::{Func, Spec};
use std::fs::File;
use std::io;
use std::io::Read;
use std::io::Write;

fn main() -> Result<(), io::Error> {
    let mut txt = Vec::new();

    let mut f = File::open("generate_src/spec.txt")?;
    f.read_to_end(&mut txt)?;

    let txt = String::from_utf8_lossy(txt.as_ref());

    let mut it = txt.as_ref();
    let mut file = None;
    loop {
        let (rest, spec) = parse::parse(&it).expect("mods");
        match spec {
            Spec::Mod(m) => {
                println!("{:?}", m.name);
                file.replace(File::create(format!("generated/{}", mod_file(&m)))?);
                let f = file.as_mut().expect("file");

                for l in m.desc.lines() {
                    writeln!(f, "//! {}", l)?;
                }
                writeln!(f)?;

                writeln!(f, "use crate::*;")?;
                writeln!(f, "#[allow(unused_imports)]")?;
                writeln!(f, "use crate::{}::*;", mod_name(&m))?;
                writeln!(f)?;
            }
            Spec::Func(fun) => {
                let f = file.as_mut().expect("file");
                generate_fn(f, &fun)?;
            }
            Spec::Eof => {
                break;
            }
        }

        it = rest;
    }

    Ok(())
}

pub fn generate_fn(f: &mut File, fun: &Func) -> Result<(), io::Error> {
    writeln!(f)?;

    if fun.summary.len() > 0 {
        writeln!(f, "/// ")?;
        for l in fun.summary.lines() {
            writeln!(f, "/// {}", l)?;
        }
    }

    if let Some(extra0) = &fun.extra0 {
        if extra0.len() > 0 {
            writeln!(f)?;
            writeln!(f, "/// Arguments:")?;
            for l in extra0.lines() {
                writeln!(f, "/// {}", l)?;
            }
        }
    }

    if let Some(constraints) = &fun.constraints {
        if constraints.len() > 0 {
            writeln!(f)?;
            writeln!(f, "/// Constraints:")?;
            for l in constraints.lines() {
                writeln!(f, "/// {}", l)?;
            }
        }
    }

    if let Some(extra1) = &fun.extra1 {
        if extra1.len() > 0 {
            writeln!(f)?;
            writeln!(f, "/// Info2:")?;
            for l in extra1.lines() {
                writeln!(f, "/// {}", l)?;
            }
        }
    }

    if fun.semantics.len() > 0 {
        writeln!(f)?;
        writeln!(f, "/// Semantics:")?;
        for l in fun.semantics.lines() {
            writeln!(f, "/// {}", l)?;
        }
    }

    if let Some(note) = &fun.note {
        if note.len() > 0 {
            writeln!(f)?;
            writeln!(f, "/// Note:")?;
            for l in note.lines() {
                writeln!(f, "/// {}", l)?;
            }
        }
    }

    if !fun.see_also.fnname.is_empty() {
        writeln!(f)?;
        write!(f, "/// See also: ")?;
        for l in &fun.see_also.fnname {
            write!(f, "{:?}, ", l)?;
        }
    }

    Ok(())
}

mod mapp {
    use crate::parse::Mod;

    pub fn mod_file(m: &Mod) -> String {
        format!("{}.rs", mod_name(m))
    }

    pub fn mod_name(m: &Mod) -> &'static str {
        match m.name.as_str() {
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
            _ => panic!("no match for {:?}", m.name),
        }
    }
}

mod parse {
    use kparse::combinators::{pchar, track};
    use kparse::prelude::*;
    use kparse::KParseError;
    use kparse::ParseSpan;
    use kparse::{define_span, Code};
    use nom::branch::alt;
    use nom::bytes::complete::{tag, take_while, take_while1};
    use nom::combinator::{opt, peek, recognize};
    use nom::error::ParseError;
    use nom::sequence::{delimited, tuple};
    use nom::{AsChar, IResult, InputIter, InputLength, InputTake, Parser, Slice};
    use std::fmt::{Debug, Display, Formatter};
    use std::mem;
    use std::ops::{RangeFrom, RangeTo};
    use std::str::Lines;

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
        pub name: String,
        pub args: Vec<Arg>,
        pub etc: bool,
        pub ret: Option<String>,

        pub summary: Text,
        pub extra0: Option<Text>,
        pub constraints: Option<Text>,
        pub extra1: Option<Text>,
        pub semantics: Text,
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
                    Ok((rest, v)) => {
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
            Ok((rest, v)) => Ok((&txt[rest.location_offset()..], v)),
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
                let (rest2, _) = parser_header3(rest)?;
                let (rest2, desc) = parse_general(rest2)?;

                return Ok((rest2, Spec::Mod(Mod::from(mod_name.fragment(), desc))));
            }
            Err(_) => {
                let (rest2, _fnname) = parser_header3(input)?;
                let (rest2, (syntax1, extra0_1)) = parse_syntax(rest2)?;
                let (rest2, summary) = parse_summary(rest2)?;
                let (rest2, (syntax2, extra0_2)) = parse_syntax(rest2)?;
                let (rest2, returns) = opt(parse_returns)(rest2)?;
                let (rest2, constraints) = opt(parse_constraints)(rest2)?;
                let (rest2, (extra1, semantics)) = parse_semantics(rest2)?;
                let (rest2, note) = opt(parse_note)(rest2)?;
                let (rest2, see_also) = opt(parse_see_also)(rest2)?;

                let syntax = syntax1.or(syntax2).expect("syntax");
                let extra0 = extra0_1.or(extra0_2);
                let see_also = see_also.unwrap_or(SeeAlso { fnname: vec![] });

                let fun = Func {
                    name: syntax.0.into(),
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

            args.push(Arg {
                type_mod: type_mod.map(|v| (*v.fragment()).into()),
                type_: (*type_.fragment()).into(),
                ident: (*ident.fragment()).into(),
                default: default.map(|v| (*v.fragment()).into()),
                opt: opt_stack > 0,
                rep: rep_stack > 0,
            });

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
        let rest = match track(SpecCode::Etc, tuple((tag("..."), whitespace)))(input)
            .with_code(SpecCode::Etc)
        {
            Ok((rest, _)) => rest,
            Err(_) => input,
        };

        Ok((rest, rest.take(0)))
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
                    alt((tag("Semantics:"), tag("Note:"), tag("See also"))),
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
                        fparse_until('\n', tag("Semantics:")),
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
            tuple((blank, tag("Note:"), fparse_until('\n', tag("See also")))),
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
