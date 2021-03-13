use proconio::input;

fn main() {
    input! {
        m: usize,
        h: usize,
    };
    let ans = h % m == 0;
    println!("{}", if ans { "Yes" } else { "No" });
}
