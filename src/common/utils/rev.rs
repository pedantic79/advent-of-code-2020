use std::{
    hash::{Hash, Hasher},
    ops::Add,
};

pub struct Rev<T>(pub T);

impl<T: num::Zero> num::Zero for Rev<T> {
    fn zero() -> Self {
        Self(T::zero())
    }

    fn is_zero(&self) -> bool {
        self.0.is_zero()
    }
}

impl<T: Add<Output = T>> std::ops::Add for Rev<T> {
    type Output = Rev<T>;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}

impl<T: Ord + PartialOrd> Ord for Rev<T> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.0.cmp(&self.0)
    }
}

impl<T: PartialOrd> PartialOrd for Rev<T> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        other.0.partial_cmp(&self.0)
    }
}

impl<T: PartialEq> PartialEq for Rev<T> {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl<T: Eq> Eq for Rev<T> {}

impl<T: Hash> Hash for Rev<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.hash(state);
    }
}

impl<T: Clone> Clone for Rev<T> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl<T: Copy> Copy for Rev<T> {}

impl<T: std::fmt::Debug> std::fmt::Debug for Rev<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("Rev").field(&self.0).finish()
    }
}

impl<T: std::fmt::Display> std::fmt::Display for Rev<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
