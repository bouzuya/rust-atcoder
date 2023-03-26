use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: Chars,
    };
    if n.len() <= 2 {
        println!("0");
        return;
    }
    println!("{}", n[0..n.len() - 2].iter().collect::<String>());
}
