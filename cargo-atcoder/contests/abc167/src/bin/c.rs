use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        x: i64,
    };
    let mut c = vec![];
    let mut a = vec![];
    for _ in 0..n {
        input! {
            c_i: i64,
            a_i: [i64; m],
        };
        c.push(c_i);
        a.push(a_i);
    }
    let inf = 1_000_000_000_000_i64;
    let mut ans = inf;
    for bits in 0..2_usize.pow(n as u32) {
        let (sc, sa) = c
            .iter()
            .zip(a.iter())
            .zip((0..n).map(|i| (bits >> i) & 1 == 1))
            .filter(|&(_, b)| b)
            .map(|(p, _)| p)
            .fold((0, vec![0_i64; m]), |(sc, mut sa), (c_i, a_i)| {
                a_i.iter().enumerate().for_each(|(j, &a_ij)| sa[j] += a_ij);
                (sc + c_i, sa)
            });
        if sa.iter().all(|&sa_j| sa_j >= x) {
            ans = std::cmp::min(ans, sc);
        }
    }
    println!("{}", if ans == inf { -1 } else { ans });
}
