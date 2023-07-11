use proconio::input;

fn main() {
    input! {
        n: usize,
        a: u64,
        b: u64,
    };
    let mut ans = 0;
    for i in 1..=n {
        let mut x = i as u64;
        let mut s = 0;
        while x > 0 {
            s += x % 10;
            x /= 10;
        }
        if (a..=b).contains(&s) {
            ans += i;
        }
    }
    println!("{}", ans);
}
