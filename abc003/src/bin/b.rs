use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        s: Chars,
        t: Chars,
    };
    let atcoder = "atcoder".chars().collect::<Vec<char>>();
    for (&s_i, &t_i) in s.iter().zip(t.iter()) {
        if s_i == t_i {
            continue;
        }
        if s_i == '@' && t_i == '@' {
            continue;
        }
        if s_i == '@' && atcoder.contains(&t_i) {
            continue;
        }
        if atcoder.contains(&s_i) && t_i == '@' {
            continue;
        }
        println!("You will lose");
        return;
    }
    println!("You can win");
}
