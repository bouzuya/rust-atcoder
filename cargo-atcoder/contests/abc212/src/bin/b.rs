use proconio::{input, marker::Bytes};

fn main() {
    input! {
        x: Bytes,
    };
    let x = x
        .into_iter()
        .map(|c| (c - b'0') as usize)
        .collect::<Vec<usize>>();
    let x_0 = x[0];
    if x.iter().copied().all(|x_i| x_i == x_0) {
        println!("Weak");
        return;
    }
    for (i, x_i) in x.iter().copied().enumerate() {
        if (x_0 + i) % 10 != x_i {
            println!("Strong");
            return;
        }
    }
    println!("Weak");
}
