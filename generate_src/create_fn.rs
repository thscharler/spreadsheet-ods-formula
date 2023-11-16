use kparse::combinators::{pchar, track};
use kparse::prelude::*;
use kparse::KParseError;
use kparse::ParseSpan;
use kparse::{define_span, Code};
use nom::branch::alt;
use nom::bytes::complete::{tag, take_while, take_while1};
use nom::combinator::{opt, recognize};
use nom::error::ParseError;
use nom::sequence::tuple;
use nom::{AsChar, IResult, InputIter, InputLength, InputTake, Parser, Slice};
use std::fmt::{Debug, Display, Formatter};
use std::fs::File;
use std::io;
use std::io::Read;
use std::ops::{RangeFrom, RangeTo};

fn main() -> Result<(), io::Error> {
    let mut txt = Vec::new();

    let mut f = File::open("generate_src/spec.txt")?;
    f.read_to_end(&mut txt)?;

    parse(&txt);

    Ok(())
}

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

    NewLine,
    Blank,
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

define_span!(pub Span = SpecCode, str);
// pub type Span<'a> = &'a str;
pub type ParserResult<'s, O> = kparse::ParserResult<SpecCode, Span<'s>, O>;
pub type TokenizerResult<'s> = kparse::TokenizerResult<SpecCode, Span<'s>, Span<'s>>;
pub type NomResult<'s> = kparse::ParserResult<SpecCode, Span<'s>, Span<'s>>;
pub type ParserError<'s> = kparse::ParserError<SpecCode, Span<'s>>;

fn parse(txt: &Vec<u8>) {
    let txt = String::from_utf8_lossy(txt.as_ref());

    let tracker = Track::new_tracker::<SpecCode, _>();
    let input = Track::new_span(&tracker, txt.as_ref());

    match parse_spec(input) {
        Ok((_rest, _v)) => {}
        Err(e) => {
            println!("{:#?}", e);
            let r = tracker.results();
            println!("{:#?}", r);
        }
    }
}

#[inline]
fn parse_spec(input: Span<'_>) -> ParserResult<'_, Span<'_>> {
    let mut loop_rest = input;
    'l: loop {
        loop_rest = match parse_header2(loop_rest) {
            Ok((rest3, module)) => {
                let (rest4, _) = parser_header3(rest3)?;
                let (rest4, desc) = parse_general(rest4)?;

                println!("Module {:?} ", module);

                rest4
            }
            Err(_) => {
                let (rest4, fnname) = parser_header3(loop_rest)?;
                let (rest4, syntax1) = opt(parse_syntax)(rest4)?;
                let (rest4, summary) = parse_summary(rest4)?;
                let (rest4, syntax2) = opt(parse_syntax)(rest4)?;
                let (rest4, returns) = opt(parse_returns)(rest4)?;
                let (rest4, constraints) = opt(parse_constraints)(rest4)?;
                let (rest4, semantics) = parse_semantics(rest4)?;
                let (rest4, note) = opt(parse_note)(rest4)?;
                let (rest4, see_also) = opt(parse_see_also)(rest4)?;

                println!("Function {:?} ", fnname);

                rest4
            }
        };

        match parse_eof(loop_rest) {
            Ok((rest, v)) => {
                break 'l;
            }
            Err(_) => {}
        };
    }

    Ok((loop_rest, loop_rest.take(0)))
}

#[inline]
fn parse_header2(input: Span<'_>) -> ParserResult<'_, Span<'_>> {
    let (rest, (_, _, _, _, _, v)) = track(
        SpecCode::Header2,
        tuple((
            parse_newline,
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
            parse_blank,
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
        pchar(' '),
    )))(input)
    .with_code(SpecCode::Header3)?;

    Ok((rest, v))
}

#[inline]
fn parse_general(input: Span<'_>) -> ParserResult<'_, Span<'_>> {
    let (rest, (_, v)) = track(
        SpecCode::General,
        tuple((parse_blank, fparse_until('\n', false, tok_header3_num))),
    )(input)
    .with_code(SpecCode::General)?;

    Ok((rest, v))
}

#[inline]
fn parse_summary(input: Span<'_>) -> ParserResult<'_, Span<'_>> {
    let (rest, (_, _, v)) = track(
        SpecCode::Summary,
        tuple((
            parse_blank,
            tag("Summary:"),
            fparse_until(
                '\n',
                false,
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

    Ok((rest, v))
}

#[inline]
fn parse_syntax(input: Span<'_>) -> ParserResult<'_, (Span<'_>, Span<'_>)> {
    let (rest, (_, _, v, w)) = track(
        SpecCode::Syntax,
        tuple((
            parse_blank,
            tag("Syntax:"),
            take_while1(|c: char| c != '\n'),
            fparse_until(
                '\n',
                true,
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

    Ok((rest, (v, w)))
}

#[inline]
fn parse_returns(input: Span<'_>) -> ParserResult<'_, Span<'_>> {
    let (rest, (_, _, v)) = track(
        SpecCode::Returns,
        tuple((
            parse_blank,
            tag("Returns: "),
            take_while1(|c: char| c != '\n'),
        )),
    )(input)
    .with_code(SpecCode::Returns)?;

    Ok((rest, v))
}

#[inline]
fn parse_constraints(input: Span<'_>) -> ParserResult<'_, Span<'_>> {
    let (rest, (_, _, v)) = track(
        SpecCode::Constraints,
        tuple((
            parse_blank,
            tag("Constraints:"),
            fparse_until(
                '\n',
                true,
                alt((tag("Semantics:"), tag("Note:"), tag("See also"))),
            ),
        )),
    )(input)
    .with_code(SpecCode::Constraints)?;

    Ok((rest, v))
}

#[inline]
fn parse_semantics(input: Span<'_>) -> ParserResult<'_, (Option<Span<'_>>, Span<'_>)> {
    let (rest, (_, u, _, v)) = track(
        SpecCode::Semantics,
        tuple((
            parse_blank,
            track(
                SpecCode::Sem0,
                opt(recognize(tuple((
                    fparse_until('\n', true, tag("Semantics:")),
                    pchar('\n'),
                )))),
            ),
            tag("Semantics:"),
            track(
                SpecCode::Sem1,
                fparse_until(
                    '\n',
                    false,
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

    Ok((rest, (u, v)))
}

#[inline]
fn parse_note(input: Span<'_>) -> ParserResult<'_, Span<'_>> {
    let (rest, (_, _, v)) = track(
        SpecCode::Note,
        tuple((
            parse_blank,
            tag("Note:"),
            fparse_until('\n', true, tag("See also")),
        )),
    )(input)
    .with_code(SpecCode::Note)?;

    Ok((rest, v))
}

#[inline]
fn parse_see_also(input: Span<'_>) -> ParserResult<'_, Span<'_>> {
    let (rest, (_, _, v)) = track(
        SpecCode::SeeAlso,
        tuple((
            parse_blank,
            alt((tag("See also "), tag("See also: "))),
            take_while1(|c: char| c != '\n'),
        )),
    )(input)
    .with_code(SpecCode::SeeAlso)?;

    Ok((rest, v))
}

#[inline]
fn parse_blank(input: Span<'_>) -> ParserResult<'_, Span<'_>> {
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
fn parse_newline(input: Span<'_>) -> ParserResult<'_, Span<'_>> {
    let (rest, v) = track(
        SpecCode::NewLine,
        take_while(|c: char| c == '\n' || c == '\r'),
    )(input)
    .with_code(SpecCode::NewLine)?;

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
pub fn fparse_until<PA, I, O, E: ParseError<I>>(
    c: char,
    check_start: bool,
    mut end: PA,
) -> impl FnMut(I) -> IResult<I, I, E>
where
    PA: Parser<I, O, E>,
    I: Slice<RangeTo<usize>> + Slice<RangeFrom<usize>> + InputIter + InputTake + Clone + Debug,
    <I as InputIter>::Item: AsChar,
{
    move |i: I| {
        if check_start {
            let tmp = i.clone();
            match end.parse(tmp) {
                Ok((rest, _v)) => {
                    return Ok((i.clone(), i.take(0)));
                }
                Err(_) => {}
            }
        }

        let mut check_next = false;
        for (idx, v) in i.clone().iter_indices() {
            if check_next {
                let tmp = i.slice(idx..);
                match end.parse(tmp) {
                    Ok((rest, _v)) => {
                        return Ok((i.slice(idx - 1..), i.slice(..idx - 1)));
                    }
                    Err(_) => {}
                }

                check_next = false;
            } else {
                let cc = v.as_char();
                check_next = c == cc;
            }
        }

        Err(nom::Err::Error(E::from_error_kind(
            i,
            nom::error::ErrorKind::TakeUntil,
        )))
    }
}
