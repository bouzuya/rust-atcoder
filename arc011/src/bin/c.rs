use proconio::input;
use proconio::marker::Chars;

fn shortest_path_spanning_tree(e: &Vec<Vec<usize>>, s: usize) -> (Vec<usize>, Vec<usize>) {
    let n = e.len();
    let inf = n;
    let mut d = vec![inf; n];
    let mut t = vec![n; n];
    let mut pq = std::collections::BinaryHeap::new();
    d[s] = 0;
    t[s] = s;
    pq.push(std::cmp::Reverse((0, s)));
    while let Some(std::cmp::Reverse((c_u, u))) = pq.pop() {
        for &v in e[u].iter() {
            let c_v = c_u + 1;
            if c_v < d[v] {
                d[v] = c_v;
                t[v] = u;
                pq.push(std::cmp::Reverse((c_v, v)));
            }
        }
    }
    (d, t)
}

fn main() {
    input! {
        first: Chars,
        last: Chars,
        n: usize,
        s: [Chars; n],
    };
    if first == last {
        println!("0");
        println!("{}", first.iter().collect::<String>());
        println!("{}", last.iter().collect::<String>());
        return;
    }
    let mut e = vec![vec![]; n + 2];
    for (i, s_i) in s.iter().enumerate() {
        for (j, s_j) in s.iter().enumerate() {
            if i <= j {
                continue;
            }
            let mut c = 0;
            for (s_i_k, s_j_k) in s_i.iter().zip(s_j.iter()) {
                if s_i_k == s_j_k {
                    c += 1;
                }
            }
            if c == s_i.len() - 1 {
                e[i].push(j);
                e[j].push(i);
            }
        }
    }
    // e[n]: first
    for (i, s_i) in s.iter().enumerate() {
        let mut c = 0;
        for (s_i_k, s_j_k) in s_i.iter().zip(first.iter()) {
            if s_i_k == s_j_k {
                c += 1;
            }
        }
        if c == s_i.len() - 1 {
            e[i].push(n);
            e[n].push(i);
        }
    }
    // e[n + 1]: last
    for (i, s_i) in s.iter().enumerate() {
        let mut c = 0;
        for (s_i_k, s_j_k) in s_i.iter().zip(last.iter()) {
            if s_i_k == s_j_k {
                c += 1;
            }
        }
        if c == s_i.len() - 1 {
            e[i].push(n + 1);
            e[n + 1].push(i);
        }
    }
    let mut c = 0;
    for (s_i_k, s_j_k) in first.iter().zip(last.iter()) {
        if s_i_k == s_j_k {
            c += 1;
        }
    }
    if c == first.len() - 1 {
        e[n].push(n + 1);
        e[n + 1].push(n);
    }

    let (d, t) = shortest_path_spanning_tree(&e, n);
    if d[n + 1] == n + 2 {
        println!("-1");
        return;
    }
    let mut ans = vec![];
    let mut c = n + 1;
    while t[c] != n {
        c = t[c];
        ans.push(&s[c]);
    }
    ans.reverse();
    println!("{}", ans.len());
    println!("{}", first.iter().collect::<String>());
    for ans_i in ans.iter() {
        println!("{}", ans_i.iter().collect::<String>());
    }
    println!("{}", last.iter().collect::<String>());
}
