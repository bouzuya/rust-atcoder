use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        m: usize,
        t: i64,
        a: [i64; n - 1],
        xy: [(Usize1, i64); m],
    };

    let mut bonus = vec![0; n];
    for (x, y) in xy {
        bonus[x] = y;
    }

    let mut sum = t;
    for i in 0..n - 1 {
        sum += bonus[i];
        sum -= a[i];
        if sum <= 0 && i != n - 1 {
            println!("No");
            return;
        }
    }

    println!("Yes");
}
