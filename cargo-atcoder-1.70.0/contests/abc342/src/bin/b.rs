use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        p: [Usize1; n],
        q: usize,
        ab: [(Usize1, Usize1); q]
    };

    let mut ip = p
        .iter()
        .copied()
        .enumerate()
        .collect::<Vec<(usize, usize)>>();
    ip.sort_by_key(|&(_, v)| v);
    for (a, b) in ab {
        println!("{}", if ip[a].0 < ip[b].0 { a + 1 } else { b + 1 });
    }
}
