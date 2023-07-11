use proconio::{input, marker::Usize1};

fn main() {
    input! {
        s: String,
        t: String,
    };
    let ans = s < t;
    println!("{}", if ans { "Yes" } else { "No" });
}
