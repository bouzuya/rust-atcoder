use proconio::input;

fn main() {
    input! {
        a: usize,
        b: usize,
        c: usize,
    };
    let ans = (c > a + b) && (4 * a * b < (c - (a + b)).pow(2));
    println!("{}", if ans { "Yes" } else { "No" });
}
