use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        s: Chars,
    };

    let mut ok = true;
    let mut m = 'M';
    for c in s.iter().copied() {
        if c != m {
            ok = false;
            break;
        }
        m = if m == 'M' { 'F' } else { 'M' };
    }
    if ok {
        println!("Yes");
        return;
    }

    let mut ok = true;
    let mut m = 'F';
    for c in s.iter().copied() {
        if c != m {
            ok = false;
            break;
        }
        m = if m == 'M' { 'F' } else { 'M' };
    }
    if ok {
        println!("Yes");
        return;
    }

    println!("No");
}
