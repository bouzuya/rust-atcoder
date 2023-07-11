use proconio::input;

// fn f(memo: &mut Vec<Vec<usize>>, pa: &[(usize, usize)], inf: usize, l: usize, r: usize) -> usize {
//     if l >= r {
//         return 0;
//     }

//     if memo[l][r] != inf {
//         return memo[l][r];
//     }
//     let left = f(memo, pa, inf, l + 1, r)
//         + if (l + 1..=r).contains(&pa[l].0) {
//             pa[l].1
//         } else {
//             0
//         };
//     let right = f(memo, pa, inf, l, r - 1)
//         + if (l..r).contains(&pa[r].0) {
//             pa[r].1
//         } else {
//             0
//         };
//     let score = left.max(right);
//     memo[l][r] = score;
//     score
// }

// fn main() {
//     input! {
//         n: usize,
//         pa: [(Usize1, usize); n],
//     };
//     let inf = 1_usize << 60;
//     let mut memo = vec![vec![inf; n + 1]; n + 1];
//     memo[0][0] = 0;
//     let ans = f(&mut memo, &pa, inf, 0, n - 1);
//     println!("{}", ans);
// }

fn main() {
    input! {
        n: usize,
        pa: [(usize, usize); n],
    };
    let mut dp = vec![vec![0; n + 2]; n + 2];
    dp[1][n] = 0;
    for len in (0..=n - 2).rev() {
        for l in 1..=n - len {
            let r = l + len;

            let score1 = if l >= 2 && (l..=r).contains(&pa[l - 1 - 1].0) {
                pa[l - 1 - 1].1
            } else {
                0
            };

            let score2 = if r < n && (l..=r).contains(&pa[r + 1 - 1].0) {
                pa[r + 1 - 1].1
            } else {
                0
            };

            dp[l][r] = if l == 1 {
                dp[l][r + 1] + score2
            } else if r == n {
                dp[l - 1][r] + score1
            } else {
                (dp[l][r + 1] + score2).max(dp[l - 1][r] + score1)
            };
        }
    }

    let ans = (1..=n).map(|i| dp[i][i]).max().unwrap();
    println!("{}", ans);
}
