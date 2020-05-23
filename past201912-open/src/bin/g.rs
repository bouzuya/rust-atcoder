use proconio::input;

fn sum(a: &Vec<Vec<i64>>, g: &Vec<Vec<usize>>) -> i64 {
    let mut res = 0_i64;
    for g_i in g.iter() {
        for i in 0..g_i.len() {
            for j in i + 1..g_i.len() {
                res += a[g_i[i]][g_i[j]];
            }
        }
    }
    res
}

fn dfs(inf: i64, a: &Vec<Vec<i64>>, g: &mut Vec<Vec<usize>>, n: usize, i: usize) -> i64 {
    if i == n {
        return sum(a, g);
    }

    let mut res = inf;
    for g_i in 0..g.len() {
        g[g_i].push(i);
        res = std::cmp::max(res, dfs(res, a, g, n, i + 1));
        g[g_i].pop();
    }
    res
}

fn main() {
    input! { n: usize };
    let mut a = vec![vec![0_i64; n]; n];
    for i in 0..n - 1 {
        for j in i + 1..n {
            input! { a_ij: i64 };
            a[i][j] = a_ij;
            a[j][i] = a_ij;
        }
    }

    let inf = -1_000_000_000_000_000;
    let mut ans = inf;
    ans = dfs(ans, &a, &mut vec![vec![]; 1], n, 0);
    ans = dfs(ans, &a, &mut vec![vec![]; 2], n, 0);
    ans = dfs(ans, &a, &mut vec![vec![]; 3], n, 0);

    println!("{}", ans);
}
