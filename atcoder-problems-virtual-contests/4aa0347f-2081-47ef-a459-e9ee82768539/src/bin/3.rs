use proconio::input;

fn main() {
    input! {
        n: usize,
        mut a: [i64; n],
    }
    a.sort();
    let mut prev = 0;
    let mut product = 1_i64;
    for a_i in a.iter().copied() {
        product *= a_i - prev + 1;
        product %= 1_000_000_007;
        prev = a_i;
    }
    let ans = product;
    println!("{}", ans);
}
