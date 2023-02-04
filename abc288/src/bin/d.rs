use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        a: [i64; n],
        q: usize,
        lr: [(usize, usize); q],
    };
    let mut c = vec![0_i64; n + k];
    for i in 0..n {
        c[i + k] = c[i] + a[i];
    }
    for (l, r) in lr {
        let (l, r) = (l - 1, r - 1);
        let mut sum = vec![];
        for i in 0..k {
            let lx = l / k * k + i;
            let l2 = if lx < l { lx + k } else { lx };
            let rx = r / k * k + i;
            let r2 = if rx > r { rx - k } else { rx } + k;
            sum.push(c[r2] - c[l2]);
        }
        let ans = sum.iter().all(|&i| i == sum[0]);
        println!("{}", if ans { "Yes" } else { "No" });
    }
}
