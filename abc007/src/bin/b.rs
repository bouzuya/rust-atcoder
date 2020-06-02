use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        a: Chars,
    };
    if a.len() > 1 {
        println!("{}", a[0..a.len() - 1].iter().collect::<String>());
        return;
    }
    if a[0] == 'a' {
        println!("-1");
        return;
    }
    println!("{}", (a[0] as u8 - 1) as char);
}
