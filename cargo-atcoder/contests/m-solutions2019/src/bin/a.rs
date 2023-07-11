use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
    };
    let ans = (n - 2) * 180;
    println!("{}", ans);
}
