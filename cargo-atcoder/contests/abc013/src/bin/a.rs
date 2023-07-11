use proconio::input;

fn main() {
    input! {
        x: char,
    };
    let ans = x as u8 - 'A' as u8 + 1;
    println!("{}", ans);
}
