use proconio::input;

fn main() {
    input! {
        mut n: usize,
        k: usize,
    };
    let mut ans = 0;
    let mut x = n;
    while x > 1 {
        x /= k;
        ans += 1;
    }
    if x == 1 {
        ans += 1;
    }
    println!("{}", ans);
}
