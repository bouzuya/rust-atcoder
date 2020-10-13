use proconio::input;
use proconio::marker::Usize1;

fn adjacency_list(n: usize, uv: &Vec<(usize, usize)>) -> Vec<Vec<(usize, usize)>> {
    let mut e = vec![vec![]; n];
    for (i, &(u, v)) in uv.iter().enumerate() {
        e[u].push((v, i));
        e[v].push((u, i));
    }
    e
}

fn main() {
    input! {
        n: usize,
        ab: [(Usize1, Usize1); n - 1]
    };

    let e = adjacency_list(n, &ab);
    let mut color_count = 0;
    let mut color = vec![n; n - 1];
    let mut q = std::collections::VecDeque::new();
    q.push_back((0, 0, n));
    while let Some((u, p, pc)) = q.pop_front() {
        let mut c = 0;
        for &(v, i_e) in e[u].iter() {
            if v == p {
                continue;
            }
            if color[i_e] != n {
                continue;
            }
            if c == pc {
                c += 1;
            }
            color[i_e] = c;
            q.push_back((v, u, c));
            c += 1;
            color_count = std::cmp::max(c, color_count);
        }
    }
    println!("{}", color_count);
    for c in color.iter() {
        println!("{}", c + 1);
    }
}
