use std::{fmt::Debug, ops::RangeFrom};

use nom::{
    character::complete::newline,
    combinator::{all_consuming, map, opt},
    error::{ErrorKind, ParseError},
    multi::separated_list0,
    sequence::terminated,
    AsChar, Compare, IResult, InputIter, InputLength, InputTake, Parser, Slice,
};

/// parser for `usize` that is a  wrapper around [u64](https://docs.rs/nom/latest/nom/character/complete/fn.u64.html).
/// This is only defined on 64-bit systems.
#[cfg(target_pointer_width = "64")]
pub fn nom_usize<T>(s: T) -> IResult<T, usize>
where
    T: InputIter + Slice<RangeFrom<usize>> + InputLength,
    <T as InputIter>::Item: AsChar,
{
    map(nom::character::complete::u64, |x| x as usize)(s)
}

/// parser for `isize` that is a  wrapper around [i64](https://docs.rs/nom/latest/nom/character/complete/fn.i64.html).
/// This is only defined on 64-bit systems.
#[cfg(target_pointer_width = "64")]
pub fn nom_isize<T>(s: T) -> IResult<T, isize>
where
    T: InputIter + Slice<RangeFrom<usize>> + InputLength + InputTake + Clone,
    <T as InputIter>::Item: AsChar,
    T: for<'a> Compare<&'a [u8]>,
{
    map(nom::character::complete::i64, |x| x as isize)(s)
}

macro_rules! uints {
    ($($n:ident, $t:tt)+) => {
        $(
            #[inline(always)]
            pub fn $n<T>(s: T) -> IResult<T, $t>
            where
                T: InputIter + Slice<RangeFrom<usize>> + InputLength,
                <T as InputIter>::Item: AsChar,
            {
                nom::character::complete::$t(s)
            }
        )+
    };
}

uints! { nom_u8,u8 nom_u16,u16 nom_u32,u32 nom_u64,u64 nom_u128,u128}

macro_rules! ints {
    ($($n:ident, $t:tt)+) => {
        $(
            #[inline(always)]
            pub fn $n<T, E: ParseError<T>>(s: T) -> IResult<T, $t, E>
                where
                T: InputIter + Slice<RangeFrom<usize>> + InputLength + InputTake + Clone,
                <T as InputIter>::Item: AsChar,
                T: for <'a> Compare<&'a[u8]>,
            {
                nom::character::complete::$t(s)
            }
        )+
    };
}

ints! { nom_i8,i8 nom_i16,i16 nom_i32,i32 nom_i64,i64 nom_i128,i128}

pub fn process_input<F, I, R, E>(mut f: F) -> impl FnMut(I) -> R
where
    I: Compare<I> + InputIter + Slice<RangeFrom<usize>> + InputLength + InputTake + Clone,
    F: Parser<I, R, E>,
    E: ParseError<I> + Debug,
    <I as InputIter>::Item: AsChar,
{
    move |i: I| {
        all_consuming(optional_trailing_nl(|x| f.parse(x)))
            .parse(i)
            .unwrap()
            .1
    }
}

pub fn optional_trailing_nl<F, I, R, E>(mut f: F) -> impl FnMut(I) -> IResult<I, R, E>
where
    I: Compare<I> + InputIter + InputTake + Clone + Slice<RangeFrom<usize>>,
    F: Parser<I, R, E>,
    E: ParseError<I>,
    <I as InputIter>::Item: AsChar,
{
    move |i: I| terminated(|x| f.parse(x), opt(newline)).parse(i)
}

pub fn nom_lines<F, I, R, E>(mut f: F) -> impl FnMut(I) -> IResult<I, Vec<R>, E>
where
    I: Compare<I> + InputIter + Slice<RangeFrom<usize>> + InputLength + InputTake + Clone,
    F: Parser<I, R, E>,
    E: ParseError<I>,
    <I as InputIter>::Item: AsChar,
{
    move |i: I| separated_list0(newline, |x| f.parse(x)).parse(i)
}

pub fn fold_separated_list0<I, O, O2, E, F, G, H, R, S>(
    mut sep: S,
    mut f: F,
    mut init: H,
    mut combiner: G,
) -> impl FnMut(I) -> IResult<I, R, E>
where
    I: Clone + InputLength,
    F: Parser<I, O, E>,
    S: Parser<I, O2, E>,
    E: ParseError<I>,
    G: FnMut(R, O) -> R,
    H: FnMut() -> R,
{
    move |mut i: I| {
        let mut res = init();

        match f.parse(i.clone()) {
            Err(nom::Err::Error(_)) => return Ok((i, res)),
            Err(e) => return Err(e),
            Ok((i1, o)) => {
                res = combiner(res, o);
                i = i1;
            }
        }

        loop {
            let len = i.input_len();
            match sep.parse(i.clone()) {
                Err(nom::Err::Error(_)) => return Ok((i, res)),
                Err(e) => return Err(e),
                Ok((i1, _)) => {
                    // infinite loop check: the parser must always consume
                    if i1.input_len() == len {
                        return Err(nom::Err::Error(E::from_error_kind(
                            i1,
                            ErrorKind::SeparatedList,
                        )));
                    }

                    match f.parse(i1.clone()) {
                        Err(nom::Err::Error(_)) => return Ok((i, res)),
                        Err(e) => return Err(e),
                        Ok((i2, o)) => {
                            res = combiner(res, o);
                            i = i2;
                        }
                    }
                }
            }
        }
    }
}

pub fn fold_separated_list1<I, O, O2, E, F, G, H, R, S>(
    mut sep: S,
    mut f: F,
    mut init: H,
    mut combiner: G,
) -> impl FnMut(I) -> IResult<I, R, E>
where
    I: Clone + InputLength,
    F: Parser<I, O, E>,
    S: Parser<I, O2, E>,
    E: ParseError<I>,
    G: FnMut(R, O) -> R,
    H: FnMut() -> R,
{
    move |mut i: I| {
        let mut res = init();

        // Parse the first element
        match f.parse(i.clone()) {
            Err(e) => return Err(e),
            Ok((i1, o)) => {
                res = combiner(res, o);
                i = i1;
            }
        }

        loop {
            let len = i.input_len();
            match sep.parse(i.clone()) {
                Err(nom::Err::Error(_)) => return Ok((i, res)),
                Err(e) => return Err(e),
                Ok((i1, _)) => {
                    // infinite loop check: the parser must always consume
                    if i1.input_len() == len {
                        return Err(nom::Err::Error(E::from_error_kind(
                            i1,
                            ErrorKind::SeparatedList,
                        )));
                    }

                    match f.parse(i1.clone()) {
                        Err(nom::Err::Error(_)) => return Ok((i, res)),
                        Err(e) => return Err(e),
                        Ok((i2, o)) => {
                            res = combiner(res, o);
                            i = i2;
                        }
                    }
                }
            }
        }
    }
}
