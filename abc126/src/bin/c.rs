use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
    };
    let mut res = 0_f64;
    for x in 1..=n {
        let c = {
            let mut m = x;
            let mut c = 0;
            while m < k {
                c += 1;
                m *= 2;
            }
            c
        };
        res += 1_f64 / 2_usize.pow(c) as f64
    }
    let ans = res / n as f64;
    println!("{}", ans);
}
