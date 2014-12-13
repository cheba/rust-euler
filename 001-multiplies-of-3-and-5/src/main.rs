
fn range_sum(bound: uint) -> uint {
    range(1u, bound)
        .filter( |&x| x % 3 == 0 || x % 5 == 0 )
        .fold(0, |sum, x| sum + x )
}

#[test]
fn range_sum_for_10() {
    assert_eq!(range_sum(10), 23u);
}

fn main() {
    println!("{}", range_sum(1000));
}
