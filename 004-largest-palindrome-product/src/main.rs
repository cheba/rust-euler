
trait Palindrome {
    fn is_palindrome(&self) -> bool;
}

impl Palindrome for uint {
    fn is_palindrome(&self) -> bool {
        let s1:String = self.to_string();
        let s2:String = self.to_string().chars().rev().collect();

        s1 == s2
    }
}

#[test]
fn test_palindrome() {
    assert_eq!(123.is_palindrome(), false);
    assert_eq!(9009.is_palindrome(), true);
}


fn main() {
    let mut p: Vec<uint> = vec![];

    for x in range(100u, 1000) {
        for y in range(100u, 1000) {
            if (x * y).is_palindrome() {
                p.push(x * y);
            }
        }
    }

    let max_palyndrome = p.iter().max().unwrap();

    println!("{}", max_palyndrome);
}
