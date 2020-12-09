use proconio::input;
use proconio::marker::Chars;

fn is_ok(n: usize, m: usize, s: &Vec<Vec<char>>) -> bool {
    let mut count = 0;
    let mut b = vec![vec![false; m]; n];
    for i in 0..n {
        for j in 0..m {
            if s[i][j] != '.' {
                continue;
            }
            if b[i][j] {
                continue;
            }
            count += 1;

            // dfs
            let mut stack = vec![];
            stack.push((i, j));
            while let Some((r, c)) = stack.pop() {
                b[r][c] = true;
                let dir = vec![(-1, 0), (0, -1), (0, 1), (1, 0)];
                for (dr, dc) in dir {
                    let (nr, nc) = (r as i64 + dr, c as i64 + dc);
                    if !(0..n as i64).contains(&nr) || !(0..m as i64).contains(&nc) {
                        continue;
                    }
                    let (nr, nc) = (nr as usize, nc as usize);
                    if s[nr][nc] != '.' {
                        continue;
                    }
                    if !b[nr][nc] {
                        b[nr][nc] = true;
                        stack.push((nr, nc));
                    }
                }
            }
        }
    }
    count == 1
}

fn main() {
    input! {
        n: usize,
        m: usize,
        mut s: [Chars; n],
    };
    let mut count = 0;
    for i in 0..n {
        for j in 0..m {
            if s[i][j] != '#' {
                continue;
            }
            s[i][j] = '.';
            if is_ok(n, m, &s) {
                count += 1;
            }
            s[i][j] = '#';
        }
    }
    let ans = count;
    println!("{}", ans);
}
