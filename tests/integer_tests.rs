use my_proc_macro::from_int;
use trasteo_macros::{MyInteger, Prev, Succ, Zero};

#[test]
fn test_to_int_zero() {
    assert_eq!(from_int!(0).to_int(), 0);
}

#[test]
fn test_to_int_positive() {
    assert_eq!(from_int!(1).to_int(), 1);
    assert_eq!(from_int!(2).to_int(), 2);
    assert_eq!(from_int!(3).to_int(), 3);
}

#[test]
fn test_to_int_negative() {
    assert_eq!(from_int!(-1).to_int(), -1);
    assert_eq!(from_int!(-2).to_int(), -2);
    assert_eq!(from_int!(-3).to_int(), -3);
}

#[test]
fn test_succ_positive() {
    assert_eq!(from_int!(0).succ().to_int(), 1);
    assert_eq!(from_int!(1).succ().to_int(), 2);
    assert_eq!(from_int!(2).succ().to_int(), 3);
}

#[test]
fn test_succ_negative() {
    assert_eq!(from_int!(-1).succ().to_int(), 0);
    assert_eq!(from_int!(-2).succ().to_int(), -1);
    assert_eq!(from_int!(-3).succ().to_int(), -2);
}

#[test]
fn test_my_add_zero() {
    assert_eq!(from_int!(0).my_add(from_int!(0)).to_int(), 0);
}

#[test]
fn test_my_add_positive_numbers() {
    assert_eq!(from_int!(0).my_add(from_int!(1)).to_int(), 1);
    assert_eq!(from_int!(0).my_add(from_int!(2)).to_int(), 2);
    assert_eq!(from_int!(1).my_add(from_int!(0)).to_int(), 1);
    assert_eq!(from_int!(1).my_add(from_int!(1)).to_int(), 2);
    assert_eq!(from_int!(1).my_add(from_int!(2)).to_int(), 3);
    assert_eq!(from_int!(2).my_add(from_int!(0)).to_int(), 2);
    assert_eq!(from_int!(2).my_add(from_int!(1)).to_int(), 3);
    assert_eq!(from_int!(2).my_add(from_int!(2)).to_int(), 4);
}

#[test]
fn test_my_add_negative_numbers() {
    assert_eq!(from_int!(0).my_add(from_int!(-1)).to_int(), -1);
    assert_eq!(from_int!(0).my_add(from_int!(-2)).to_int(), -2);
    assert_eq!(from_int!(-1).my_add(from_int!(0)).to_int(), -1);
    assert_eq!(from_int!(-1).my_add(from_int!(-1)).to_int(), -2);
    assert_eq!(from_int!(-1).my_add(from_int!(-2)).to_int(), -3);
    assert_eq!(from_int!(-2).my_add(from_int!(0)).to_int(), -2);
    assert_eq!(from_int!(-2).my_add(from_int!(-1)).to_int(), -3);
    assert_eq!(from_int!(-2).my_add(from_int!(-2)).to_int(), -4);
}

#[test]
fn test_mull_zero() {
    assert_eq!(from_int!(0).mull(from_int!(0)).to_int(), 0);
}

#[test]
fn test_mull_positive_numbers() {
    assert_eq!(from_int!(0).mull(from_int!(1)).to_int(), 0);
    assert_eq!(from_int!(0).mull(from_int!(2)).to_int(), 0);
    assert_eq!(from_int!(1).mull(from_int!(0)).to_int(), 0);
    assert_eq!(from_int!(1).mull(from_int!(1)).to_int(), 1);
    assert_eq!(from_int!(1).mull(from_int!(2)).to_int(), 2);
    assert_eq!(from_int!(2).mull(from_int!(0)).to_int(), 0);
    assert_eq!(from_int!(2).mull(from_int!(1)).to_int(), 2);
    assert_eq!(from_int!(2).mull(from_int!(2)).to_int(), 4);
}

#[test]
fn test_mull_negative_numbers() {
    assert_eq!(from_int!(0).mull(from_int!(-1)).to_int(), 0);
    assert_eq!(from_int!(0).mull(from_int!(-2)).to_int(), 0);
    assert_eq!(from_int!(-1).mull(from_int!(0)).to_int(), 0);
    assert_eq!(from_int!(-1).mull(from_int!(-1)).to_int(), 1);
    assert_eq!(from_int!(-1).mull(from_int!(-2)).to_int(), 2);
    assert_eq!(from_int!(-2).mull(from_int!(0)).to_int(), 0);
    assert_eq!(from_int!(-2).mull(from_int!(-1)).to_int(), 2);
    assert_eq!(from_int!(-2).mull(from_int!(-2)).to_int(), 4);
}

#[test]
fn test_prev_positive() {
    assert_eq!(from_int!(0).prev().to_int(), -1);
    assert_eq!(from_int!(1).prev().to_int(), 0);
    assert_eq!(from_int!(2).prev().to_int(), 1);
}

#[test]
fn test_prev_negative() {
    assert_eq!(from_int!(-1).prev().to_int(), -2);
    assert_eq!(from_int!(-2).prev().to_int(), -3);
    assert_eq!(from_int!(-3).prev().to_int(), -4);
}

#[test]
fn test_neg() {
    assert_eq!(from_int!(0).neg().to_int(), 0);
    assert_eq!(from_int!(1).neg().to_int(), -1);
    assert_eq!(from_int!(2).neg().to_int(), -2);
    assert_eq!(from_int!(-1).neg().to_int(), 1);
    assert_eq!(from_int!(-2).neg().to_int(), 2);
    assert_eq!(from_int!(-3).neg().to_int(), 3);
}

#[test]
fn test_sub_zero() {
    assert_eq!(from_int!(0).sub(from_int!(0)).to_int(), 0);
}

#[test]
fn test_sub_positive_numbers() {
    assert_eq!(from_int!(0).sub(from_int!(1)).to_int(), -1);
    assert_eq!(from_int!(0).sub(from_int!(2)).to_int(), -2);
    assert_eq!(from_int!(1).sub(from_int!(0)).to_int(), 1);
    assert_eq!(from_int!(1).sub(from_int!(1)).to_int(), 0);
    assert_eq!(from_int!(1).sub(from_int!(2)).to_int(), -1);
    assert_eq!(from_int!(2).sub(from_int!(0)).to_int(), 2);
    assert_eq!(from_int!(2).sub(from_int!(1)).to_int(), 1);
    assert_eq!(from_int!(2).sub(from_int!(2)).to_int(), 0);
}

#[test]
fn test_sub_negative_numbers() {
    assert_eq!(from_int!(0).sub(from_int!(-1)).to_int(), 1);
    assert_eq!(from_int!(0).sub(from_int!(-2)).to_int(), 2);
    assert_eq!(from_int!(-1).sub(from_int!(0)).to_int(), -1);
    assert_eq!(from_int!(-1).sub(from_int!(-1)).to_int(), 0);
    assert_eq!(from_int!(-1).sub(from_int!(-2)).to_int(), 1);
    assert_eq!(from_int!(-2).sub(from_int!(0)).to_int(), -2);
    assert_eq!(from_int!(-2).sub(from_int!(-1)).to_int(), -1);
    assert_eq!(from_int!(-2).sub(from_int!(-2)).to_int(), 0);
}

#[test]
fn test_chain_operations() {
    assert_eq!(
        from_int!(1).my_add(from_int!(2)).mull(from_int!(3)).to_int(),
        9
    );
    assert_eq!(
        from_int!(1).sub(from_int!(2)).mull(from_int!(3)).to_int(),
        -3
    );
    assert_eq!(
        from_int!(1).mull(from_int!(2).my_add(from_int!(3))).to_int(),
        5
    );
    assert_eq!(
        from_int!(1).mull(from_int!(2).sub(from_int!(3))).to_int(),
        -1
    );
}

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
    assert!(from_int!(1) + from_int!(1) == from_int!(2));
    assert!(from_int!(1) + from_int!(2) == from_int!(3));
    assert!(from_int!(2) + from_int!(1) == from_int!(3));
    assert!(from_int!(2) + from_int!(2) == from_int!(4));
}

#[test]
fn test_add_negative() {
    assert!(from_int!(-1) + from_int!(-1) == from_int!(-2));
    assert!(from_int!(-1) + from_int!(-2) == from_int!(-3));
    assert!(from_int!(-2) + from_int!(-1) == from_int!(-3));
    assert!(from_int!(-2) + from_int!(-2) == from_int!(-4));
}