use proconio::input;
use proconio::marker::Chars;

fn dfs(p: &mut Vec<bool>, n: usize, d: &Vec<u64>) -> u64 {
    if p.len() == n - 1 {
        let mut xs = vec![];
        xs.push(d[0]);
        for (&p_i, &d_i) in p.iter().zip(d.iter().skip(1)) {
            if p_i {
                xs.push(d_i);
            } else {
                let n_x = xs.len();
                let x = xs[n_x - 1];
                xs[n_x - 1] = x * 10 + d_i;
            }
        }
        return xs.iter().sum::<u64>();
    }
    let mut sum = 0;
    let b = vec![false, true];
    for &b_i in b.iter() {
        p.push(b_i);
        sum += dfs(p, n, d);
        p.pop();
    }
    sum
}

fn main() {
    input! {
        s: Chars
    };
    let d = s
        .iter()
        .map(|&c| c.to_digit(10).unwrap() as u64)
        .collect::<Vec<u64>>();
    let n = d.len();
    let ans = dfs(&mut vec![], n, &d);
    println!("{}", ans);
}
