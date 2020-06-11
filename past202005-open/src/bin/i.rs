use proconio::input;

fn swap(a: &mut Vec<Vec<usize>>, a_x: usize, b_x: usize, row: bool) {
    if row {
        let t = a[a_x][0];
        a[a_x][0] = a[b_x][0];
        a[b_x][0] = t;
    } else {
        let t = a[0][a_x];
        a[0][a_x] = a[0][b_x];
        a[0][b_x] = t;
    }
}

fn main() {
    input! {
        n: usize,
        q: usize,
    };
    let mut a = vec![vec![0; n + 1]; n + 1];
    for i in 1..=n {
        for j in 1..=n {
            a[i][j] = n * (i - 1) + j - 1;
        }
    }
    for i in 1..=n {
        a[i][0] = i; // a[y][0] = r // y 行目の実体は r 行目
        a[0][i] = i; // a[0][x] = c // x 列目の実体は c 列目
    }
    let mut is_ij_rc = true;
    for _ in 0..q {
        input! { t: usize };
        match t {
            1 => {
                input! { r_a: usize, r_b: usize };
                swap(&mut a, r_a, r_b, is_ij_rc);
            }
            2 => {
                input! { c_a: usize, c_b: usize };
                swap(&mut a, c_a, c_b, !is_ij_rc);
            }
            3 => {
                is_ij_rc = !is_ij_rc;
            }
            4 => {
                input! { r_a: usize, c_b: usize };
                let (r, c) = if is_ij_rc {
                    (a[r_a][0], a[0][c_b])
                } else {
                    (a[c_b][0], a[0][r_a])
                };
                let ans = a[r][c];
                println!("{}", ans);
            }
            _ => unreachable!(),
        }
    }
}
