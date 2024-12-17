use crate::{prev::Prev, succ::Succ};
use std::{
    cmp::Ordering,
    fmt::{Debug, Display},
    ops::{Add, Mul, Neg, Sub},
};

#[derive(Clone, Copy)]
pub struct Zero;

impl Into<i32> for Zero {
    fn into(self) -> i32 {
        0
    }
}

impl Debug for Zero {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Zero")
    }
}

impl Display for Zero {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "0")
    }
}

impl<N> PartialEq<Succ<N>> for Zero {
    fn eq(&self, _other: &Succ<N>) -> bool {
        false
    }
}

impl<N> PartialEq<Prev<N>> for Zero {
    fn eq(&self, _other: &Prev<N>) -> bool {
        false
    }
}

impl PartialEq for Zero {
    fn eq(&self, _other: &Self) -> bool {
        true
    }
}

impl Eq for Zero {}

impl<N> PartialOrd<Prev<N>> for Zero {
    fn partial_cmp(&self, _other: &Prev<N>) -> Option<Ordering> {
        Some(Ordering::Greater)
    }
}

impl PartialOrd for Zero {
    fn partial_cmp(&self, _other: &Zero) -> Option<Ordering> {
        Some(Ordering::Equal)
    }
}

impl<N> PartialOrd<Succ<N>> for Zero {
    fn partial_cmp(&self, _other: &Succ<N>) -> Option<Ordering> {
        Some(Ordering::Less)
    }
}

impl Ord for Zero {
    fn cmp(&self, other: &Zero) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl<N> Add<N> for Zero {
    type Output = N;

    fn add(self, other: N) -> N {
        other
    }
}

impl Neg for Zero {
    type Output = Zero;

    fn neg(self) -> Zero {
        self
    }
}

impl<N> Sub<N> for Zero
where
    N: Neg,
{
    type Output = <N as Neg>::Output;

    fn sub(self, other: N) -> Self::Output {
        other.neg()
    }
}

impl<N> Mul<N> for Zero {
    type Output = Zero;

    fn mul(self, _other: N) -> Self::Output {
        Zero
    }
}