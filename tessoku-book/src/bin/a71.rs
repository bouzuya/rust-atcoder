use proconio::input;

fn main() {
    input! {
        n: usize,
        mut a: [usize; n],
        mut b: [usize; n],
    };
    a.sort();
    b.sort();
    b.reverse();

    let ans = a
        .into_iter()
        .zip(b.into_iter())
        .map(|(a_i, b_i)| a_i * b_i)
        .sum::<usize>();
    println!("{}", ans);
}
