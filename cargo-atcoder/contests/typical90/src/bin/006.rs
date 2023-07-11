use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        n: usize,
        k: usize,
        s: Chars,
    };
    let chars = (0..26).map(|i| (b'a' + i) as char).collect::<Vec<char>>();
    let mut p = vec![vec![n; chars.len()]; n + 1];
    for (i, &s_i) in s.iter().enumerate().rev() {
        for (j, &c) in chars.iter().enumerate() {
            p[i][j] = if c == s_i { i } else { p[i + 1][j] };
        }
    }

    let mut i = 0;
    let mut t = vec![];
    while t.len() < k {
        for j in 0..26 {
            if p[i][j] <= n - (k - t.len()) {
                t.push((b'a' + j as u8) as char);
                i = p[i][j] + 1;
                break;
            }
        }
    }
    let ans = t.iter().collect::<String>();
    println!("{}", ans);
}
