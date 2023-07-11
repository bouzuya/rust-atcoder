use proconio::input;

fn main() {
    input! {
        c: char,
    };
    let ans = (c as u8 + 1) as char;
    println!("{}", ans);
}
