use proconio::input;

fn main() {
    input! {
        k: usize,
    };
    for i in 0..k {
        print!("{}", (b'A' + i as u8) as char);
    }
    println!();
}
