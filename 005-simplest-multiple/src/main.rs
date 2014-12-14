use std::iter::count;

fn main() {
    let r = count(20u, 1).find(|&n| {
        range(1u, 21).all( |x| n % x == 0 )
    }).unwrap();

    println!("{}", r);
}
