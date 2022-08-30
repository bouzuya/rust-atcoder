use proconio::input;

fn main() {
    input! {
        n: usize,
    };

    let mut ans = 3.5_f64;
    for _ in 1..n {
        let mut s = 0_f64;
        for x in 1..=6 {
            s += (x as f64).max(ans);
        }
        s /= 6_f64;
        ans = s;
    }
    println!("{}", ans);
}
