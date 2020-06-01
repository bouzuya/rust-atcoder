use proconio::input;
use proconio::marker::Usize1;

// TODO
fn main() {
    input! {
        n: usize,
        a: [Usize1; n],
    };
    let ans = n - a.len();
    println!("{}", ans);
}
