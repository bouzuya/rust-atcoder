use proconio::{input, marker::Chars};

fn main() {
    input! {
        h: usize,
        w: usize,
        c: [Chars; h],
    };
    let mut a = vec![vec![26_usize; w]; h];
    for i in 0..h {
        for j in 0..w {
            a[i][j] = (c[i][j] as u8 - b'a') as usize;
        }
    }

    let m = 26;
    let mut row = vec![vec![0_usize; m]; h];
    let mut col = vec![vec![0_usize; m]; w];
    for r in 0..h {
        for c in 0..w {
            row[r][a[r][c]] += 1;
            col[c][a[r][c]] += 1;
        }
    }

    let mut row_deleted = vec![false; h];
    let mut col_deleted = vec![false; w];

    loop {
        let mut delete_rows = vec![];
        let mut delete_cols = vec![];
        for r in 0..h {
            if row_deleted[r] {
                continue;
            }
            let mut count = 0_usize;
            let mut kinds = 0_usize;
            for j in 0..m {
                count += row[r][j];
                if row[r][j] > 0 {
                    kinds += 1;
                }
            }
            if count >= 2 && kinds == 1 {
                delete_rows.push(r);
            }
        }
        for c in 0..w {
            if col_deleted[c] {
                continue;
            }
            let mut count = 0_usize;
            let mut kinds = 0_usize;
            for j in 0..m {
                count += col[c][j];
                if col[c][j] > 0 {
                    kinds += 1;
                }
            }
            if count >= 2 && kinds == 1 {
                delete_cols.push(c);
            }
        }

        for r in delete_rows.iter().copied() {
            for c in 0..w {
                if row_deleted[r] || col_deleted[c] {
                    continue;
                }
                row[r][a[r][c]] -= 1;
                col[c][a[r][c]] -= 1;
            }
            row_deleted[r] = true;
        }
        for c in delete_cols.iter().copied() {
            for r in 0..h {
                if row_deleted[r] || col_deleted[c] {
                    continue;
                }
                row[r][a[r][c]] -= 1;
                col[c][a[r][c]] -= 1;
            }
            col_deleted[c] = true;
        }

        if delete_rows.is_empty() && delete_cols.is_empty() {
            break;
        }
    }

    let mut ans = 0_usize;
    for i in 0..h {
        for j in 0..w {
            if row_deleted[i] || col_deleted[j] {
                continue;
            }
            ans += 1;
        }
    }
    println!("{}", ans);
}
