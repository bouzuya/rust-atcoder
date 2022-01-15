use proconio::{input, marker::Bytes};

fn main() {
    input! {
        abc: Bytes,
    };
    let a = (abc[0] - b'0') as usize;
    let b = (abc[1] - b'0') as usize;
    let c = (abc[2] - b'0') as usize;
    let ans = (a * 100 + b * 10 + c) + (b * 100 + c * 10 + a) + (c * 100 + a * 10 + b);
    println!("{}", ans);
}
