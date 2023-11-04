use std::collections::HashSet;

use proconio::input;

fn main() {
    input! {
        a: [[usize; 9]; 9],
    };

    for i in 0..9 {
        if (0..9).map(|j| a[i][j]).collect::<HashSet<usize>>().len() != 9 {
            println!("No");
            return;
        }
    }
    for j in 0..9 {
        if (0..9).map(|i| a[i][j]).collect::<HashSet<usize>>().len() != 9 {
            println!("No");
            return;
        }
    }

    for bi in 0..3 {
        for bj in 0..3 {
            let mut b = HashSet::new();
            for i in 0..3 {
                for j in 0..3 {
                    b.insert(a[bi * 3 + i][bj * 3 + j]);
                }
            }
            if b.len() != 9 {
                println!("No");
                return;
            }
        }
    }

    println!("Yes");
}
