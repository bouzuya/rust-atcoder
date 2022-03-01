use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        k: usize,
        a: [Usize1; n],
    };
    if k <= 200_000 {
        let mut curr = 0;
        for _ in 0..k {
            curr = a[curr];
        }
        println!("{}", curr + 1);
    } else {
        let inf = 2 * n;
        let mut pos = vec![inf; n];
        let mut curr = 0;
        for i in 0..n {
            if pos[curr] != inf {
                let start = curr;
                let size = i + 1 - pos[start];
                let mut curr = start;
                for _ in 0..(k + 1 - pos[start]) % size {
                    curr = a[curr];
                }
                println!("{}", curr + 1);
                return;
            }
            pos[curr] = i + 1;
            curr = a[curr];
        }
        unreachable!();
    }
}
