use std::num::Int;


fn is_pythagorean_triplet(a: uint, b: uint, c: uint) -> bool {
    a.pow(2) + b.pow(2) == c.pow(2) && a < b && b < c
}

#[test]
fn test_is_pythagorean_triplet() {
    assert_eq!(is_pythagorean_triplet(3, 4, 5), true);
    assert_eq!(is_pythagorean_triplet(3, 5, 4), false);
}

fn main() {
    for a in range(1u, 999) {
        for b in range(a + 1, 1000) {
            for c in range(b + 1, 1001) {
                if is_pythagorean_triplet(a, b, c) && a + b + c == 1000 {
                    println!("{}", a * b * c);
                    return
                }
            }
        }
    }
}
