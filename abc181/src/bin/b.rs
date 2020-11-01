use proconio::input;

fn main() {
    input! {
        n: usize,
        ab: [(i64, i64); n],
    };
    let mut sum = 0_i64;
    for (a_i, b_i) in ab {
        sum += (b_i - a_i + 1) * (a_i + b_i) / 2;
    }
    let ans = sum;
    println!("{}", ans);
}
