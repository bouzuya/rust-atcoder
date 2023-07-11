use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    };

    let mut dp = a;
    for i in (1..n).rev() {
        let mut parent = vec![0; dp.len() - 1];
        for j in 0..parent.len() {
            let l = dp[j];
            let r = dp[j + 1];
            parent[j] = if i % 2 == 0 { l.min(r) } else { l.max(r) };
        }
        dp = parent;
    }

    let ans = dp[0];
    println!("{}", ans);
}
