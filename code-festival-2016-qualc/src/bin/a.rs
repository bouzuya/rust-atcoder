use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        s: Chars
    };
    match s
        .iter()
        .position(|&s_i| s_i == 'C')
        .and_then(|i_c| s.iter().skip(i_c + 1).position(|&s_i| s_i == 'F'))
    {
        None => println!("No"),
        Some(_) => println!("Yes"),
    };
}
