use proconio::input;

fn main() {
    input! {
        n: usize,
        ab: [(usize, usize); n],
    };
    let mut a = ab.iter().copied().map(|(a_i, _)| a_i).collect::<Vec<_>>();
    let mut b = ab.iter().copied().map(|(_, b_i)| b_i).collect::<Vec<_>>();
    a.sort();
    b.sort();
    let (bottom, top) = if n % 2 == 0 {
        (a[n / 2 - 1] + a[n / 2], b[n / 2 - 1] + b[n / 2])
    } else {
        (a[n / 2], b[n / 2])
    };
    let ans = top - bottom + 1;
    println!("{}", ans);
}
