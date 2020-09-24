use proconio::input;

fn main() {
    input! {
        n: usize,
        x: [i64; n],
    };
    let mut a = x
        .iter()
        .enumerate()
        .map(|(i, &x_i)| (x_i, i))
        .collect::<Vec<(i64, usize)>>();
    a.sort();
    let m1 = a[n / 2 - 1].0;
    let m2 = a[n / 2].0;
    let mut b = a
        .iter()
        .enumerate()
        .map(|(j, &(_, i))| (i, if j < n / 2 { m2 } else { m1 }))
        .collect::<Vec<(usize, i64)>>();
    b.sort();
    for &(_, m_i) in b.iter() {
        println!("{}", m_i);
    }
}
