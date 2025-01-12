use super::extensions::AddIsize;
use arrayvec::ArrayVec;
use std::{fmt::Debug, ops::Mul};

pub use super::parse::*;

pub mod rev;

pub trait MyInteger: num::Integer + Clone + for<'a> Mul<&'a Self, Output = Self> {}

impl<T> MyInteger for T where T: num::Integer + Clone + for<'a> Mul<&'a T, Output = T> {}

// Based on the C++ algorithm here: https://stackoverflow.com/a/53604277/7263440
#[inline]
pub fn mod_inv<U>(mut a: U, mut m: U) -> U
where
    U: MyInteger,
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

        let (q, temp) = a.div_rem(&m);
        a = m;
        m = temp;

        let q = q.mul(&x0.0);

        x1 = if x0.1 != x1.1 {
            (x1.0 + q, x1.1)
        } else if x1.0 > q {
            (x1.0 - q, x1.1)
        } else {
            (q - x1.0, !x0.1)
        };

        std::mem::swap(&mut x0, &mut x1);
    }

    if x1.1 {
        m0 - x1.0
    } else {
        x1.0
    }
}

pub fn mod_pow<T>(mut base: T, mut exp: T, modulus: T) -> T
where
    T: MyInteger,
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
    I: MyInteger + num::integer::Roots + num::ToPrimitive + std::hash::Hash,
{
    let m = num::integer::sqrt(modulo.clone());

    let precomp = num::range(I::zero(), m.clone())
        .map(|j| (mod_pow(base.clone(), j.clone(), modulo.clone()), j))
        .collect::<std::collections::HashMap<_, _>>();

    let invgenerator = mod_inv(mod_pow(base, m.clone(), modulo.clone()), modulo.clone());
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
    T: MyInteger,
    I: Iterator<Item = (T, T)> + Clone,
{
    let mut product = T::one();

    for n in inputs.clone() {
        product = product * n.1;
    }

    let mut sum = T::zero();
    for (x, m) in inputs {
        let a = product.clone() / m.clone();
        let y = mod_inv(a.clone(), m.clone());

        sum = sum + x * a * y;
    }

    sum % product
}

pub fn build_array<T, I, const N: usize>(iter: I) -> [T; N]
where
    T: Debug,
    I: IntoIterator<Item = T>,
{
    iter.into_iter()
        .collect::<ArrayVec<T, N>>()
        .into_inner()
        .unwrap()
}

pub fn neighbors_arbitray(
    diffs: &[(isize, isize)],
    r: usize,
    c: usize,
    r_max: usize,
    c_max: usize,
) -> impl Iterator<Item = (usize, usize)> + '_ {
    diffs.iter().filter_map(move |&(y, x)| {
        let r_new = r.checked_add_isize_clamp(y, r_max)?;
        let c_new = c.checked_add_isize_clamp(x, c_max)?;

        Some((r_new, c_new))
    })
}

pub fn neighbors(
    r: usize,
    c: usize,
    r_max: usize,
    c_max: usize,
) -> impl Iterator<Item = (usize, usize)> {
    neighbors_arbitray(&[(-1, 0), (0, -1), (0, 1), (1, 0)], r, c, r_max, c_max)
}

pub fn neighbors_and_self(
    r: usize,
    c: usize,
    r_max: usize,
    c_max: usize,
) -> impl Iterator<Item = (usize, usize)> {
    neighbors_arbitray(
        &[(0, 0), (-1, 0), (0, -1), (0, 1), (1, 0)],
        r,
        c,
        r_max,
        c_max,
    )
}

pub fn neighbors_diag(
    r: usize,
    c: usize,
    r_max: usize,
    c_max: usize,
) -> impl Iterator<Item = (usize, usize)> {
    neighbors_arbitray(
        &[
            (-1, -1),
            (-1, 0),
            (-1, 1),
            (0, -1),
            (0, 1),
            (1, -1),
            (1, 0),
            (1, 1),
        ],
        r,
        c,
        r_max,
        c_max,
    )
}

pub fn neighbors_diag_and_self(
    r: usize,
    c: usize,
    r_max: usize,
    c_max: usize,
) -> impl Iterator<Item = (usize, usize)> {
    neighbors_arbitray(
        &[
            (0, 0),
            (-1, -1),
            (-1, 0),
            (-1, 1),
            (0, -1),
            (0, 1),
            (1, -1),
            (1, 0),
            (1, 1),
        ],
        r,
        c,
        r_max,
        c_max,
    )
}

pub fn slice_get_mut_twice<T>(slice: &mut [T], index0: usize, index1: usize) -> (&mut T, &mut T) {
    assert_ne!(index0, index1);
    assert!(index0 < slice.len());
    assert!(index1 < slice.len());

    // SAFETY: guarantee that the indices are never the same. So it is safe to
    // have two mutable references into the Vec. We'll double check that the
    // indices are within the bounds.
    unsafe {
        let ptr = slice.as_mut_ptr();
        let one = &mut *ptr.add(index0);
        let two = &mut *ptr.add(index1);
        (one, two)
    }
}

#[inline]
pub fn cantor2d_a<T>(x: T, y: T) -> usize
where
    T: num::Signed + num::PrimInt,
{
    // make sure the compiler optimises this conversion for types smaller than usize
    let to_usize = |o: T| (unsafe { o.to_i32().unwrap_unchecked() }) as usize;

    let x1 = to_usize((x.abs() << 1) | T::from(x.is_negative() as u8).unwrap());
    let y1 = to_usize((y.abs() << 1) | T::from(y.is_negative() as u8).unwrap());

    let sum = x1 + y1;
    sum * (sum + 1) / 2 + y1
}

#[inline]
pub fn cantor2d_b<T>(x: T, y: T) -> usize
where
    T: num::Signed + num::PrimInt + num::FromPrimitive,
{
    let to_usize = |o: T| (unsafe { o.to_i32().unwrap_unchecked() }) as usize;
    let neg = T::from_i8(-1).unwrap();

    let xsign = T::from_u8(x.is_negative() as u8).unwrap();
    let ysign = T::from_u8(y.is_negative() as u8).unwrap();

    let x1 = to_usize(x ^ (xsign * neg));
    let y1 = to_usize(y ^ (ysign * neg));
    let sum = x1 + y1;
    let tri = sum * (sum + 1) / 2 + y1;

    (to_usize(xsign) | (to_usize(ysign) << 1)) + tri * 4
}

pub fn calculate_area_perimeter<T>(points: impl Iterator<Item = (T, T)>) -> (T, T)
where
    T: num::PrimInt + num::Signed,
{
    let (a, p, _) = points.fold(
        (T::zero(), T::zero(), (T::zero(), T::zero())),
        |(area, perimeter, prev), curr| {
            (
                area + (prev.0 * curr.1 - prev.1 * curr.0),
                perimeter + (prev.0 - curr.0).abs() + (prev.1 - curr.1).abs(),
                curr,
            )
        },
    );

    (a, p)
}
