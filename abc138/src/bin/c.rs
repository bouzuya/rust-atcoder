use proconio::input;

fn main() {
    input! {
        n: usize,
        mut v: [i64; n],
    };
    v.sort();
    let mut x = v[0] as f64;
    for &v_i in v.iter().skip(1) {
        x += v_i as f64;
        x /= 2_f64;
    }
    let ans = x;
    println!("{}", ans);
}
