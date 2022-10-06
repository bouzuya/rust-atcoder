use proconio::input;

fn main() {
    input! {
        n: usize,
        abcd: [(usize, usize, usize, usize); n],
    };
    let m = 1500;
    let mut count = vec![vec![0_i64; m + 2]; m + 2];
    for (a, b, c, d) in abcd {
        count[a][b] += 1;
        count[a][d] -= 1;
        count[c][b] -= 1;
        count[c][d] += 1;
    }
    for i in 0..=m {
        for j in 0..=m {
            count[i][j + 1] += count[i][j];
        }
    }
    for j in 0..=m {
        for i in 0..=m {
            count[i + 1][j] += count[i][j];
        }
    }

    let mut area = 0_usize;
    for i in 0..m {
        for j in 0..m {
            if count[i][j] > 0 {
                area += 1;
            }
        }
    }
    let ans = area;
    println!("{}", ans);
}
