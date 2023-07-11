use proconio::{input, marker::Bytes};

fn main() {
    input! {
        n: Bytes,
    };

    let mut x = 0_usize;
    for n_i in n {
        x += (n_i - b'0') as usize;
        x %= 9;
    }
    let ans = x == 0;
    println!("{}", if ans { "Yes" } else { "No" });
}
