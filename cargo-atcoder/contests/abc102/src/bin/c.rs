use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [i64; n],
    };
    let mut c = vec![];
    for (i, a_i) in a.iter().copied().enumerate() {
        c.push(a_i - (i + 1) as i64);
    }
    c.sort();

    let x = if n % 2 == 0 {
        (c[n / 2 - 1] + c[n / 2]) / 2
    } else {
        c[n / 2]
    };
    let ans = c.iter().copied().map(|c_i| (c_i - x).abs()).sum::<i64>();
    println!("{}", ans);
}
