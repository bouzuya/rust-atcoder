use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [i64; n],
    };
    let mut b = a.iter().copied().enumerate().collect::<Vec<(usize, i64)>>();
    b.sort_by_key(|&(_, a_i)| -a_i);
    let ans = b[1].0 + 1;
    println!("{}", ans);
}
