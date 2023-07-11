use proconio::{input, marker::Chars};

fn main() {
    input! {
        mut s: Chars,
        t: Chars,
    };
    let mut first = true;
    let n = s.len();
    for i in 0..n {
        if s[i] == t[i] {
            continue;
        }

        if !first {
            println!("No");
            return;
        }
        first = false;

        if i + 1 >= n {
            println!("No");
            return;
        }

        s.swap(i, i + 1);

        if s[i] == t[i] {
            continue;
        }

        println!("No");
        return;
    }
    println!("Yes");
}
