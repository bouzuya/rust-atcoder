use proconio::input;

fn main() {
    input! {
        a: usize,
        b: usize,
    };
    let (a, b) = (a.min(b), a.max(b));
    let ans = (a + 1 == b) && (a % 3 != 0);
    println!("{}", if ans { "Yes" } else { "No" });
}
