use proconio::input;

fn main() {
    input! {
        n: usize,
        ab: [(i64, i64); n],
    };
    let mut sum = 0_i64;
    for &(a_i, b_i) in ab.iter().rev() {
        let r = (sum + a_i) % b_i;
        sum += if r == 0 { 0 } else { b_i - r };
    }
    let ans = sum;
    println!("{}", ans);
}
