use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
    };
    let mut c = 0;
    let mut x = n;
    while x > 0 {
        x /= k;
        c += 1;
    }
    let ans = c;
    println!("{}", ans);
}
