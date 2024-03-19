use proconio::input;

fn main() {
    input! {
        a: usize,
        b: usize,
    };
    let ans = (a == 1 && b == 10) || (a + 1 == b);
    println!("{}", if ans { "Yes" } else { "No" });
}
