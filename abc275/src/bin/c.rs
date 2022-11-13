use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: [Chars; 9],
    };
    let mut count = 0_usize;
    for i1 in 0..9 {
        for j1 in 0..9 {
            if s[i1][j1] != '#' {
                continue;
            }
            for i2 in 0..9 {
                for j2 in 0..9 {
                    if i1 == i2 && j1 == j2 {
                        continue;
                    }

                    if s[i2][j2] != '#' {
                        continue;
                    }
                    let (di, dj) = (i2 as i64 - i1 as i64, j2 as i64 - j1 as i64);

                    let (i, j) = (i2 as i64 - dj, j2 as i64 + di);
                    if !(0..9 as i64).contains(&i) || !(0..9 as i64).contains(&j) {
                        continue;
                    }
                    let (i3, j3) = (i as usize, j as usize);
                    if s[i3][j3] != '#' {
                        continue;
                    }

                    let (i, j) = (i3 as i64 - di, j3 as i64 - dj);
                    if !(0..9 as i64).contains(&i) || !(0..9 as i64).contains(&j) {
                        continue;
                    }
                    let (i4, j4) = (i as usize, j as usize);
                    if s[i4][j4] != '#' {
                        continue;
                    }

                    count += 1;
                }
            }
        }
    }
    let ans = count / 4;
    println!("{}", ans);
}
