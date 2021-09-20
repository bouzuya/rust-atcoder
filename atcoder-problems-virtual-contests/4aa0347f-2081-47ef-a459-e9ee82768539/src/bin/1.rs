use proconio::input;

fn main() {
    input! {
        c: char,
    }
    let ans = (c as u8 - b'a' + 1 + b'a') as char;
    println!("{}", ans);
}
