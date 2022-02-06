use proconio::input;

fn main() {
    input! {
        n: usize,
        b: [usize; n],
        r: [usize; n],
    };
    let sum_b = b.into_iter().sum::<usize>();
    let sum_r = r.into_iter().sum::<usize>();
    let e_b = sum_b as f64 / n as f64;
    let e_r = sum_r as f64 / n as f64;
    let e = e_b + e_r;
    let ans = e;
    println!("{}", ans);
}
