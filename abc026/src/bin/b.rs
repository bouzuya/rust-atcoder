use proconio::input;

fn main() {
    input! {
        n: usize,
        mut r: [i64; n],
    };
    r.sort();
    let mut sum = 0_i64;
    let mut s = 1_i64;
    for &r_i in r.iter().rev() {
        sum += s * r_i.pow(2);
        s *= -1;
    }
    let ans = sum as f64 * std::f64::consts::PI;
    println!("{}", ans);
}
