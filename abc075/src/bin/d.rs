use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        xy: [(i64, i64); n],
    };

    let mut x = xy.iter().map(|&(x, _)| x).collect::<Vec<i64>>();
    x.sort();
    let mut y = xy.iter().map(|&(_, y)| y).collect::<Vec<i64>>();
    y.sort();
    let mut ans = (x[n - 1] - x[0]) * (y[n - 1] - y[0]);

    for (i_x, &x_l) in x.iter().enumerate() {
        for &x_r in x.iter().skip(i_x + 1) {
            for (i_y, &y_l) in y.iter().enumerate() {
                for &y_r in y.iter().skip(i_y + 1) {
                    if xy
                        .iter()
                        .filter(|&&(x_i, y_i)| x_l <= x_i && x_i <= x_r && y_l <= y_i && y_i <= y_r)
                        .count()
                        >= k
                    {
                        ans = std::cmp::min(ans, (x_r - x_l) * (y_r - y_l));
                    }
                }
            }
        }
    }

    println!("{}", ans);
}
