use std::collections::HashSet;

use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        m: usize,
    };

    let mut x = vec![];
    for _ in 0..m {
        input! {
            k_i: usize,
            x_i: [Usize1; k_i]
        }
        x.push(x_i);
    }

    let mut set = HashSet::new();
    for x_i in x {
        for j1 in 0..x_i.len() {
            for j2 in j1 + 1..x_i.len() {
                set.insert((x_i[j1], x_i[j2]));
            }
        }
    }

    for i in 0..n {
        for j in i + 1..n {
            if !set.contains(&(i, j)) {
                println!("No");
                return;
            }
        }
    }
    println!("Yes");
}
