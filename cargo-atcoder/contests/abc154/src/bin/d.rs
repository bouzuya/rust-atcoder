use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        p: [usize; n],
    };
    let mut max_index = 0_usize;
    let mut max = 0_usize;
    for i in 0..k {
        max += p[i];
    }
    let mut prev = max;
    for l in 1..n - k + 1 {
        let curr = prev - p[l - 1] + p[l + k - 1];
        if curr > max {
            max = curr;
            max_index = l;
        }
        prev = curr;
    }

    let mut ans = 0_f64;
    for i in 0..k {
        let x = p[max_index + i] as f64;
        ans += (((1_f64 + x) * x) / 2_f64) / x;
    }
    println!("{}", ans);
}
