use proconio::input;

fn main() {
    input! {
        n: usize,
        mut ab: [(i64, i64); n],
    };
    ab.sort();
    let (a_n, b_n) = ab[n - 1];
    let ans = a_n + b_n;
    println!("{}", ans);
}
