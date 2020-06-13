use proconio::input;

fn main() {
    input! { n: usize };
    let mut t = vec![];
    for _ in 0..n {
        input! {
            k_i: usize,
            t_i: [i64; k_i],
        };
        t.push(t_i);
    }
    input! {
        m: usize,
        a: [usize; m],
    };

    let mut tj = vec![0; n];
    let mut a1 = std::collections::BTreeSet::new();
    let mut a2 = std::collections::BTreeSet::new();
    for (i, t_i) in t.iter().enumerate() {
        for j in 0..std::cmp::min(2, t_i.len()) {
            if j == 0 {
                a1.insert((t[i][j], i));
            }
            if j == 1 {
                a2.insert((t[i][j], i));
            }
        }
    }

    for &a_i in a.iter() {
        match a_i {
            1 => match a1.iter().rev().next() {
                Some(&(t_ij, i)) => {
                    a1.remove(&(t_ij, i));
                    tj[i] += 1;
                    let j1 = tj[i];
                    let j2 = j1 + 1;
                    if j1 < t[i].len() {
                        a1.insert((t[i][j1], i));
                        a2.remove(&(t[i][j1], i));
                    }
                    if j2 < t[i].len() {
                        a2.insert((t[i][j2], i));
                    }
                    println!("{}", t_ij);
                }
                None => unreachable!(),
            },
            2 => match (a1.iter().rev().next(), a2.iter().rev().next()) {
                (Some(&(t_ij1, i1)), Some(&(t_ij2, i2))) => {
                    if t_ij1 >= t_ij2 {
                        let (t_ij, i) = (t_ij1, i1);
                        a1.remove(&(t_ij, i));
                        tj[i] += 1;
                        let j1 = tj[i];
                        let j2 = j1 + 1;
                        if j1 < t[i].len() {
                            a1.insert((t[i][j1], i));
                            a2.remove(&(t[i][j1], i));
                        }
                        if j2 < t[i].len() {
                            a2.insert((t[i][j2], i));
                        }
                        println!("{}", t_ij);
                    } else {
                        let (t_ij, i) = (t_ij2, i2);
                        a2.remove(&(t_ij, i));
                        tj[i] += 1;
                        let j1 = tj[i];
                        let j2 = j1 + 1;
                        if j2 < t[i].len() {
                            a2.insert((t[i][j2], i));
                        }
                        println!("{}", t_ij);
                    };
                }
                (Some(&(t_ij, i)), None) => {
                    a1.remove(&(t_ij, i));
                    tj[i] += 1;
                    let j1 = tj[i];
                    let j2 = j1 + 1;
                    if j1 < t[i].len() {
                        a1.insert((t[i][j1], i));
                        a2.remove(&(t[i][j1], i));
                    }
                    if j2 < t[i].len() {
                        a2.insert((t[i][j2], i));
                    }
                    println!("{}", t_ij);
                }
                (None, Some(_)) => unreachable!(),
                (None, None) => unreachable!(),
            },
            _ => unreachable!(),
        }
    }
}
