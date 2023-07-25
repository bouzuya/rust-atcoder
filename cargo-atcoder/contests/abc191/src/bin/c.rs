use proconio::{input, marker::Chars};

fn main() {
    input! {
        h: usize,
        w: usize,
        s: [Chars; h],
    };

    let mut count = 0_usize;
    for i in 0..h {
        for j in 0..w {
            if s[i][j] == '#' {
                continue;
            }

            let mut b = vec![vec![false; 3]; 3];
            for dr in -1..=1 {
                for dc in -1..=1 {
                    let (nr, nc) = (i as i64 + dr, j as i64 + dc);
                    if !(0..h as i64).contains(&nr) || !(0..w as i64).contains(&nc) {
                        continue;
                    }
                    let (nr, nc) = (nr as usize, nc as usize);
                    if s[nr][nc] == '.' {
                        continue;
                    }
                    b[(1 + dr) as usize][(1 + dc) as usize] = true;
                }
            }

            if b[0][2] && (b[0][1] == b[1][2]) {
                count += 1;
            }
            if b[2][2] && (b[1][2] == b[2][1]) {
                count += 1;
            }
            if b[2][0] && (b[2][1] == b[1][0]) {
                count += 1;
            }
            if b[0][0] && (b[1][0] == b[0][1]) {
                count += 1;
            }
        }
    }
    let ans = count;
    println!("{}", ans);
}
