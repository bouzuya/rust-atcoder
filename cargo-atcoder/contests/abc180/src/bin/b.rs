use proconio::input;

fn main() {
    input! {
        n: usize,
        x: [i64; n],
    };
    println!("{}", x.iter().copied().map(|x_i| x_i.abs()).sum::<i64>());
    println!(
        "{}",
        x.iter()
            .copied()
            .map(|x_i| x_i.pow(2) as f64)
            .sum::<f64>()
            .sqrt()
    );
    println!("{}", x.iter().copied().map(|x_i| x_i.abs()).max().unwrap());
}
