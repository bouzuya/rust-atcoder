use proconio::input;

fn main() {
    input! {
        a: usize,
        b: usize,
        c: usize,
        d: usize,
    }
    let ans = !(d < a || b < c);
    println!("{}", if ans { "Yes" } else { "No" });
}
