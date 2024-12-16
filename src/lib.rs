use my_proc_macro::MyInteger;
use std::{cmp::Ordering, ops::Add};

pub trait MyInteger: Clone + Copy {
    fn succ(self) -> impl MyInteger;
    fn my_add<N: MyInteger>(self, other: N) -> impl MyInteger;
    fn mull<N: MyInteger>(self, other: N) -> impl MyInteger;
    fn prev(self) -> impl MyInteger;
    fn neg(self) -> impl MyInteger;
    fn sub<N: MyInteger>(self, other: N) -> impl MyInteger;
    fn to_int(self) -> i32;
}

#[derive(Clone, Copy, MyInteger)]
pub struct Zero;

#[derive(Clone, Copy)]
pub struct Succ<N: MyInteger>(pub N);
impl<N: MyInteger> MyInteger for Succ<N> {
    fn succ(self) -> impl MyInteger {
        Succ(self)
    }
    fn my_add<M: MyInteger>(self, other: M) -> impl MyInteger {
        self.0.my_add(other).succ()
    }
    fn mull<M: MyInteger>(self, other: M) -> impl MyInteger {
        self.0.mull(other).my_add(other)
    }
    fn prev(self) -> impl MyInteger {
        self.0
    }
    fn neg(self) -> impl MyInteger {
        self.0.neg().prev()
    }
    fn sub<M: MyInteger>(self, other: M) -> impl MyInteger {
        self.my_add(other.neg())
    }
    fn to_int(self) -> i32 {
        1 + self.0.to_int()
    }
}

impl<N1: MyInteger + PartialEq<N2>, N2: MyInteger> PartialEq<Succ<N2>> for Succ<N1> {
    fn eq(&self, other: &Succ<N2>) -> bool {
        self.0 == other.0
    }
}

impl<N1: MyInteger, N2: MyInteger> PartialEq<Prev<N2>> for Succ<N1> {
    fn eq(&self, _other: &Prev<N2>) -> bool {
        false
    }
}

impl<N: MyInteger> PartialEq<Zero> for Succ<N> {
    fn eq(&self, _other: &Zero) -> bool {
        false
    }
}

impl<N: MyInteger + Eq> Eq for Succ<N> {}

impl<N: MyInteger> PartialOrd<Prev<N>> for Succ<N> {
    fn partial_cmp(&self, _other: &Prev<N>) -> Option<Ordering> {
        Some(Ordering::Greater)
    }
}

impl<N: MyInteger> PartialOrd<Zero> for Succ<N> {
    fn partial_cmp(&self, _other: &Zero) -> Option<Ordering> {
        Some(Ordering::Greater)
    }
}

impl<N1: MyInteger + PartialOrd<N2>, N2: MyInteger> PartialOrd<Succ<N2>> for Succ<N1> {
    fn partial_cmp(&self, other: &Succ<N2>) -> Option<Ordering> {
        Some(self.0.partial_cmp(&other.0).unwrap())
    }
}

impl<N: MyInteger + Ord> Ord for Succ<N> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl<N1, N2> Add<N2> for Succ<N1>
where
    N1: MyInteger + Add<N2>,
    N2: MyInteger,
    <N1 as Add<N2>>::Output: MyInteger,
{
    type Output = Succ<<N1 as Add<N2>>::Output>;
    fn add(self, other: N2) -> Self::Output {
        Succ(self.0 + other)
    }
}

#[derive(Clone, Copy)]
pub struct Prev<N: MyInteger>(pub N);
impl<N: MyInteger> MyInteger for Prev<N> {
    fn succ(self) -> impl MyInteger {
        self.0
    }
    fn my_add<M: MyInteger>(self, other: M) -> impl MyInteger {
        self.0.my_add(other).prev()
    }
    fn mull<M: MyInteger>(self, other: M) -> impl MyInteger {
        self.0.mull(other).my_add(other.neg())
    }
    fn prev(self) -> impl MyInteger {
        Prev(self)
    }
    fn neg(self) -> impl MyInteger {
        self.0.neg().succ()
    }
    fn sub<M: MyInteger>(self, other: M) -> impl MyInteger {
        self.my_add(other.neg())
    }
    fn to_int(self) -> i32 {
        self.0.to_int() - 1
    }
}

impl<N1: MyInteger, N2: MyInteger> PartialEq<Succ<N2>> for Prev<N1> {
    fn eq(&self, _other: &Succ<N2>) -> bool {
        false
    }
}

impl<N1: MyInteger + PartialEq<N2>, N2: MyInteger> PartialEq<Prev<N2>> for Prev<N1> {
    fn eq(&self, other: &Prev<N2>) -> bool {
        self.0 == other.0
    }
}

impl<N: MyInteger> PartialEq<Zero> for Prev<N> {
    fn eq(&self, _other: &Zero) -> bool {
        false
    }
}

impl<N: MyInteger + Eq> Eq for Prev<N> {}

impl<N1: MyInteger + PartialOrd<N2>, N2: MyInteger> PartialOrd<Prev<N2>> for Prev<N1> {
    fn partial_cmp(&self, other: &Prev<N2>) -> Option<Ordering> {
        Some(self.0.partial_cmp(&other.0).unwrap())
    }
}

impl<N: MyInteger> PartialOrd<Zero> for Prev<N> {
    fn partial_cmp(&self, _other: &Zero) -> Option<Ordering> {
        Some(Ordering::Less)
    }
}

impl<N1: MyInteger, N2: MyInteger> PartialOrd<Succ<N2>> for Prev<N1> {
    fn partial_cmp(&self, _other: &Succ<N2>) -> Option<Ordering> {
        Some(Ordering::Less)
    }
}

impl<N: MyInteger + Ord> Ord for Prev<N> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl<N1, N2> Add<N2> for Prev<N1>
where
    N1: MyInteger + Add<N2>,
    N2: MyInteger,
    <N1 as Add<N2>>::Output: MyInteger,
{
    type Output = Prev<<N1 as Add<N2>>::Output>;
    fn add(self, other: N2) -> Self::Output {
        Prev(self.0 + other)
    }
}
