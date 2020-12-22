#![warn(rust_2018_idioms)]
#[macro_use]
extern crate aoc_runner_derive;

pub mod matrix;

pub mod day01;
pub mod day02;
pub mod day03;
pub mod day04;
pub mod day05;
pub mod day06;
pub mod day07;
pub mod day08;
pub mod day09;
pub mod day10;
pub mod day11;
pub mod day12;
pub mod day13;
pub mod day14;
pub mod day15;
pub mod day16;
pub mod day17;
pub mod day18;
pub mod day19;
pub mod day20;
pub mod day21;

trait MinMaxIterator: Iterator {
    fn min_max<'a, T>(mut self) -> Option<(&'a T, &'a T)>
    where
        T: Ord,
        Self: Iterator<Item = &'a T> + Sized,
    {
        self.next()
            .map(|x| self.fold((x, x), |(min, max), num| (min.min(num), max.max(num))))
    }
}

impl<T: ?Sized> MinMaxIterator for T where T: Iterator {}

pub fn mod_inv<T: num::Integer + Clone>(a: T, m: T) -> T {
    a.extended_gcd(&m).x
}

pub fn mod_inv_unsigned<U: num::Num + PartialOrd + Copy + num::One + num::ToPrimitive>(
    a: U,
    m: U,
) -> U {
    let a = a % m;

    for x in num::iter::range(U::one(), m) {
        if (a * x) % m == U::one() {
            return x;
        }
    }

    U::one()
}

aoc_lib! { year = 2020 }
