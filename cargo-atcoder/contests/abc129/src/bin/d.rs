use proconio::{input, marker::Chars};

fn main() {
    input! {
        h: usize,
        w: usize,
        s: [Chars; h],
    };

    let mut left = vec![vec![0; w]; h];
    for i in 0..h {
        let mut count = 0;
        for j in 0..w {
            if s[i][j] == '.' {
                left[i][j] = count;
                count += 1;
            } else {
                count = 0;
            }
        }
    }

    let mut right = vec![vec![0; w]; h];
    for i in 0..h {
        let mut count = 0;
        for j in (0..w).rev() {
            if s[i][j] == '.' {
                right[i][j] = count;
                count += 1;
            } else {
                count = 0;
            }
        }
    }

    let mut up = vec![vec![0; w]; h];
    for j in 0..w {
        let mut count = 0;
        for i in 0..h {
            if s[i][j] == '.' {
                up[i][j] = count;
                count += 1;
            } else {
                count = 0;
            }
        }
    }

    let mut down = vec![vec![0; w]; h];
    for j in 0..w {
        let mut count = 0;
        for i in (0..h).rev() {
            if s[i][j] == '.' {
                down[i][j] = count;
                count += 1;
            } else {
                count = 0;
            }
        }
    }

    let mut ans = 0;
    for i in 0..h {
        for j in 0..w {
            if s[i][j] == '#' {
                continue;
            }

            ans = ans.max(1 + left[i][j] + right[i][j] + up[i][j] + down[i][j]);
        }
    }

    println!("{}", ans);
}
