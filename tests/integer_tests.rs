use my_proc_macro::from_int;
use trasteo_macros::{prev::Prev, succ::Succ, zero::Zero};

#[test]
fn test_eq_zero() {
    assert!(from_int!(0) != from_int!(-1));
    assert!(from_int!(0) == from_int!(0));
    assert!(from_int!(0) != from_int!(1));
}

#[test]
fn test_eq_positive() {
    assert!(from_int!(1) != from_int!(-1));
    assert!(from_int!(1) != from_int!(0));
    assert!(from_int!(1) == from_int!(1));
    assert!(from_int!(1) != from_int!(2));
}

#[test]
fn test_eq_negative() {
    assert!(from_int!(-1) != from_int!(-2));
    assert!(from_int!(-1) == from_int!(-1));
    assert!(from_int!(-1) != from_int!(0));
    assert!(from_int!(-1) != from_int!(1));
}

#[test]
fn test_cmp_zero() {
    assert!(from_int!(0) == from_int!(0));

    assert!(from_int!(0) > from_int!(-1));
    assert!(!(from_int!(0) > from_int!(0)));
    assert!(!(from_int!(0) < from_int!(0)));
    assert!(from_int!(0) < from_int!(1));

    assert!(from_int!(0) >= from_int!(-2));
    assert!(from_int!(0) >= from_int!(-1));
    assert!(from_int!(0) >= from_int!(0));
    assert!(from_int!(0) <= from_int!(0));
    assert!(from_int!(0) <= from_int!(1));
    assert!(from_int!(0) <= from_int!(2));
}

#[test]
fn test_cmp_positive() {
    assert!(from_int!(1) == from_int!(1));

    assert!(from_int!(1) > from_int!(-1));
    assert!(from_int!(1) > from_int!(0));
    assert!(!(from_int!(1) > from_int!(1)));
    assert!(!(from_int!(1) < from_int!(1)));
    assert!(from_int!(1) < from_int!(2));

    assert!(from_int!(1) >= from_int!(-1));
    assert!(from_int!(1) >= from_int!(0));
    assert!(from_int!(1) >= from_int!(1));
    assert!(from_int!(1) <= from_int!(1));
    assert!(from_int!(1) <= from_int!(2));
}

#[test]
fn test_cmp_negative() {
    assert!(from_int!(-1) == from_int!(-1));

    assert!(from_int!(-1) > from_int!(-2));
    assert!(!(from_int!(-1) > from_int!(-1)));
    assert!(!(from_int!(-1) < from_int!(-1)));
    assert!(from_int!(-1) < from_int!(0));
    assert!(from_int!(-1) < from_int!(1));

    assert!(from_int!(-1) >= from_int!(-2));
    assert!(from_int!(-1) >= from_int!(-1));
    assert!(from_int!(-1) <= from_int!(-1));
    assert!(from_int!(-1) <= from_int!(0));
    assert!(from_int!(-1) <= from_int!(1));
}

#[test]
fn test_add_zero() {
    assert!(from_int!(0) + from_int!(0) == from_int!(0));

    assert!(from_int!(0) + from_int!(1) == from_int!(1));
    assert!(from_int!(0) + from_int!(-1) == from_int!(-1));
}

#[test]
fn test_add_positive() {
    assert!(from_int!(1) + from_int!(-1) == from_int!(0));
    assert!(from_int!(1) + from_int!(0) == from_int!(1));
    assert!(from_int!(1) + from_int!(1) == from_int!(2));
    assert!(from_int!(2) + from_int!(-1) == from_int!(1));
    assert!(from_int!(2) + from_int!(0) == from_int!(2));
    assert!(from_int!(2) + from_int!(1) == from_int!(3));
}

#[test]
fn test_add_negative() {
    assert!(from_int!(-2) + from_int!(-1) == from_int!(-3));
    assert!(from_int!(-2) + from_int!(0) == from_int!(-2));
    assert!(from_int!(-2) + from_int!(1) == from_int!(-1));
    assert!(from_int!(-1) + from_int!(-1) == from_int!(-2));
    assert!(from_int!(-1) + from_int!(0) == from_int!(-1));
    assert!(from_int!(-1) + from_int!(1) == from_int!(0));
}

#[test]
fn test_neg_zero() {
    assert!(-from_int!(0) == from_int!(0));
}

#[test]
fn test_neg_positive() {
    assert!(-from_int!(1) == from_int!(-1));
    assert!(-from_int!(2) == from_int!(-2));
}

#[test]
fn test_neg_negative() {
    assert!(-from_int!(-1) == from_int!(1));
    assert!(-from_int!(-2) == from_int!(2));
}

#[test]
fn test_into() {
    let zero: i32 = from_int!(0).into();
    assert_eq!(zero, 0);

    let one: i32 = from_int!(1).into();
    assert_eq!(one, 1);

    let minus_one: i32 = from_int!(-1).into();
    assert_eq!(minus_one, -1);
}

#[test]
fn test_debug() {
    assert_eq!(format!("{:?}", from_int!(0)), "Zero");
    assert_eq!(format!("{:?}", from_int!(1)), "Succ(Zero)");
    assert_eq!(format!("{:?}", from_int!(2)), "Succ(Succ(Zero))");
    assert_eq!(format!("{:?}", from_int!(-1)), "Prev(Zero)");
    assert_eq!(format!("{:?}", from_int!(-2)), "Prev(Prev(Zero))");
}

#[test]
fn test_display() {
    assert_eq!(format!("{}", from_int!(0)), "0");
    assert_eq!(format!("{}", from_int!(1)), "1");
    assert_eq!(format!("{}", from_int!(-1)), "-1");
}

#[test]
fn test_sub_zero() {
    assert!(from_int!(0) - from_int!(0) == from_int!(0));

    assert!(from_int!(0) - from_int!(1) == from_int!(-1));
    assert!(from_int!(0) - from_int!(-1) == from_int!(1));
}

#[test]
fn test_sub_positive() {
    assert!(from_int!(1) - from_int!(-1) == from_int!(2));
    assert!(from_int!(1) - from_int!(0) == from_int!(1));
    assert!(from_int!(1) - from_int!(1) == from_int!(0));
    assert!(from_int!(2) - from_int!(-1) == from_int!(3));
    assert!(from_int!(2) - from_int!(0) == from_int!(2));
    assert!(from_int!(2) - from_int!(1) == from_int!(1));
}

#[test]
fn test_sub_negative() {
    assert!(from_int!(-2) - from_int!(-1) == from_int!(-1));
    assert!(from_int!(-2) - from_int!(0) == from_int!(-2));
    assert!(from_int!(-2) - from_int!(1) == from_int!(-3));
    assert!(from_int!(-1) - from_int!(-1) == from_int!(0));
    assert!(from_int!(-1) - from_int!(0) == from_int!(-1));
    assert!(from_int!(-1) - from_int!(1) == from_int!(-2));
}

#[test]
fn test_mul_zero() {
    assert!(from_int!(0) * from_int!(0) == from_int!(0));

    assert!(from_int!(0) * from_int!(1) == from_int!(0));
    assert!(from_int!(0) * from_int!(-1) == from_int!(0));
}

#[test]
fn test_mul_positive() {
    assert!(from_int!(1) * from_int!(-1) == from_int!(-1));
    assert!(from_int!(1) * from_int!(0) == from_int!(0));
    assert!(from_int!(1) * from_int!(1) == from_int!(1));
    assert!(from_int!(2) * from_int!(-1) == from_int!(-2));
    assert!(from_int!(2) * from_int!(0) == from_int!(0));
    assert!(from_int!(2) * from_int!(1) == from_int!(2));
}

#[test]
fn test_mul_negative() {
    assert!(from_int!(-2) * from_int!(-1) == from_int!(2));
    assert!(from_int!(-2) * from_int!(0) == from_int!(0));
    assert!(from_int!(-2) * from_int!(1) == from_int!(-2));
    assert!(from_int!(-1) * from_int!(-1) == from_int!(1));
    assert!(from_int!(-1) * from_int!(0) == from_int!(0));
    assert!(from_int!(-1) * from_int!(1) == from_int!(-1));
}