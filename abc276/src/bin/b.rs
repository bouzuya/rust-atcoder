use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        m: usize,
        ab: [(Usize1, Usize1); m],
    };
    let mut edges = vec![vec![]; n];
    for (u, v) in ab.iter().copied() {
        edges[u].push(v);
        edges[v].push(u);
    }

    for u in 0..n {
        let mut e_u = edges[u].iter().copied().collect::<Vec<usize>>();
        e_u.sort();
        print!("{}", e_u.len());
        if !e_u.is_empty() {
            print!(" ");
            for (i, v) in e_u.iter().copied().enumerate() {
                print!("{}{}", v + 1, if i == e_u.len() - 1 { '\n' } else { ' ' });
            }
        } else {
            println!();
        }
    }
}
