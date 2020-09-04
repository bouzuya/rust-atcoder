use proconio::input;

fn main() {
    input! {
        n: usize,
        abcde: [(i64, i64, i64, i64, i64); n],
    };
    let mut max = 0_f64;
    for &(a_i, b_i, c_i, d_i, e_i) in abcde.iter() {
        let x = (a_i + b_i + c_i + d_i) as f64 + (e_i as f64 * 110_f64 / 900_f64);
        if x > max {
            max = x;
        }
    }
    let ans = max;
    println!("{}", ans);
}
