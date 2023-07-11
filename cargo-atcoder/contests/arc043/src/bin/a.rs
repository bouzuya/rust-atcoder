use proconio::input;

fn main() {
    input! {
        n: usize,
        a: i64,
        b: i64,
        s: [i64; n],
    };
    let mut s_max = s[0];
    let mut s_min = s[0];
    for &s_i in s.iter().skip(1) {
        s_max = std::cmp::max(s_max, s_i);
        s_min = std::cmp::min(s_min, s_i);
    }
    if s_max - s_min == 0 {
        println!("-1");
        return;
    }
    let p = b as f64 / (s_max - s_min) as f64;
    let mut sum = 0_f64;
    for &s_i in s.iter() {
        sum += p * s_i as f64;
    }
    let avg = sum / s.len() as f64;
    let q = a as f64 - avg;
    println!("{} {}", p, q);
}
