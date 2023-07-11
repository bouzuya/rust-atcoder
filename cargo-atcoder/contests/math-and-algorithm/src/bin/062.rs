use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        k: usize,
        a: [Usize1; n],
    };
    if k <= n {
        let mut curr = 0;
        for _ in 0..k {
            curr = a[curr];
        }
        println!("{}", curr + 1);
    } else {
        let inf = 2 * n;
        let mut index = vec![inf; n];
        let mut curr = 0;
        for i in 0..n {
            if index[curr] != inf {
                let loop_range = index[curr]..i;
                let len = k - loop_range.start;
                for _ in 0..len % loop_range.len() {
                    curr = a[curr];
                }
                println!("{}", curr + 1);
                return;
            }
            index[curr] = i;
            curr = a[curr];
        }
        unreachable!();
    }
}
