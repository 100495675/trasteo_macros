use crate::{succ::Succ, zero::Zero};

use std::{
    cmp::Ordering,
    fmt::{Debug, Display},
    ops::{Add, Mul, Neg, Sub},
};

#[derive(Clone, Copy)]
pub struct Prev<N>(pub N);

impl<N> Into<i32> for Prev<N>
where
    N: Into<i32>,
{
    fn into(self) -> i32 {
        self.0.into() - 1
    }
}

impl<N> Debug for Prev<N>
where
    N: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Prev({:?})", self.0)
    }
}

impl<N> Display for Prev<N>
where
    N: Into<i32> + Clone,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", Into::<i32>::into((*self).clone()))
    }
}

impl<N1, N2> PartialEq<Succ<N2>> for Prev<N1> {
    fn eq(&self, _other: &Succ<N2>) -> bool {
        false
    }
}

impl<N1: PartialEq<N2>, N2> PartialEq<Prev<N2>> for Prev<N1> {
    fn eq(&self, other: &Prev<N2>) -> bool {
        self.0 == other.0
    }
}

impl<N> PartialEq<Zero> for Prev<N> {
    fn eq(&self, _other: &Zero) -> bool {
        false
    }
}

impl<N: Eq> Eq for Prev<N> {}

impl<N1: PartialOrd<N2>, N2> PartialOrd<Prev<N2>> for Prev<N1> {
    fn partial_cmp(&self, other: &Prev<N2>) -> Option<Ordering> {
        Some(self.0.partial_cmp(&other.0).unwrap())
    }
}

impl<N> PartialOrd<Zero> for Prev<N> {
    fn partial_cmp(&self, _other: &Zero) -> Option<Ordering> {
        Some(Ordering::Less)
    }
}

impl<N1, N2> PartialOrd<Succ<N2>> for Prev<N1> {
    fn partial_cmp(&self, _other: &Succ<N2>) -> Option<Ordering> {
        Some(Ordering::Less)
    }
}

impl<N: Ord> Ord for Prev<N> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl<N1, N2> Add<Prev<N2>> for Prev<N1>
where
    Prev<N1>: Add<N2>,
{
    type Output = Prev<<Prev<N1> as Add<N2>>::Output>;
    fn add(self, other: Prev<N2>) -> Self::Output {
        Prev(self + other.0)
    }
}

impl<N1> Add<Zero> for Prev<N1> {
    type Output = Prev<N1>;
    fn add(self, _other: Zero) -> Self::Output {
        self
    }
}

impl<N1, N2> Add<Succ<N2>> for Prev<N1>
where
    N1: Add<N2>,
{
    type Output = <N1 as Add<N2>>::Output;
    fn add(self, other: Succ<N2>) -> Self::Output {
        self.0 + other.0
    }
}

impl<N> Neg for Prev<N>
where
    N: Neg,
{
    type Output = Succ<<N as Neg>::Output>;
    fn neg(self) -> Self::Output {
        Succ(self.0.neg())
    }
}

impl<N1, N2> Sub<Succ<N2>> for Prev<N1>
where
    Prev<N1>: Sub<N2>,
{
    type Output = Prev<<Prev<N1> as Sub<N2>>::Output>;
    fn sub(self, other: Succ<N2>) -> Self::Output {
        Prev(self - other.0)
    }
}

impl<N> Sub<Zero> for Prev<N> {
    type Output = Prev<N>;
    fn sub(self, _other: Zero) -> Self::Output {
        self
    }
}

impl<N1, N2> Sub<Prev<N2>> for Prev<N1>
where
    N1: Sub<N2>,
{
    type Output = <N1 as Sub<N2>>::Output;
    fn sub(self, other: Prev<N2>) -> Self::Output {
        self.0 - other.0
    }
}


impl<N1, N2> Mul<Prev<N2>> for Prev<N1>
where
    Prev<N1>: Mul<N2> + Clone,
    <Prev<N1> as Mul<N2>>::Output: Sub<Prev<N1>>,
{
    type Output = <<Prev<N1> as Mul<N2>>::Output as Sub<Prev<N1>>>::Output;
    fn mul(self, other: Prev<N2>) -> Self::Output {
        self.clone() * other.0 - self
    }
}

impl<N> Mul<Zero> for Prev<N> {
    type Output = Zero;
    fn mul(self, _other: Zero) -> Self::Output {
        Zero
    }
}

impl<N1, N2> Mul<Succ<N2>> for Prev<N1>
where
    Prev<N1>: Mul<N2> + Clone,
    <Prev<N1> as Mul<N2>>::Output: Add<Prev<N1>>,
{
    type Output = <<Prev<N1> as Mul<N2>>::Output as Add<Prev<N1>>>::Output;
    fn mul(self, other: Succ<N2>) -> Self::Output {
        self.clone() * other.0 + self
    }
}
