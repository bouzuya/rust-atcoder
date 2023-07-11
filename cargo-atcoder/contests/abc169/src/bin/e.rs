use proconio::input;

fn main() {
    input! {
        n: usize,
        ab: [(i64, i64); n],
    };
    let mut a = ab.iter().map(|&(a_i, _)| a_i).collect::<Vec<_>>();
    let mut b = ab.iter().map(|&(_, b_i)| b_i).collect::<Vec<_>>();
    a.sort();
    b.sort();
    let ans = if n % 2 == 0 {
        let lx2 = a[n / 2 - 1] + a[n / 2];
        let rx2 = b[n / 2 - 1] + b[n / 2];
        rx2 - lx2 + 1
    } else {
        let l = a[n / 2];
        let r = b[n / 2];
        r - l + 1
    };
    println!("{}", ans);
}
