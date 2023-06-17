use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        a: [Usize1; 3 * n],
    };
    let mut indexes = vec![vec![]; n];
    for (i, a_i) in a.iter().copied().enumerate() {
        indexes[a_i].push(i);
    }

    let mut b = indexes
        .iter()
        .enumerate()
        .map(|(i, index)| (i, index[1]))
        .collect::<Vec<(usize, usize)>>();
    b.sort_by_key(|&(_, index)| index);
    for x in b.into_iter().map(|(i, _)| i + 1) {
        println!("{}", x);
    }
}
