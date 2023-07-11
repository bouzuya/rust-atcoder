use proconio::input;

fn main() {
    input! {
        n: usize,
    };
    let mut ans = 0_usize;
    let mut l = 1;
    while l <= n {
        let x = n / l;
        let r = n / x + 1;
        ans += (r - l) * x;
        l = r;
    }
    println!("{}", ans);
}
