use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        s: Chars,
    };
    let c1 = s[0] == 'A';
    let c2 = s[2..=s.len() - 2].iter().filter(|&&c| c == 'C').count() == 1;
    let i_c = s.iter().position(|&c| c == 'C');
    let c3 = s
        .iter()
        .enumerate()
        .all(|(i, &c_i)| i == 0 || Some(i) == i_c || c_i.is_lowercase());
    let ans = c1 && c2 && c3;
    println!("{}", if ans { "AC" } else { "WA" });
}
