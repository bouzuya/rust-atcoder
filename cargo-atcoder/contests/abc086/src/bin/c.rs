use proconio::input;

fn main() {
    input! {
        n: usize,
        txy: [(i64, i64, i64); n],
    };
    let mut p = (0, 0, 0);
    for (t_i, x_i, y_i) in txy {
        let (t_p, x_p, y_p) = p;
        let dt = t_i - t_p;
        if (t_i % 2 != (x_i + y_i) % 2)
            || (((x_i + y_i) - (x_p + y_p)).abs() > dt)
            || ((x_i - x_p).abs() > dt)
            || ((y_i - y_p).abs() > dt)
        {
            println!("No");
            return;
        }
        p = (t_i, x_i, y_i);
    }
    println!("Yes");
}
