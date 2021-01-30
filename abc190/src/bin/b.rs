use proconio::input;

fn main() {
    input! {
        n: usize,
        s: i64,
        d: i64,
        xy: [(i64, i64); n],
    };
    for (x_i, y_i) in xy {
        if x_i >= s {
            continue;
        }
        if y_i <= d {
            continue;
        }
        println!("Yes");
        return;
    }
    println!("No");
}
