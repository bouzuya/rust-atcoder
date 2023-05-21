use im_rc::HashSet;
use proconio::{input, marker::Usize1};

fn dfs(
    ans: &mut Vec<bool>,
    parents: &mut HashSet<usize>,
    queries: &[Vec<(usize, usize)>],
    edges: &[Vec<usize>],
    u: usize,
) {
    for (b, ans_index) in queries[u].iter().copied() {
        ans[ans_index] = parents.contains(&b);
    }
    parents.insert(u);
    for v in edges[u].iter().copied() {
        dfs(ans, parents, queries, edges, v);
    }
    parents.remove(&u);
}

fn main() {
    input! {
        n: usize,
        p: [i64; n],
        q: usize,
        ab: [(Usize1, Usize1); q],
    }

    let mut root = 0;
    let mut edges = vec![vec![]; n];
    for (i, p_i) in p.iter().copied().enumerate() {
        if p_i == -1 {
            root = i;
            continue;
        }
        let parent = (p_i - 1) as usize;
        edges[parent].push(i);
    }

    let mut queries = vec![vec![]; n];
    for (i, (a, b)) in ab.iter().copied().enumerate() {
        queries[a].push((b, i));
    }

    let mut ans = vec![false; q];
    let mut parents = HashSet::new();
    dfs(&mut ans, &mut parents, &queries, &edges, root);

    for a in ans {
        println!("{}", if a { "Yes" } else { "No" });
    }
}
