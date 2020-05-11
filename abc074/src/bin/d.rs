use proconio::input;

fn f(n: usize, e: &Vec<Vec<(usize, usize)>>, u: usize, v: usize) -> Option<usize> {
    let mut c = vec![None; n];
    let mut pq = std::collections::BinaryHeap::new();
    pq.push(std::cmp::Reverse((0, u)));
    while let Some(std::cmp::Reverse((c_u, u))) = pq.pop() {
        if c[u].is_some() && Some(c_u) > c[u] {
            continue;
        }
        for &(v, c_v1) in e[u].iter() {
            let c_v2 = c_u + c_v1;
            if c[v].is_none() || Some(c_v2) < c[v] {
                c[v] = Some(c_v2);
                pq.push(std::cmp::Reverse((c_v2, v)));
            }
        }
    }
    c[v]
}

fn main() {
    input! {
        n: usize,
        a: [[usize; n]; n],
    };

    let mut e_a = vec![];
    for u in 0..n {
        for v in u + 1..n {
            e_a.push((a[u][v], u, v));
        }
    }
    e_a.sort();

    let mut ans = 0;
    let mut e = vec![vec![]; n];
    for &(a_i, u, v) in e_a.iter() {
        match f(n, &e, u, v) {
            None => {
                e[u].push((v, a_i));
                e[v].push((u, a_i));
                ans += a_i;
            }
            Some(d) => {
                if d < a_i {
                    println!("-1");
                    return;
                }
                // d == a_i はほかから辿れるため不要な辺
                if d > a_i {
                    e[u].push((v, a_i));
                    e[v].push((u, a_i));
                    ans += a_i;
                }
            }
        }
    }
    println!("{}", ans);
}
