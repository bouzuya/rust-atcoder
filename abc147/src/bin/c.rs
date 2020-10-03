use proconio::input;
use proconio::marker::Usize1;

fn main() {
    input! {
        n: usize,
    };
    // 0: 正直者, 1: 不親切, 2: 未証言
    let mut a = vec![vec![2; n]; n];
    for i in 0..n {
        input! {
            a_i: usize,
            xy: [(Usize1, i64); a_i]
        };
        for &(x_i, y_i) in xy.iter() {
            a[i][x_i] = y_i;
        }
    }
    let mut max_count = 0;
    for bits in 0..1 << n {
        let is = (0..n).map(|i| (bits >> i) & 1 == 1).collect::<Vec<bool>>();
        if a.iter().zip(is.iter()).all(|(a_i, is_i)| {
            !is_i
                || a_i.iter().zip(is.iter()).all(|(a_ij, &is_j)| match a_ij {
                    0 => is_j == false,
                    1 => is_j == true,
                    2 => true,
                    _ => unreachable!(),
                })
        }) {
            max_count = std::cmp::max(max_count, is.iter().filter(|&&is_i| is_i).count());
        }
    }
    let ans = max_count;
    println!("{}", ans);
}
