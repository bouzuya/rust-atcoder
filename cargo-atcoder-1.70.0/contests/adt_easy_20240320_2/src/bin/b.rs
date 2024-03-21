use proconio::input;

fn main() {
    input! {
        a: usize,
        b: usize,
    };
    let ans = a == b / 2;
    println!("{}", if ans { "Yes" } else { "No" });
}
