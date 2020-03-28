use proconio::input;
use proconio::marker::Usize1;
use std::collections::VecDeque;

fn main() {
    input! {
        n: usize,
        x: Usize1,
        y: Usize1,
    };

    let mut evv = vec![vec![]; n];
    for i in 0..n - 1 {
        evv[i].push(i + 1);
        evv[i + 1].push(i);
    }
    evv[x].push(y);
    evv[y].push(x);

    let mut ans = vec![0_usize; n]; // ans[d] = count;
    for i in 0..n {
        let inf = n + 1;
        let mut dv = vec![inf; n]; // dv[i] = d;
        let mut q = VecDeque::new();
        q.push_back((i, 0));
        while let Some((i, d)) = q.pop_front() {
            if dv[i] != inf {
                continue;
            }
            dv[i] = d;
            for &j in evv[i].iter() {
                q.push_back((j, d + 1));
            }
        }
        for d in dv {
            ans[d] += 1;
        }
    }
    for a in 1..n {
        println!("{}", ans[a] / 2);
    }
}
