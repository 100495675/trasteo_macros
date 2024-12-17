use crate::{prev::Prev, zero::Zero};

use std::{
    cmp::Ordering,
    fmt::{Debug, Display},
    ops::{Add, Mul, Neg, Sub},
};

#[derive(Clone, Copy)]
pub struct Succ<N>(pub N);

impl<N> Into<i32> for Succ<N>
where
    N: Into<i32>,
{
    fn into(self) -> i32 {
        1 + self.0.into()
    }
}

impl<N> Debug for Succ<N>
where
    N: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Succ({:?})", self.0)
    }
}

impl<N> Display for Succ<N>
where
    N: Into<i32> + Clone,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", Into::<i32>::into((*self).clone()))
    }
}

impl<N1: PartialEq<N2>, N2> PartialEq<Succ<N2>> for Succ<N1> {
    fn eq(&self, other: &Succ<N2>) -> bool {
        self.0 == other.0
    }
}

impl<N1, N2> PartialEq<Prev<N2>> for Succ<N1> {
    fn eq(&self, _other: &Prev<N2>) -> bool {
        false
    }
}

impl<N> PartialEq<Zero> for Succ<N> {
    fn eq(&self, _other: &Zero) -> bool {
        false
    }
}

impl<N: Eq> Eq for Succ<N> {}

impl<N> PartialOrd<Prev<N>> for Succ<N> {
    fn partial_cmp(&self, _other: &Prev<N>) -> Option<Ordering> {
        Some(Ordering::Greater)
    }
}

impl<N> PartialOrd<Zero> for Succ<N> {
    fn partial_cmp(&self, _other: &Zero) -> Option<Ordering> {
        Some(Ordering::Greater)
    }
}

impl<N1: PartialOrd<N2>, N2> PartialOrd<Succ<N2>> for Succ<N1> {
    fn partial_cmp(&self, other: &Succ<N2>) -> Option<Ordering> {
        Some(self.0.partial_cmp(&other.0).unwrap())
    }
}

impl<N: Ord> Ord for Succ<N> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl<N1, N2> Add<Prev<N2>> for Succ<N1>
where
    N1: Add<N2>,
{
    type Output = <N1 as Add<N2>>::Output;
    fn add(self, other: Prev<N2>) -> Self::Output {
        self.0 + other.0
    }
}

impl<N1> Add<Zero> for Succ<N1> {
    type Output = Succ<N1>;
    fn add(self, _other: Zero) -> Self::Output {
        self
    }
}

impl<N1, N2> Add<Succ<N2>> for Succ<N1>
where
    Succ<N1>: Add<N2>,
{
    type Output = Succ<<Succ<N1> as Add<N2>>::Output>;
    fn add(self, other: Succ<N2>) -> Self::Output {
        Succ(self + other.0)
    }
}

impl<N> Neg for Succ<N>
where
    N: Neg,
{
    type Output = Prev<<N as Neg>::Output>;
    fn neg(self) -> Self::Output {
        Prev(self.0.neg())
    }
}

impl<N1, N2> Sub<N2> for Succ<N1>
where
    Succ<N1>: Add<<N2 as Neg>::Output>,
    N2: Neg,
{
    type Output = <Succ<N1> as Add<<N2 as Neg>::Output>>::Output;
    fn sub(self, other: N2) -> Self::Output {
        self + (-other)
    }
}

impl<N1, N2> Mul<Prev<N2>> for Succ<N1>
where
    Succ<N1>: Mul<N2> + Clone,
    <Succ<N1> as Mul<N2>>::Output: Sub<Succ<N1>>,
{
    type Output = <<Succ<N1> as Mul<N2>>::Output as Sub<Succ<N1>>>::Output;
    fn mul(self, other: Prev<N2>) -> Self::Output {
        self.clone() * other.0 - self
    }
}

impl<N> Mul<Zero> for Succ<N> {
    type Output = Zero;
    fn mul(self, _other: Zero) -> Self::Output {
        Zero
    }
}

impl<N1, N2> Mul<Succ<N2>> for Succ<N1>
where
    Succ<N1>: Mul<N2> + Clone,
    <Succ<N1> as Mul<N2>>::Output: Add<Succ<N1>>,
{
    type Output = <<Succ<N1> as Mul<N2>>::Output as Add<Succ<N1>>>::Output;
    fn mul(self, other: Succ<N2>) -> Self::Output {
        self.clone() * other.0 + self
    }
}
