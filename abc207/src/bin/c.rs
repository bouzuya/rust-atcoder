use proconio::input;

fn main() {
    input! {
        n: usize,
        tlr: [(usize, usize, usize); n],
    };
    let mut count = 0;
    for i in 0..n {
        for j in i + 1..n {
            let f = |(t, l, r)| match t {
                1 => (2 * l, 2 * r),         // [l, r]
                2 => (2 * l, 2 * r - 1),     // [l, r)
                3 => (2 * l + 1, 2 * r),     // (l, r]
                4 => (2 * l + 1, 2 * r - 1), // (l, r)
                _ => unreachable!(),
            };
            let (l_i, r_i) = f(tlr[i]);
            let (l_j, r_j) = f(tlr[j]);
            if !(r_j < l_i || r_i < l_j) {
                count += 1;
            }
        }
    }
    let ans = count;
    println!("{}", ans);
}
