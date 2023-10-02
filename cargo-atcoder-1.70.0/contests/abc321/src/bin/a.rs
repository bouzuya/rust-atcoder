use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: Chars,
    };

    let mut p = n[0];
    for d in n.into_iter().skip(1) {
        if p <= d {
            println!("No");
            return;
        }
        p = d;
    }
    println!("Yes");
}
