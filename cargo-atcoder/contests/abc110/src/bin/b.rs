use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        capital_x: i64,
        capital_y: i64,
        x: [i64; n],
        y: [i64; m],
    };
    for z in capital_x + 1..=capital_y {
        if !x.iter().copied().all(|x_i| x_i < z) {
            continue;
        }
        if !y.iter().copied().all(|y_i| y_i >= z) {
            continue;
        }

        println!("No War");
        return;
    }

    println!("War");
}
