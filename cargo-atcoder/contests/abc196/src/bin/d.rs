use proconio::input;

fn dfs(
    count: &mut usize,
    used: &mut [Vec<bool>],
    h: usize,
    w: usize,
    a: usize,
    b: usize,
    c_a: usize,
    c_b: usize,
) {
    if used.iter().all(|row| row.iter().copied().all(|b| b)) {
        *count += 1;
        return;
    }

    for i in 0..h {
        let mut ok = false;
        for j in 0..w {
            if used[i][j] {
                continue;
            }

            if c_a < a {
                if j + 1 < w && !used[i][j + 1] {
                    used[i][j] = true;
                    used[i][j + 1] = true;
                    dfs(count, used, h, w, a, b, c_a + 1, c_b);
                    used[i][j] = false;
                    used[i][j + 1] = false;
                    ok = true;
                }
                if i + 1 < h && !used[i + 1][j] {
                    used[i][j] = true;
                    used[i + 1][j] = true;
                    dfs(count, used, h, w, a, b, c_a + 1, c_b);
                    used[i][j] = false;
                    used[i + 1][j] = false;
                    ok = true;
                }
            }
            if c_b < b {
                used[i][j] = true;
                dfs(count, used, h, w, a, b, c_a, c_b + 1);
                used[i][j] = false;
                ok = true;
            }
            if ok {
                break;
            }
        }
        if ok {
            break;
        }
    }
}

fn main() {
    input! {
        h: usize,
        w: usize,
        a: usize,
        b: usize,
    };
    let mut count = 0_usize;
    let mut used = vec![vec![false; w]; h];
    dfs(&mut count, &mut used, h, w, a, b, 0, 0);
    let ans = count;
    println!("{}", ans);
}
