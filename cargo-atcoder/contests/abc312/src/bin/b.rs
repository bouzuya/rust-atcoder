use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        m: usize,
        s: [Chars; n],
    };
    let mut ans = vec![];
    for i_tl in 0..n {
        'next: for j_tl in 0..m {
            if s[i_tl][j_tl] == '.' {
                continue;
            }
            if i_tl + 9 > n || j_tl + 9 > m {
                continue;
            }

            for i_offset in 0..3 {
                for j_offset in 0..3 {
                    let (i, j) = (i_tl + i_offset, j_tl + j_offset);
                    if s[i][j] == '.' {
                        continue 'next;
                    }
                }
            }
            for i_offset in 0..4 {
                let j_offset = 3;
                let (i, j) = (i_tl + i_offset, j_tl + j_offset);
                if s[i][j] == '#' {
                    continue 'next;
                }
            }
            for j_offset in 0..4 {
                let i_offset = 3;
                let (i, j) = (i_tl + i_offset, j_tl + j_offset);
                if s[i][j] == '#' {
                    continue 'next;
                }
            }

            for i_offset in 6..9 {
                let j_offset = 5;
                let (i, j) = (i_tl + i_offset, j_tl + j_offset);
                if s[i][j] == '#' {
                    continue 'next;
                }
            }
            for j_offset in 6..9 {
                let i_offset = 5;
                let (i, j) = (i_tl + i_offset, j_tl + j_offset);
                if s[i][j] == '#' {
                    continue 'next;
                }
            }
            for i_offset in 6..9 {
                for j_offset in 6..9 {
                    let (i, j) = (i_tl + i_offset, j_tl + j_offset);
                    if s[i][j] == '.' {
                        continue 'next;
                    }
                }
            }
            ans.push((i_tl, j_tl));
        }
    }

    for (i, j) in ans {
        println!("{} {}", i + 1, j + 1);
    }
}
