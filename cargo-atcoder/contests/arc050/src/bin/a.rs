use proconio::input;

fn main() {
    input! {
        c_large: char,
        c_small: char,
    };
    let i_large = c_large as u8 - 'A' as u8;
    let i_small = c_small as u8 - 'a' as u8;
    let ans = i_large == i_small;
    println!("{}", if ans { "Yes" } else { "No" });
}
