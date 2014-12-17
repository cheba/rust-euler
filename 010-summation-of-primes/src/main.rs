use std::iter::AdditiveIterator;

/*******************************************************************
 * Prime iterator from 003                                         *
 *******************************************************************/
use std::iter::count;

trait Prime {
    fn is_prime(self) -> bool;
}

impl Prime for uint {
    fn is_prime(self) -> bool {
        for x in range(2u, self / 2 + 1) {
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

fn primes() -> Primes {
    Primes::new()
}

/*******************************************************************/



fn main() {
    let sum = primes().take_while( |&x| x < 2000000 ).sum();

    println!("{}", sum);
}
