use proconio::input;

fn f(a: u64, b: u64) -> u64 {
    if a >= b {
        a - b
    } else {
        b - a
    }
}

fn dfs(n: usize, abc: &Vec<u64>, l: &Vec<u64>, i: usize, a: u64, b: u64, c: u64, mp: u64) -> u64 {
    if i == n {
        if a == 0 || b == 0 || c == 0 {
            return 3_000;
        }
        return mp + f(a, abc[0]) - 10 + f(b, abc[1]) - 10 + f(c, abc[2]) - 10;
    }
    *vec![
        dfs(n, abc, l, i + 1, a + l[i], b, c, mp + 10),
        dfs(n, abc, l, i + 1, a, b + l[i], c, mp + 10),
        dfs(n, abc, l, i + 1, a, b, c + l[i], mp + 10),
        dfs(n, abc, l, i + 1, a, b, c, mp),
    ]
    .iter()
    .min()
    .unwrap()
}

fn main() {
    input! {
        n: usize,
        abc: [u64; 3],
        l: [u64; n],
    };
    let ans = dfs(n, &abc, &l, 0, 0, 0, 0, 0);
    println!("{}", ans);
}
