use std::collections::HashSet;

use proconio::{input, marker::Chars};

fn main() {
    input! {
        h_a: usize,
        w_a: usize,
        a: [Chars; h_a],
        h_b: usize,
        w_b: usize,
        b: [Chars; h_b],
        h_x: usize,
        w_x: usize,
        x: [Chars; h_x],
    };

    let mut set_a = HashSet::new();
    for i in 0..h_a {
        for j in 0..w_a {
            if a[i][j] == '#' {
                set_a.insert((i, j));
            }
        }
    }

    let mut set_b = HashSet::new();
    for i in 0..h_b {
        for j in 0..w_b {
            if b[i][j] == '#' {
                set_b.insert((i, j));
            }
        }
    }

    let mut set_x = HashSet::new();
    for i in 0..h_x {
        for j in 0..w_x {
            if x[i][j] == '#' {
                set_x.insert((i, j));
            }
        }
    }

    for i_a in -10_i64..10 {
        for j_a in -10_i64..10 {
            for i_b in -10_i64..10 {
                'outer: for j_b in -10_i64..10 {
                    let mut x = HashSet::new();
                    for (i, j) in set_a.clone() {
                        let i = i as i64 + i_a;
                        let j = j as i64 + j_a;
                        if !(0..h_x as i64).contains(&i) || !(0..w_x as i64).contains(&j) {
                            continue 'outer;
                        }
                        x.insert((i as usize, j as usize));
                    }
                    for (i, j) in set_b.clone() {
                        let i = i as i64 + i_b;
                        let j = j as i64 + j_b;
                        if !(0..h_x as i64).contains(&i) || !(0..w_x as i64).contains(&j) {
                            continue 'outer;
                        }
                        x.insert((i as usize, j as usize));
                    }

                    if x == set_x {
                        println!("Yes");
                        return;
                    }
                }
            }
        }
    }
    println!("No");
}
