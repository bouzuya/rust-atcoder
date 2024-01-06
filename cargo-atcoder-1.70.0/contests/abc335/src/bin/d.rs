use proconio::input;

fn main() {
    input! {
        n: usize,
    };
    let mut t = vec![vec!["0".to_string(); n]; n];
    t[n / 2][n / 2] = "T".to_string();
    let dir = vec![(0, 1), (1, 0), (0, -1), (-1, 0)];
    let mut d = 0_usize;
    let mut i = 0_usize;
    let mut j = 0_usize;
    let mut count = 1_usize;
    t[0][0] = count.to_string();
    count += 1;
    for _ in 0..n * n - 1 {
        let (dr, dc) = dir[d];
        loop {
            let (nr, nc) = (i as i64 + dr, j as i64 + dc);
            if !(0..n as i64).contains(&nr) || !(0..n as i64).contains(&nc) {
                break;
            }
            let (nr, nc) = (nr as usize, nc as usize);
            if t[nr][nc] == "T" {
                i = nr;
                j = nc;
                break;
            }
            if t[nr][nc] != "0" {
                break;
            }
            t[nr][nc] = count.to_string();
            count += 1;
            i = nr;
            j = nc;
        }
        d += 1;
        d %= dir.len();
    }

    for i in 0..n {
        for j in 0..n {
            print!("{}{}", t[i][j], if j == n - 1 { "\n" } else { " " });
        }
    }
}
