use proconio::input;

fn main() {
    input! {
        n: usize,
        w: [i64; n],
    };
    let ans = (0..n)
        .map(|i| {
            let s1 = w[0..i].iter().sum::<i64>();
            let s2 = w[i..].iter().sum::<i64>();
            (s1 - s2).abs()
        })
        .min()
        .unwrap();
    println!("{}", ans);
}
