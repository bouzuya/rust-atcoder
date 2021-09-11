use proconio::{input, marker::Chars};

fn rotate(n: usize, s: &mut [Vec<char>]) -> Vec<Vec<char>> {
    let mut t = vec![vec![' '; n]; n];
    for i in 0..n {
        for j in 0..n {
            t[j][n - 1 - i] = s[i][j];
        }
    }
    t
}

fn count(n: usize, s: &[Vec<char>]) -> usize {
    let mut count = 0;
    for i in 0..n {
        for j in 0..n {
            if s[i][j] == '#' {
                count += 1;
            }
        }
    }
    count
}

fn main() {
    input! {
        n: usize,
        mut s: [Chars; n],
        t: [Chars; n],
    };
    let s_count = count(n, &s);
    let t_count = count(n, &t);
    if s_count != t_count {
        println!("No");
        return;
    }

    for _ in 0..4 {
        // for i in 0..n {
        //     for j in 0..n {
        //         print!("{}", s[i][j]);
        //     }
        //     println!();
        // }
        // println!();

        let m = n as i64;
        for oi in -m..=m {
            for oj in -m..=m {
                let mut c = 0;
                let mut ok = true;
                'outer: for i in 0..m {
                    for j in 0..m {
                        let (si, sj) = (i as usize, j as usize);
                        let sij = s[si][sj];
                        if sij == '#' {
                            if !(0..m).contains(&(i + oi)) || !(0..m).contains(&(j + oj)) {
                                ok = false;
                                break 'outer;
                            }
                        }

                        if (0..m).contains(&(i + oi)) && (0..m).contains(&(j + oj)) {
                            let ti = (i + oi) as usize;
                            let tj = (j + oj) as usize;
                            if s[si][sj] != t[ti][tj] {
                                ok = false;
                                break 'outer;
                            } else {
                                if s[si][sj] == '#' {
                                    c += 1;
                                }
                            }
                        }
                    }
                }
                if c != s_count || c != t_count {
                    ok = false;
                }
                if ok {
                    println!("Yes");
                    return;
                }
            }
        }

        s = rotate(n, &mut s);
    }
    println!("No");
}
