use proconio::{input, marker::Usize1};

fn dfs(n: usize, t: usize, ng: &[Vec<bool>], count: &mut usize, a: &mut Vec<usize>) {
    if a.len() == n {
        if a.iter().copied().max().unwrap() + 1 != t {
            return;
        }
        for j in 0..n {
            for k in j + 1..n {
                if a[j] == a[k] && ng[j][k] {
                    return;
                }
            }
        }
        *count += 1;
        return;
    }

    for j in 0..=(t - 1).min(a.iter().copied().max().unwrap() + 1) {
        a.push(j);
        dfs(n, t, ng, count, a);
        a.pop();
    }
}

fn main() {
    input! {
        n: usize,
        t: usize,
        m: usize,
        ab: [(Usize1, Usize1); m],
    };

    let mut ng = vec![vec![false; n]; n];
    for (a, b) in ab {
        ng[a][b] = true;
        ng[b][a] = true;
    }

    let mut count = 0_usize;
    dfs(n, t, &ng, &mut count, &mut vec![0]);

    let ans = count;
    println!("{}", ans);
}
