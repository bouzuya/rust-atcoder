use proconio::input;

fn main() {
    input! {
        a: usize,
        b: usize,
        n: usize,
    };
    let f = |x: usize| -> usize { (a * x) / b - a * (x / b) };
    let c = n / b;
    let r = (c * b).saturating_sub(1);
    let ans = f(r).max(f(n));
    println!("{}", ans);
}
