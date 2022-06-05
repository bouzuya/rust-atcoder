use proconio::input;

fn main() {
    input! {
        n: usize,
    };
    let mut ans = 0_usize;
    for i in 1..=n {
        let mut x = i;
        for k2 in (2_usize..).map(|k| k.pow(2)) {
            if k2 > x {
                break;
            }
            while x % k2 == 0 {
                x /= k2;
            }
        }
        ans += (1_usize..)
            .map(|k| k.pow(2))
            .take_while(|&k2| x * k2 <= n)
            .count();
    }
    println!("{}", ans);
}
