use proconio::{input, marker::Chars};

fn main() {
    input! {
        x: Chars,
    };

    if x.iter().copied().all(|c| c == x[0]) {
        println!("Weak");
        return;
    }

    for (p, x_i) in x.iter().copied().zip(x.iter().copied().skip(1)) {
        if ((p as u8 - b'0') as usize + 1) % 10 != (x_i as u8 - b'0') as usize {
            println!("Strong");
            return;
        }
    }
    println!("Weak");
}
