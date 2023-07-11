use proconio::{input, marker::Usize1};

fn main() {
    input! {
        h: usize,
        w: usize,
        n: usize,
        a: [[Usize1; w]; h],
        c: [Usize1; n],
    };
    for i in 0..h {
        for j in 0..w {
            let dir = vec![(-1, 0), (0, -1), (0, 1), (1, 0)];
            for (dr, dc) in dir {
                let (nr, nc) = (i as i64 + dr, j as i64 + dc);
                if !(0..h as i64).contains(&nr) || !(0..w as i64).contains(&nc) {
                    continue;
                }
                let (nr, nc) = (nr as usize, nc as usize);
                if a[i][j] != a[nr][nc] && c[a[i][j]] == c[a[nr][nc]] {
                    println!("No");
                    return;
                }
            }
        }
    }
    println!("Yes");
}
