use proconio::input;

fn main() {
    input! {
        x: u64
    };
    let mut ans = 0;
    let mut n = 100;
    while n < x {
        ans += 1;
        n += n / 100;
    }
    println!("{}", ans);
}
