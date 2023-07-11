use proconio::input;

fn dfs(ans: &mut Vec<usize>, n: usize, m: usize) {
    if ans.len() == n {
        for (i, a_i) in ans.iter().copied().enumerate() {
            print!("{}{}", a_i, if i == n - 1 { "\n" } else { " " });
        }
        return;
    }

    if ans.is_empty() {
        for x in 1..=m {
            ans.push(x);
            dfs(ans, n, m);
            ans.pop();
        }
    } else {
        let last = ans[ans.len() - 1];
        for x in last + 1..=m {
            ans.push(x);
            dfs(ans, n, m);
            ans.pop();
        }
    }
}

fn main() {
    input! {
        n: usize,
        m: usize,
    };
    let mut ans = vec![];
    dfs(&mut ans, n, m);
}
