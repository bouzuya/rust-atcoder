use proconio::input;

fn main() {
    input! {
        a: usize,
        b: usize,
    };
    let ans = (2 * a..=2 * a + 1).contains(&b);
    println!("{}", if ans { "Yes" } else { "No" });
}
