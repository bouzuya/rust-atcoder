use proconio::input;

fn main() {
    input! {
        a: usize,
        b: usize
    };
    let mut m = b;
    let mut n = 0;
    while m > 0 {
        m /= 10;
        n += 1;
    }
    let ans = (a * 10_usize.pow(n) + b) * 2;
    println!("{}", ans);
}
