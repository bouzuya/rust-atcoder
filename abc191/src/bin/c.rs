use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        h: usize,
        w: usize,
        s: [Chars; h],
    };

    let mut tbl = vec![vec![false; w + 1]; h + 1];
    for i in 1..h {
        for j in 1..w {
            if s[i - 1][j - 1] == '#'
                || s[i - 1][j + 0] == '#'
                || s[i + 0][j - 1] == '#'
                || s[i + 0][j + 0] == '#'
            {
                tbl[i][j] = true;
            }
        }
    }

    let mut ans = 0;
    for i in 1..h {
        for j in 1..w {
            if !tbl[i][j] {
                continue;
            }
            let count = vec![
                s[i - 1][j - 1] == '#',
                s[i - 1][j + 0] == '#',
                s[i + 0][j - 1] == '#',
                s[i + 0][j + 0] == '#',
            ]
            .iter()
            .filter(|&&x| x)
            .count();
            if count % 2 != 0 {
                ans += 1;
            }
        }
    }
    println!("{}", ans);
}
