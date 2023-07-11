use proconio::{input, marker::Chars};

fn main() {
    input! {
        h: usize,
        w: usize,
        mut c: [Chars; h],
    };

    for i in 0..h {
        for j in 0..w {
            if c[i][j] != '.' {
                continue;
            }
            let mut set = vec![false; 5];
            let dir = vec![(-1, 0), (0, -1), (0, 1), (1, 0)];
            for (dr, dc) in dir {
                let (nr, nc) = (i as i64 + dr, j as i64 + dc);
                if !(0..h as i64).contains(&nr) || !(0..w as i64).contains(&nc) {
                    continue;
                }
                let (nr, nc) = (nr as usize, nc as usize);
                if c[nr][nc] == '.' {
                    continue;
                }
                set[(c[nr][nc] as u8 - b'1') as usize] = true;
            }
            for k in 0..5 {
                if set[k] {
                    continue;
                }
                c[i][j] = (k as u8 + b'1') as char;
                break;
            }
        }
    }
    for i in 0..h {
        for j in 0..w {
            print!("{}", c[i][j]);
        }
        println!();
    }
}
