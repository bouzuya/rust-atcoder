use proconio::input;
use superslice::*;

fn main() {
    input! {
        n: usize,
        m: i64,
        mut a: [i64; n],
    };

    a.sort();

    let c = std::iter::once(0_i64)
        .chain(a.iter().scan(0_i64, |acc, &a_i| {
            *acc += a_i;
            Some(*acc)
        }))
        .collect::<Vec<i64>>();

    let mut ans = 0_i64;
    let mut ok = 1;
    let mut ng = 1_000_000_000_000;
    while ng - ok > 1 {
        let x = ok + (ng - ok) / 2;

        let mut m_x = 0_i64;
        let mut s_x = 0_i64;
        for &a_i in a.iter() {
            let x_l = a_i;
            let x_r = x - x_l;
            let i = a[..].lower_bound(&x_r); // 右手として選択できる最小の位置を二分探索
            let p = (n - i) as i64; // 作成できる手の組数
            m_x += p;
            s_x += (x_l * p) + (c[n] - c[i]);
        }
        if m_x >= m {
            ok = x;
            ans = s_x - (m_x - m) * x; // x が 2 個以上あるとき足しすぎていることがある
        } else {
            ng = x;
        }
    }
    println!("{}", ans);
}
