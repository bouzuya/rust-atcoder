use proconio::input;

fn main() {
    input! {
        n: usize,
        mut a: [i64; n],
        mut b: [i64; n],
    };
    a.sort();
    b.sort();
    let mut d = 0;
    for (a_i, b_i) in a.iter().zip(b.iter()) {
        d += (a_i - b_i).abs();
    }
    let ans = d;
    println!("{}", ans);
}
