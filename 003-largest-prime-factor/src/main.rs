use std::iter::count;

trait Prime {
    fn is_prime(self) -> bool;
}

impl Prime for uint {
    fn is_prime(self) -> bool {
        for x in range(2u, self - 1) {
            if self % x == 0 {
                return false
            }
        }

        true
    }
}

#[test]
fn test_primes() {
    assert_eq!(1.is_prime(), true);
    assert_eq!(2.is_prime(), true);
    assert_eq!(3.is_prime(), true);
    assert_eq!(4.is_prime(), false);
    assert_eq!(23.is_prime(), true);
    assert_eq!(997.is_prime(), true);
    assert_eq!(998.is_prime(), false);
}

struct Primes {
    curr: uint
}

impl Primes {
    fn new() -> Primes {
        Primes { curr: 1 }
    }
}

impl Iterator<uint> for Primes {

    fn next(&mut self) -> Option<uint> {
        let p = count(self.curr + 1, 1)
            .find( |&x| x.is_prime() );

        match p {
            Some(x) => self.curr = x,
            None => {}
        }

        p
    }
}

#[test]
fn test_primes_iter() {
    let mut i = Primes::new();

    assert_eq!(i.next(), Some(2));
    assert_eq!(i.next(), Some(3));
    assert_eq!(i.next(), Some(5));
    assert_eq!(i.next(), Some(7));
    assert_eq!(i.next(), Some(11));
}

fn factors(num: uint) -> Vec<uint> {
    if num.is_prime() {
        vec![num]
    }
    else {
        let f = Primes::new()
            .filter( |&x| x < num )
            .find( |&x| num % x == 0);

        match f {
            Some(x) => vec![x, num / x],
            None => vec![num]
        }
    }
}

#[test]
fn test_factors() {
    assert_eq!(factors(6), vec![2, 3]);
}

fn prime_factors(num: uint) -> Vec<uint> {
    let mut fs = factors(num);

    while fs.iter().any( |&x| !x.is_prime() ) {
        fs = fs.iter().flat_map( |&x| factors(x).into_iter() ).collect();
    }

    fs
}

#[test]
fn test_prime_factors() {
    assert_eq!(prime_factors(30), [2, 3, 5])
}

fn max_prime_factor(num: uint) -> uint {
    *prime_factors(num).iter().max().unwrap()
}

#[test]
fn test_max_prime_factor() {
    assert_eq!(max_prime_factor(30), 5);
}

fn main() {
    println!("{}", max_prime_factor(600851475143));
}
