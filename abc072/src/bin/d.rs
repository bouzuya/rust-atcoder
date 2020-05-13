use proconio::input;
use proconio::marker::Usize1;

fn f(m: usize, n: usize) -> usize {
    if m > n {
        m - n
    } else {
        n - m
    }
}

fn main() {
    input! {
        n: usize,
        p: [Usize1; n],
    };
    let mut q = p
        .iter()
        .enumerate()
        .map(|(i, &p_i)| f(i, p_i))
        .collect::<Vec<usize>>();
    let mut ans = 0;
    for i in 0..n - 1 {
        if q[i] == 0 {
            let p_i = p[i];
            let p_j = p[i + 1];
            q[i] = f(i, p_j);
            q[i + 1] = f(i + 1, p_i);
            ans += 1;
        }
    }
    for i in (0..n - 1).rev() {
        if q[i + 1] == 0 {
            let p_i = p[i];
            let p_j = p[i + 1];
            q[i] = f(i, p_j);
            q[i + 1] = f(i + 1, p_i);
            ans += 1;
        }
    }
    println!("{}", ans);
}
