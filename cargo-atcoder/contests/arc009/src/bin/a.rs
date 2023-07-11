use proconio::input;

fn main() {
    input! {
        n: usize,
        ab: [(i64, i64); n],
    };
    let mut sum = 0_i64;
    for &(a_i, b_i) in ab.iter() {
        sum += a_i * b_i;
    }
    let ans = sum * 105 / 100;
    println!("{}", ans);
}
