use proconio::input;

fn main() {
    input! {
        a: usize,
        b: usize,
    };
    let ans = (a..=b).any(|x| 100 % x == 0);
    println!("{}", if ans { "Yes" } else { "No" });
}
