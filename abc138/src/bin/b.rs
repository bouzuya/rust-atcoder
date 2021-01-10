use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [i64; n],
    };
    let mut s = 0_f64;
    for &a_i in a.iter() {
        s += 1_f64 / a_i as f64;
    }
    let ans = 1_f64 / s;
    println!("{}", ans);
}
