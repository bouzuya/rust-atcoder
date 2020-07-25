use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        a: [i64; n],
    };
    for i in k..n {
        let ans = a[i - k] < a[i];
        println!("{}", if ans { "Yes" } else { "No" });
    }
}
