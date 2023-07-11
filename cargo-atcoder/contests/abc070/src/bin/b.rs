use proconio::input;

fn main() {
    input! {
        a: usize,
        b: usize,
        c: usize,
        d: usize,
    };
    let mut p_a = vec![false; 200 + 1];
    for i in a..=b - 1 {
        p_a[i] = true;
    }
    let mut p_b = vec![false; 200 + 1];
    for i in c..=d - 1 {
        p_b[i] = true;
    }

    let ans = p_a
        .into_iter()
        .zip(p_b.into_iter())
        .filter(|&(a, b)| a && b)
        .count();
    println!("{}", ans);
}
