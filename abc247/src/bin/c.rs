use proconio::input;

fn dfs(ans: &mut Vec<Vec<usize>>, n: usize, i: usize) {
    if i == n {
        return;
    }
    ans.push(
        ans[i]
            .iter()
            .copied()
            .chain(std::iter::once(i + 1))
            .chain(ans[i].iter().copied())
            .collect::<Vec<usize>>(),
    );
    dfs(ans, n, i + 1);
}

fn main() {
    input! {
        n: usize,
    };
    let mut ans = vec![vec![], vec![1]];
    dfs(&mut ans, n, 1);
    for a in ans[n].iter().copied() {
        println!("{}", a);
    }
}
