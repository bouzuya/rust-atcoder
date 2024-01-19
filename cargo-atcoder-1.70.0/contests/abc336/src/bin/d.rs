use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    };

    let mut b = vec![1; n];
    let mut p = 0_usize;
    for (i, a_i) in a.iter().copied().enumerate() {
        p = (p + 1).min(a_i);
        b[i] = p;
    }

    let mut c = vec![1; n];
    let mut p = 0_usize;
    for (i, a_i) in a.iter().copied().enumerate().rev() {
        p = (p + 1).min(a_i);
        c[i] = p;
    }

    let ans = b
        .into_iter()
        .zip(c.into_iter())
        .map(|(b_i, c_i)| b_i.min(c_i))
        .max()
        .unwrap();
    println!("{}", ans);
}
