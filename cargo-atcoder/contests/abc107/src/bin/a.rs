use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        i: Usize1,
    };
    let ans = n - i;
    println!("{}", ans);
}
