use proconio::input;
use proconio::marker::Usize1;

fn main() {
    input! {
        h: usize,
        w: usize,
        n: usize,
        m: usize,
        ab: [(Usize1,Usize1); n],
        cd: [(Usize1,Usize1); m],
    };
    let mut tbl = vec![vec![' '; w]; h];
    for (a_i, b_i) in ab {
        tbl[a_i][b_i] = '.';
    }
    for (c_i, d_i) in cd {
        tbl[c_i][d_i] = '#';
    }
    for i in 0..h {
        let mut l = false;
        for j in 0..w {
            match tbl[i][j] {
                '.' => {
                    l = true;
                }
                '#' => {
                    l = false;
                }
                ' ' => {
                    if l {
                        tbl[i][j] = ',';
                    }
                }
                ',' => {}
                _ => unreachable!(),
            }
        }
        let mut l = false;
        for j in (0..w).rev() {
            match tbl[i][j] {
                '.' => {
                    l = true;
                }
                '#' => {
                    l = false;
                }
                ' ' => {
                    if l {
                        tbl[i][j] = ',';
                    }
                }
                ',' => {}
                _ => unreachable!(),
            }
        }
    }
    for i in 0..w {
        let mut l = false;
        for j in 0..h {
            match tbl[j][i] {
                '.' => {
                    l = true;
                }
                '#' => {
                    l = false;
                }
                ' ' => {
                    if l {
                        tbl[j][i] = ',';
                    }
                }
                ',' => {}
                _ => unreachable!(),
            }
        }
        let mut l = false;
        for j in (0..h).rev() {
            match tbl[j][i] {
                '.' => {
                    l = true;
                }
                '#' => {
                    l = false;
                }
                ' ' => {
                    if l {
                        tbl[j][i] = ',';
                    }
                }
                ',' => {}
                _ => unreachable!(),
            }
        }
    }
    let mut count = 0;
    for i in 0..h {
        for j in 0..w {
            match tbl[i][j] {
                '.' | ',' => count += 1,
                _ => {}
            }
        }
    }
    let ans = count;
    println!("{}", ans);
}
