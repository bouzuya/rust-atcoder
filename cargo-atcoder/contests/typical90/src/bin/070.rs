use proconio::input;

fn main() {
    input! {
        n: usize,
        xy: [(i64, i64); n],
    };
    let med = |a: &mut Vec<i64>| -> i64 {
        a.sort();
        if n % 2 == 0 {
            (a[n / 2 - 1] + a[n / 2]) / 2
        } else {
            a[n / 2]
        }
    };
    let mut x = xy.iter().map(|(x_i, _)| *x_i).collect::<Vec<i64>>();
    let mut y = xy.iter().map(|(_, y_i)| *y_i).collect::<Vec<i64>>();
    let mx = med(&mut x);
    let my = med(&mut y);
    let mut res = 0_i64;
    for (x_i, y_i) in xy {
        res += (x_i - mx).abs() + (y_i - my).abs();
    }
    let ans = res;
    println!("{}", ans);
}
