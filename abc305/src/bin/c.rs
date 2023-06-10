use proconio::{input, marker::Chars};

fn main() {
    input! {
        h: usize,
        w: usize,
        s: [Chars; h],
    };

    for i in 0..h {
        for j in 0..w {
            if s[i][j] == '#' {
                continue;
            }

            let mut count = 0_usize;
            let dir = vec![(-1, 0), (0, -1), (0, 1), (1, 0)];
            for (dr, dc) in dir {
                let (nr, nc) = (i as i64 + dr, j as i64 + dc);
                if !(0..h as i64).contains(&nr) || !(0..w as i64).contains(&nc) {
                    continue;
                }
                let (nr, nc) = (nr as usize, nc as usize);
                if s[nr][nc] == '#' {
                    count += 1;
                }
            }
            if count >= 2 {
                println!("{} {}", i + 1, j + 1);
                return;
            }
        }
    }
}
