use num::rational::Ratio;
use proconio::input;

fn main() {
    input! {
        x: usize,
        a: usize,
        b: usize,
        c: usize,
    };
    let t1 = Ratio::new(x, a) + c;
    let t2 = Ratio::new(x, b);
    let ans = if t1 < t2 {
        "Hare"
    } else if t1 > t2 {
        "Tortoise"
    } else {
        "Tie"
    };
    println!("{}", ans);
}
