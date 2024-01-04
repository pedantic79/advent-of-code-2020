use std::{fmt::Debug, str::FromStr};

pub fn parse_pair<T: FromStr>(s: &str) -> Option<(T, T)> {
    parse_split_once(s, ',')
}

pub fn parse_range<T: FromStr>(s: &str) -> Option<(T, T)> {
    parse_split_once(s, "..")
}

pub fn parse_split_n<'a, const N: usize, T, P>(s: &'a str, p: P) -> Option<[T; N]>
where
    T: FromStr,
    P: stable_pattern::Pattern<'a>,
{
    pattern::split(s, p)
        .take(N)
        .map(|x| x.parse::<T>().ok())
        .collect::<Option<arrayvec::ArrayVec<T, N>>>()
        .and_then(|o| o.into_inner().ok())
}

pub fn parse_split_once<'a, T, P>(s: &'a str, p: P) -> Option<(T, T)>
where
    T: FromStr,
    P: stable_pattern::Pattern<'a>,
{
    let (l, r) = pattern::split_once(s, p)?;

    let l = l.parse().ok()?;
    let r = r.parse().ok()?;
    Some((l, r))
}

pub fn parse_lines<T>(s: &str) -> Vec<T>
where
    T: FromStr,
    <T as FromStr>::Err: Debug,
{
    s.lines()
        .map(|line| line.parse())
        .collect::<Result<_, _>>()
        .unwrap()
}

pub fn parse_split<'a, T, P>(s: &'a str, p: P) -> Vec<T>
where
    T: FromStr,
    <T as FromStr>::Err: Debug,
    P: stable_pattern::Pattern<'a>,
{
    pattern::split(s, p)
        .map(|line| line.parse())
        .collect::<Result<_, _>>()
        .unwrap()
}

mod pattern {
    use stable_pattern::{Pattern, Searcher, Split, SplitInternal};

    pub fn split_once<'a, P: Pattern<'a>>(s: &'a str, delimiter: P) -> Option<(&'a str, &'a str)> {
        let (start, end) = delimiter.into_searcher(s).next_match()?;
        // SAFETY: `Searcher` is known to return valid indices.
        unsafe { Some((s.get_unchecked(..start), s.get_unchecked(end..))) }
    }

    pub fn split<'a, P: Pattern<'a>>(s: &'a str, pat: P) -> Split<'a, P> {
        Split(SplitInternal {
            start: 0,
            end: s.len(),
            matcher: pat.into_searcher(s),
            allow_trailing_empty: true,
            finished: false,
        })
    }
}
