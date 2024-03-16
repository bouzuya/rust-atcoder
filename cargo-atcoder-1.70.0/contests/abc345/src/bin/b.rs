use num::Integer;
use proconio::input;

fn main() {
    input! {
        x: i64,
    };
    let ans = x.div_ceil(&10);
    println!("{}", ans);
}
