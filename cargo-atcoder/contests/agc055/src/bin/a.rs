use proconio::{input, marker::Chars};
use superslice::Ext;

fn main() {
    input! {
        n: usize,
        s: Chars
    };

    let f = |c: char| -> usize { (c as u8 - b'A') as usize };
    let t = vec![&s[0..n], &s[n..2 * n], &s[2 * n..3 * n]];
    let mut count = vec![vec![0; 3]; 3];
    for i in 0..3 {
        for ch in t[i].iter().copied() {
            count[i][f(ch)] += 1;
        }
    }

    let mut group = 1;
    let mut ans = vec![0; 3 * n];
    let mut chars = "ABC".chars().collect::<Vec<char>>();
    loop {
        let mut min = n;
        for i in 0..3 {
            let ch = chars[i];
            min = min.min(count[i][f(ch)]);
        }
        for i in 0..3 {
            let ch = chars[i];
            count[i][f(ch)] -= min;
        }

        for i in 0..3 {
            let mut used = 0;
            for j in 0..n {
                if used < min && t[i][j] == chars[i] && ans[i * n + j] == 0 {
                    ans[i * n + j] = group;
                    used += 1;
                }
            }
        }

        if min > 0 {
            group += 1;
        }

        if !chars.next_permutation() {
            break;
        }
    }

    let ans = ans
        .into_iter()
        .map(|i| (i as u8 + b'0') as char)
        .collect::<String>();
    println!("{}", ans);
}
