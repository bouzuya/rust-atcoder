use proconio::input;
use proconio::marker::Usize1;

fn main() {
    input! {
        n: usize,
        m: usize,
        a: [Usize1; m],
    };

    let dc = vec![2, 5, 5, 4, 5, 6, 3, 7, 6]
        .iter()
        .enumerate()
        .filter(|(i, _)| a.iter().find(|&a_i| a_i == i).is_some())
        .map(|(i, &c)| ((i + 1) as i64, c))
        .collect::<Vec<(i64, usize)>>();
    let mut dp = vec![-1_000_000_000_i64; n + 1];
    dp[0] = 0;
    for i in 0..n {
        for &(_, c) in dc.iter() {
            if i + c <= n {
                dp[i + c] = std::cmp::max(dp[i + c], dp[i] + 1);
            }
        }
    }

    let mut dcdesc = dc.clone();
    dcdesc.sort_by_key(|&(d, _)| -d);
    let mut ans = vec![];
    let mut nr = n;
    while nr > 0 {
        for &(d, c) in dcdesc.iter() {
            if nr >= c && dp[nr - c] == dp[nr] - 1 {
                ans.push(d);
                nr -= c;
                break;
            }
        }
    }
    for a in ans {
        print!("{}", a);
    }
    println!();
}
