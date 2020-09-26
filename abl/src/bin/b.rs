use proconio::input;

fn main() {
    input! {
        a: usize,
        b: usize,
        c: usize,
        d: usize,
    };
    let ans = !(b < c || d < a);
    println!("{}", if ans { "Yes" } else { "No" });
}
