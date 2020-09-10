use proconio::input;

fn main() {
    input! {
        n: usize,
    };
    let x = n * 100 / 108;
    for x_i in x.saturating_sub(1)..=x + 1 {
        if x_i * 108 / 100 == n {
            println!("{}", x_i);
            return;
        }
    }
    println!(":(");
}
