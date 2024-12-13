pub trait MyInteger: Clone + Copy {
    fn succ(self) -> impl MyInteger;
    fn add<N: MyInteger>(self, other: N) -> impl MyInteger;
    fn mull<N: MyInteger>(self, other: N) -> impl MyInteger;
    fn prev(self) -> impl MyInteger;
    fn neg(self) -> impl MyInteger;
    fn sub<N: MyInteger>(self, other: N) -> impl MyInteger;
    fn to_int(self) -> i32;
}

#[derive(Clone, Copy)]
pub struct Zero;
impl MyInteger for Zero {
    fn succ(self) -> impl MyInteger {
        Succ(Zero)
    }
    fn add<N: MyInteger>(self, other: N) -> impl MyInteger {
        other
    }
    fn mull<N: MyInteger>(self, _: N) -> impl MyInteger {
        Zero
    }
    fn prev(self) -> impl MyInteger {
        Prev(Zero)
    }
    fn neg(self) -> impl MyInteger {
        Zero
    }
    fn sub<N: MyInteger>(self, other: N) -> impl MyInteger {
        other.neg()
    }
    fn to_int(self) -> i32 {
        0
    }
}

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
