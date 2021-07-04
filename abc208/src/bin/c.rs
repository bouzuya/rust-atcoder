use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        k: usize,
        a: [Usize1; n],
    };
    let b = k / n;

    let mut a = a
        .iter()
        .enumerate()
        .map(|(i, &a_i)| (i, a_i))
        .collect::<Vec<(usize, usize)>>();
    a.sort_by_key(|(_, a_i)| *a_i);
    let mut c = vec![0; n];
    let mut k = k % n;
    for &(i, _) in a.iter() {
        c[i] = b + if k > 0 { 1 } else { 0 };
        k = k.saturating_sub(1);
    }
    for c_i in c {
        println!("{}", c_i);
    }
}
