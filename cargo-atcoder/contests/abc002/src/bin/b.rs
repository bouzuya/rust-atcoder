use proconio::input;
use proconio::marker::Chars;

fn is_aiueo(c: char) -> bool {
    match c {
        'a' | 'i' | 'u' | 'e' | 'o' => true,
        _ => false,
    }
}

fn main() {
    input! {
        w: Chars,
    };

    let ans: String = w.iter().filter(|&&c| !is_aiueo(c)).collect();
    println!("{}", ans);
}
