use proconio::input;

fn main() {
    input! {
        n: usize,
        a: usize,
        b: usize,
    };
    let ans = if n <= a {
        true
    } else {
        if a > b {
            true
        } else if a < b {
            false
        } else {
            n % (a + 1) != 0
        }
    };
    println!("{}", if ans { "Takahashi" } else { "Aoki" });
}
