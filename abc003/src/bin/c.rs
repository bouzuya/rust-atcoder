use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        mut r: [usize; n],
    };
    r.sort();
    let mut c = 0_f64;
    for &r_i in r.iter().rev().take(k).rev() {
        c = (c + r_i as f64) / 2_f64;
    }
    let ans = c;
    println!("{}", ans);
}
