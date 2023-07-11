use proconio::input;

fn main() {
    input! {
        a: usize,
        b: usize,
        x: usize,
    };
    let ans = (a..=a + b).contains(&x);
    println!("{}", if ans { "YES" } else { "NO" });
}
