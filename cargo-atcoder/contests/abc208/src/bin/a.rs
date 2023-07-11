use proconio::input;

fn main() {
    input! {
        a: usize,
        b: usize,
    };
    let ans = (a..=6 * a).contains(&b);
    println!("{}", if ans { "Yes" } else { "No" });
}
