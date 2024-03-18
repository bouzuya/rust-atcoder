use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        k: Usize1,
        a: Usize1,
    };
    let ans = (a + k) % n + 1;
    println!("{}", ans);
}
