use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [i64; n],
    };
    let avg = a.iter().sum::<i64>();
    let mut b = a
        .iter()
        .enumerate()
        .map(|(i, &a_i)| ((avg - n as i64 * a_i).abs(), i))
        .collect::<Vec<(i64, usize)>>();
    b.sort();
    let ans = b[0].1;
    println!("{}", ans);
}
