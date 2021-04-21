use proconio::input;

fn main() {
    input! {
        n: usize,
        mut r: [f64; n],
    };
    r.sort_by(|&a, &b| b.partial_cmp(&a).unwrap());
    let mut a = 0_f64;
    for (i, r_i) in r.iter().enumerate() {
        a += if i % 2 == 0 { 1_f64 } else { -1_f64 } * r_i * r_i * std::f64::consts::PI;
    }
    let ans = a;
    println!("{}", ans);
}
