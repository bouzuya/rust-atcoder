use proconio::input;

fn dfs(a: &[usize]) -> usize {
    if a.is_empty() {
        return 1;
    }
    let mut count = 0_usize;
    let (i, a_i) = (0, a[0]);
    for j in 1..a.len() {
        let a_j = a[j];
        for k in j + 1..a.len() {
            let a_k = a[k];
            let (a_i, a_j) = (a_i.min(a_j), a_i.max(a_j));
            let (a_j, a_k) = (a_j.min(a_k), a_j.max(a_k));
            if a_i + a_j > a_k {
                let b = a
                    .iter()
                    .copied()
                    .enumerate()
                    .filter(|(l, _)| l != &i && l != &j && l != &k)
                    .map(|(_, x)| x)
                    .collect::<Vec<usize>>();
                count += dfs(&b);
            }
        }
    }
    count
}

fn main() {
    input! {
        n: usize,
        a: [usize; 3 * n],
    };

    let ans = dfs(&a);
    println!("{}", ans);
}
