use numberswap_srikanth_kotturi::swap_numbers;

#[test]
fn test_swap_numbers_positive() {
    let mut x = 5;
    let mut y = 10;
    swap_numbers(&mut x, &mut y);
    assert_eq!(x, 10);
    assert_eq!(y, 5);
}

#[test]
fn test_swap_numbers_negative() {
    let mut x = -7;
    let mut y = -3;
    swap_numbers(&mut x, &mut y);
    assert_eq!(x, -3);
    assert_eq!(y, -7);
}

#[test]
fn test_swap_numbers_zero() {
    let mut x = 0;
    let mut y = 0;
    swap_numbers(&mut x, &mut y);
    assert_eq!(x, 0);
    assert_eq!(y, 0);
}

#[test]
fn test_swap_numbers_same_value() {
    let mut x = 42;
    let mut y = 42;
    swap_numbers(&mut x, &mut y);
    assert_eq!(x, 42);
    assert_eq!(y, 42);
}

#[test]
fn test_swap_numbers_large_values() {
    let mut x = 1_000_000_000;
    let mut y = -2_000_000_000;
    swap_numbers(&mut x, &mut y);
    assert_eq!(x, -2_000_000_000);
    assert_eq!(y, 1_000_000_000);
}