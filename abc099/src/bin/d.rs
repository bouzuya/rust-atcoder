use proconio::{input, marker::Usize1};

fn dfs(ans: &mut usize, choice: &mut Vec<usize>, sum: &[Vec<usize>], c: usize) {
    if choice.len() == 3 {
        let mut x = 0_usize;
        for (i, choice_i) in choice.iter().copied().enumerate() {
            x += sum[choice_i][i];
        }
        *ans = (*ans).min(x);
        return;
    }

    for i in 0..c {
        if choice.contains(&i) {
            continue;
        }
        choice.push(i);
        dfs(ans, choice, sum, c);
        choice.pop();
    }
}

fn main() {
    input! {
        n: usize,
        capital_c: usize,
        d: [[usize; capital_c]; capital_c],
        c: [[Usize1; n]; n]
    };
    let mut sum = vec![vec![0_usize; 3]; capital_c];
    for color in 0..capital_c {
        for i in 0..n {
            for j in 0..n {
                sum[color][(i + j) % 3] += d[c[i][j]][color];
            }
        }
    }

    let mut ans = 1 << 60;
    let mut choice = vec![];
    dfs(&mut ans, &mut choice, &sum, capital_c);
    println!("{}", ans);
}
