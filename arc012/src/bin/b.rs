use proconio::input;

fn main() {
    input! {
        n: usize,
        v_a: f64,
        v_b: f64,
        l: f64,
    };
    let mut d = l;
    for _ in 0..n {
        let s = d / v_a;
        d = v_b * s;
    }
    let ans = d;
    println!("{}", ans);
}
