#![warn(rust_2018_idioms)]
// The following are enabled by clippy::pedantic
// #![warn(clippy::pedantic)]
#![allow(clippy::must_use_candidate)]
#![allow(clippy::implicit_hasher)]
#![allow(clippy::explicit_iter_loop)]
#![allow(clippy::redundant_closure_for_method_calls)]
#![allow(clippy::if_not_else)]
#![allow(clippy::option_if_let_else)]
#![allow(clippy::match_on_vec_items)]
#![allow(clippy::needless_pass_by_value)]
#![warn(clippy::trivially_copy_pass_by_ref)]
#![warn(clippy::inline_always)]
#![warn(clippy::cast_possible_truncation)]
#![warn(clippy::cast_possible_wrap)]
#![warn(clippy::cast_sign_loss)]
#![warn(clippy::unreadable_literal)]
#![warn(clippy::unseparated_literal_suffix)]
#![warn(clippy::filter_map)]
#![warn(clippy::find_map)]
#![warn(clippy::map_unwrap_or)]
#![warn(clippy::similar_names)]
#![warn(clippy::unused_self)]

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
pub mod day22;
pub mod day23;
pub mod day24;
pub mod day25;

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

#[inline]
pub fn mod_inv<U>(mut a: U, mut m: U) -> U
where
    U: num::Integer + Clone,
{
    if m <= U::one() {
        return U::zero();
    }

    let m0 = m.clone();
    let mut x0 = (U::zero(), false);
    let mut x1 = (U::one(), false);

    while a > U::one() {
        if m == U::zero() {
            return U::zero();
        }

        let q = a.clone() / m.clone();
        let t = m.clone();
        m = a % m;
        a = t;

        let t2 = x0.clone();
        let qx0 = q * x0.0;

        if x0.1 != x1.1 {
            x0 = (x1.0 + qx0, x1.1);
        } else {
            x0 = if x1.0 > qx0 {
                (x1.0 - qx0, x1.1)
            } else {
                (qx0 - x1.0, !x0.1)
            };
        }

        x1 = t2;
    }

    if x1.1 {
        m0 - x1.0
    } else {
        x1.0
    }
}

pub fn mod_pow<T>(mut base: T, mut exp: T, modulus: T) -> T
where
    T: num::Integer + Clone,
{
    if modulus == T::one() {
        return T::zero();
    }

    let mut result = T::one();
    base = base % modulus.clone();
    while exp > T::zero() {
        if exp.is_odd() {
            result = result * base.clone() % modulus.clone();
        }

        exp = exp / (T::one() + T::one());
        base = base.clone() * base % modulus.clone()
    }
    result
}

pub fn baby_step_giant_step<I>(modulo: I, base: I, target: I) -> Option<I>
where
    I: num::Integer + Clone + num::integer::Roots + num::ToPrimitive + std::hash::Hash,
{
    let m = num::integer::sqrt(modulo.clone());

    let precomp = num::range(I::zero(), m.clone())
        .map(|j| (crate::mod_pow(base.clone(), j.clone(), modulo.clone()), j))
        .collect::<std::collections::HashMap<_, _>>();

    let invgenerator = crate::mod_inv(
        crate::mod_pow(base, m.clone(), modulo.clone()),
        modulo.clone(),
    );
    let mut value = target;

    for i in num::range(I::zero(), m.clone()) {
        if let Some(v) = precomp.get(&value) {
            return Some(i * m + v.clone());
        }

        value = value * invgenerator.clone() % modulo.clone();
    }

    None
}

pub fn chinese_remainder_theorem<T, I>(inputs: I) -> T
where
    T: num::Integer + Clone,
    I: Iterator<Item = (T, T)> + Clone,
{
    let mut product = T::one();

    for n in inputs.clone() {
        product = product * n.1;
    }

    let mut sum = T::zero();
    for (x, m) in inputs {
        let a = product.clone() / m.clone();
        let y = crate::mod_inv(a.clone(), m.clone());

        sum = sum + x * a * y;
    }

    sum % product
}

aoc_lib! { year = 2020 }
