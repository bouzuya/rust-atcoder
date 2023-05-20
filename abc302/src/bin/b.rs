use proconio::{input, marker::Chars};

fn main() {
    input! {
        h: usize,
        w: usize,
        s: [Chars; h],
    };

    for i in 0..h {
        for j in 0..w {
            for dr in -1..=1 {
                for dc in -1..=1 {
                    let mut ans = vec![];
                    let mut ok = true;
                    for (k, c) in "snuke".chars().enumerate() {
                        let k = k as i64;
                        let (nr, nc) = (i as i64 + k * dr, j as i64 + k * dc);
                        if !(0..h as i64).contains(&nr) || !(0..w as i64).contains(&nc) {
                            ok = false;
                            break;
                        }
                        let (nr, nc) = (nr as usize, nc as usize);
                        if s[nr][nc] != c {
                            ok = false;
                            break;
                        }
                        ans.push((nr, nc));
                    }
                    if ok {
                        for (r, c) in ans {
                            println!("{} {}", r + 1, c + 1);
                        }
                        return;
                    }
                }
            }
        }
    }
}
