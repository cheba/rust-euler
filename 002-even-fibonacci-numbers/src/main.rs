use std::mem;

struct FibonacchiSequence {
    curr: uint,
    next: uint,
}

impl FibonacchiSequence {
    fn new() -> FibonacchiSequence {
        FibonacchiSequence { curr: 1, next: 1 }
    }
}

impl Iterator<uint> for FibonacchiSequence {
    fn next(&mut self) -> Option<uint> {
        let new_next = self.curr + self.next;
        let new_curr = mem::replace(&mut self.next, new_next);

        Some(mem::replace(&mut self.curr, new_curr))
    }
}

fn even_fib_sum(bound: uint) -> uint {
    FibonacchiSequence::new()
        .take_while( |&x| x <= bound )
        .filter( |&x| x % 2 == 0 )
        .fold(0, |sum, x| sum + x )
}

#[test]
fn even_fib_sum_for_10() {
    assert_eq!(even_fib_sum(55), 44)
}

fn main() {
    println!("{}", even_fib_sum(4000000));
}
