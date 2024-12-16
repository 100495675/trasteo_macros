use my_proc_macro::MyInteger;

pub trait MyInteger: Clone + Copy {
    fn succ(self) -> impl MyInteger;
    fn add<N: MyInteger>(self, other: N) -> impl MyInteger;
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
    fn add<M: MyInteger>(self, other: M) -> impl MyInteger {
        self.0.add(other).succ()
    }
    fn mull<M: MyInteger>(self, other: M) -> impl MyInteger {
        self.0.mull(other).add(other)
    }
    fn prev(self) -> impl MyInteger {
        self.0
    }
    fn neg(self) -> impl MyInteger {
        self.0.neg().prev()
    }
    fn sub<M: MyInteger>(self, other: M) -> impl MyInteger {
        self.add(other.neg())
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

#[derive(Clone, Copy)]
pub struct Prev<N: MyInteger>(pub N);
impl<N: MyInteger> MyInteger for Prev<N> {
    fn succ(self) -> impl MyInteger {
        self.0
    }
    fn add<M: MyInteger>(self, other: M) -> impl MyInteger {
        self.0.add(other).prev()
    }
    fn mull<M: MyInteger>(self, other: M) -> impl MyInteger {
        self.0.mull(other).add(other.neg())
    }
    fn prev(self) -> impl MyInteger {
        Prev(self)
    }
    fn neg(self) -> impl MyInteger {
        self.0.neg().succ()
    }
    fn sub<M: MyInteger>(self, other: M) -> impl MyInteger {
        self.add(other.neg())
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
