use proconio::input;

fn main() {
    input! {
        a: usize,
        b: usize,
        c: usize,
    };
    let ans = (a + b == c) || (a + c == b) || (a == b + c);
    println!("{}", if ans { "Yes" } else { "No" });
}
