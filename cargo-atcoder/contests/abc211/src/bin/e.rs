use proconio::{input, marker::Chars};

fn dfs(n: usize, k: usize, s: &mut Vec<Vec<char>>, res: &mut usize, c: usize) {
    if k == c {
        *res += 1;
        return;
    }
    for i in 0..n {
        for j in 0..n {
            if s[i][j] != '.' {
                continue;
            }
            if c > 0 {
                let mut ok = false;
                let dir = vec![(-1, 0), (0, -1), (0, 1), (1, 0)];
                for (dr, dc) in dir {
                    let (nr, nc) = (i as i64 + dr, j as i64 + dc);
                    if !(0..n as i64).contains(&nr) || !(0..n as i64).contains(&nc) {
                        continue;
                    }
                    let (nr, nc) = (nr as usize, nc as usize);
                    if s[nr][nc] == '@' {
                        ok = true;
                    }
                }
                if !ok {
                    continue;
                }
            }
            s[i][j] = '@';
            dfs(n, k, s, res, c + 1);
            s[i][j] = '#';
            dfs(n, k, s, res, c);
            s[i][j] = '.';
            return;
        }
    }
}

fn main() {
    input! {
        n: usize,
        k: usize,
        mut s: [Chars; n],
    };
    let mut res = 0;
    dfs(n, k, &mut s, &mut res, 0);
    let ans = res;
    println!("{}", ans);
}
