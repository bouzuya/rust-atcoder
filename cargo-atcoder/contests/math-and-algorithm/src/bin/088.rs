use proconio::input;

fn main() {
    input! {
        a: usize,
        b: usize,
        c: usize,
    };
    let modp = 998_244_353;
    let f = |x| (x + 1) * x / 2 % modp;
    let ans = f(a) * f(b) % modp * f(c) % modp;
    println!("{}", ans);
}
