use proconio::{input, marker::Chars};

fn rotate(p: &[Vec<usize>]) -> Vec<Vec<usize>> {
    let mut v = vec![vec![0; 4]; 4];
    for i in 0..4 {
        for j in 0..4 {
            v[j][4 - 1 - i] = p[i][j];
        }
    }
    v
}

#[allow(dead_code)]
fn print(p: &[Vec<usize>]) {
    for i in 0..4 {
        for j in 0..4 {
            print!("{}", p[i][j]);
        }
        println!();
    }
}

fn f(p: &[Vec<Vec<usize>>], o: &[(i64, i64)]) -> bool {
    let mut board = vec![vec![0_usize; 4]; 4];
    for k in 0..3 {
        for i in 0..4 {
            for j in 0..4 {
                if p[k][i][j] != 1 {
                    continue;
                }
                if !(0..4).contains(&(o[k].0 + i as i64)) || !(0..4).contains(&(o[k].1 + j as i64))
                {
                    return false;
                }
                let (bi, bj) = ((o[k].0 + i as i64) as usize, (o[k].1 + j as i64) as usize);
                board[bi][bj] += p[k][i][j];
                if board[bi][bj] > 1 {
                    return false;
                }
            }
        }
    }

    for i in 0..4 {
        for j in 0..4 {
            if board[i][j] != 1 {
                return false;
            }
        }
    }

    true
}

fn main() {
    input! {
        mut p: [[Chars; 4]; 3],
    };
    let mut p = {
        let mut v = vec![vec![vec![0_usize; 4]; 4]; 4];
        for k in 0..3 {
            for i in 0..4 {
                for j in 0..4 {
                    v[k][i][j] = if p[k][i][j] == '#' { 1 } else { 0 };
                }
            }
        }
        v
    };

    let mut offsets = vec![];
    for i in -3..=3 {
        for j in -3..=3 {
            offsets.push((i, j));
        }
    }

    for _ in 0..4 {
        p[0] = rotate(&p[0]);
        for o0 in offsets.iter().copied() {
            for _ in 0..4 {
                p[1] = rotate(&p[1]);
                for o1 in offsets.iter().copied() {
                    for _ in 0..4 {
                        p[2] = rotate(&p[2]);
                        for o2 in offsets.iter().copied() {
                            if f(&p, &vec![o0, o1, o2]) {
                                println!("Yes");
                                return;
                            }
                        }
                    }
                }
            }
        }
    }
    println!("No");
}
