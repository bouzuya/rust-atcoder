use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [i64; n],
    };
    let f = |a: &[i64]| -> i64 {
        let mut res = 0_i64;
        let mut min = 0_i64;
        let mut cur = 0_i64;
        for a_i in a.iter().copied() {
            cur += a_i;
            res = res.max(cur - min);
            min = min.min(cur);
        }
        res
    };
    let b = a
        .into_iter()
        .map(|a_i| match a_i {
            0 => 1,
            1 => -1,
            _ => unreachable!(),
        })
        .collect::<Vec<i64>>();
    let c = b.iter().copied().map(|b_i| -b_i).collect::<Vec<i64>>();
    let ans = f(&b) + f(&c) + 1;
    println!("{}", ans);
}
