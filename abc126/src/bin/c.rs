use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
    };
    let mut ans = 0_f64;
    for i in 1..=n {
        let mut c = i;
        let mut p = 1;
        while c < k {
            p <<= 1;
            c <<= 1;
        }
        ans += (1_f64 / n as f64) / p as f64;
    }
    println!("{}", ans);
}
