use proconio::{input, marker::Usize1};

fn adjacency_list(n: usize, uv: &[(usize, usize)]) -> Vec<Vec<usize>> {
    let mut e = vec![vec![]; n];
    for &(u, v) in uv.iter() {
        e[u].push(v);
        e[v].push(u);
    }
    e
}

fn dfs(
    es: &[Vec<usize>],
    cs: &[u64],
    i: &mut usize,
    s: &mut u64,
    vs: &mut [u64],
    u: usize,
    p: usize,
) {
    for &v in es[u].iter() {
        if v == p {
            continue;
        }
        dfs(es, cs, i, s, vs, v, u);
        *s += cs[*i];
        vs[v] = cs[*i];
        *i += 1;
    }
}

fn main() {
    input! {
        n: usize,
        ab: [(Usize1, Usize1); n - 1],
        mut c: [u64; n],
    };
    c.sort();

    let e = adjacency_list(n, &ab);
    let mut i = 0_usize;
    let mut m = 0_u64;
    let mut v = vec![0_u64; n];
    dfs(&e, &c, &mut i, &mut m, &mut v, 0, 0);
    v[0] = c[i];

    println!("{}", m);
    for (i, &v_i) in v.iter().enumerate() {
        print!("{}{}", v_i, if i == n - 1 { "\n" } else { " " });
    }
}
