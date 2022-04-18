use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        xy: [(i64, i64); n],
    };
    if k == 1 {
        println!("Infinity");
        return;
    }

    let mut used = vec![vec![false; n]; n];
    let mut ans = 0_usize;
    for i in 0..n {
        for j in i + 1..n {
            if used[i][j] {
                continue;
            }
            let (x_i, y_i) = xy[i];
            let (x_j, y_j) = xy[j];
            let mut ps = vec![i, j];
            for k in j + 1..n {
                let (x_k, y_k) = xy[k];
                if (y_k - y_i) * (x_j - x_i) == (y_j - y_i) * (x_k - x_i) {
                    ps.push(k);
                }
            }
            if ps.len() >= k {
                ans += 1;
            }
            for i in ps.iter().copied() {
                for j in ps.iter().copied() {
                    used[i][j] = true;
                }
            }
        }
    }

    println!("{}", ans);
}
