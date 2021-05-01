use proconio::input;

fn main() {
    input! {
        a: usize,
        b: usize,
        c: usize,
    };
    let ans = a + b >= c;
    println!("{}", if ans { "Yes" } else { "No" });
}
