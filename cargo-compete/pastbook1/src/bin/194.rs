use im_rc::HashMap;
use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        m: usize,
        a: [Chars; n],
    }
    let mut pos = vec![vec![]; 10 + 1];
    for i in 0..n {
        for j in 0..m {
            match a[i][j] {
                'S' => pos[0].push((i, j)),
                'G' => pos[10].push((i, j)),
                '1'..='9' => pos[(a[i][j] as u8 - b'0') as usize].push((i, j)),
                _ => unreachable!(),
            }
        }
    }

    let dist = |p: (usize, usize), q: (usize, usize)| -> i64 {
        (p.0 as i64 - q.0 as i64).abs() + (p.1 as i64 - q.1 as i64).abs()
    };
    let mut dp = vec![HashMap::new(); 10 + 1];
    dp[0].insert(pos[0][0], 0);
    for i in 0..10 {
        let keys = dp[i].keys().cloned().collect::<Vec<_>>();
        for p in keys {
            let d_p = *dp[i].get(&p).unwrap();
            for q in pos[i + 1].iter().copied() {
                let d_q = d_p + dist(q, p);
                let entry = dp[i + 1].entry(q).or_insert(d_q);
                if *entry > d_q {
                    *entry = d_q;
                }
            }
        }
    }

    if dp[10].is_empty() {
        println!("-1");
        return;
    }
    let ans = dp[10].iter().next().unwrap().1;
    println!("{}", ans);
}
