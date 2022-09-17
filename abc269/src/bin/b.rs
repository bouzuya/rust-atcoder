use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: [Chars; 10]
    };
    let mut a = 0;
    for i in 0..10 {
        let mut ok = false;
        for j in 0..10 {
            if s[i][j] == '#' {
                ok = true;
            }
        }
        if ok {
            a = i;
            break;
        }
    }

    let mut b = 0;
    for i in (0..10).rev() {
        let mut ok = false;
        for j in 0..10 {
            if s[i][j] == '#' {
                ok = true;
            }
        }
        if ok {
            b = i;
            break;
        }
    }

    let mut c = 0;
    for j in 0..10 {
        let mut ok = false;
        for i in 0..10 {
            if s[i][j] == '#' {
                ok = true;
            }
        }
        if ok {
            c = j;
            break;
        }
    }

    let mut d = 0;
    for j in (0..10).rev() {
        let mut ok = false;
        for i in 0..10 {
            if s[i][j] == '#' {
                ok = true;
            }
        }
        if ok {
            d = j;
            break;
        }
    }

    println!("{} {}", a + 1, b + 1);
    println!("{} {}", c + 1, d + 1);
}
