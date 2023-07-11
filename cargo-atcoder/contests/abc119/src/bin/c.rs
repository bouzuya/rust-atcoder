use proconio::input;

fn dfs(ans: &mut usize, x: &mut Vec<usize>, n: usize, abc: &[usize], l: &[usize], i: usize) {
    if x.len() == n {
        let mut ls = vec![vec![]; 3];
        let mut sum = vec![0; 3];
        for (i, x_i) in x.iter().copied().enumerate() {
            if x_i < ls.len() {
                ls[x_i].push(l[i]);
                sum[x_i] += l[i];
            }
        }
        let mut a = 0_usize;
        for i in 0..3 {
            if ls[i].is_empty() {
                return;
            }
            a += if abc[i] >= sum[i] {
                abc[i] - sum[i]
            } else {
                sum[i] - abc[i]
            } + (ls[i].len() - 1) * 10;
        }
        *ans = (*ans).min(a);
        return;
    }
    for j in 0..4 {
        x.push(j);
        dfs(ans, x, n, abc, l, i + 1);
        x.pop();
    }
}

fn main() {
    input! {
        n: usize,
        abc: [usize; 3],
        l: [usize; n],
    };

    let mut ans = 1_usize << 60;
    let mut x = vec![];
    dfs(&mut ans, &mut x, n, &abc, &l, 0);
    println!("{}", ans);
}
