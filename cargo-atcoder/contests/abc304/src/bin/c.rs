use std::collections::VecDeque;

use proconio::input;

fn main() {
    input! {
        n: usize,
        d: i64,
        xy: [(i64, i64); n],
    };

    let mut checked = vec![false; n];
    let mut q = VecDeque::new();
    checked[0] = true;
    q.push_back(0);
    while let Some(i) = q.pop_front() {
        let (x_i, y_i) = xy[i];
        for (j, (x_j, y_j)) in xy.iter().copied().enumerate() {
            if checked[j] {
                continue;
            }

            if (x_i - x_j).pow(2) + (y_i - y_j).pow(2) <= d.pow(2) {
                checked[j] = true;
                q.push_back(j);
            }
        }
    }

    for b in checked {
        println!("{}", if b { "Yes" } else { "No" });
    }
}
