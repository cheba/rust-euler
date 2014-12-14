use std::iter::range_inclusive;
use std::num::Int;
use std::iter::AdditiveIterator;


fn sum_square_difference(n: uint) -> uint {
    // square of sums
    range_inclusive(1, n).sum().pow(2)
    -
    // sum of squares
    range_inclusive(1, n).map(|y| y.pow(2) ).sum()
}

#[test]
fn test_sum_square_difference() {
    assert_eq!(sum_square_difference(10), 2640);
}

fn main() {
    println!("{}", sum_square_difference(100));
}
