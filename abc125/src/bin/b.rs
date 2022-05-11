use proconio::input;

fn main() {
    input! {
        n: usize,
        v: [usize; n],
        c: [usize; n],
    };
    let ans = v
        .iter()
        .copied()
        .zip(c.iter().copied())
        .filter(|(v_i, c_i)| v_i > c_i)
        .map(|(v_i, c_i)| v_i - c_i)
        .sum::<usize>();
    println!("{}", ans);
}
