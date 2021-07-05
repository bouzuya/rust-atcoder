use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        k: usize,
        a: [Usize1; n],
    };
    let mut a = a
        .iter()
        .enumerate()
        .map(|(i, &a_i)| (i, a_i, k / n))
        .collect::<Vec<(usize, usize, usize)>>();
    a.sort_by_key(|(_, a_i, _)| *a_i);
    let mut k = k % n;
    for (_, _, v) in a.iter_mut() {
        *v += if k > 0 { 1 } else { 0 };
        k = k.saturating_sub(1);
    }
    a.sort_by_key(|(i, _, _)| *i);
    for (_, _, v) in a {
        println!("{}", v);
    }
}
