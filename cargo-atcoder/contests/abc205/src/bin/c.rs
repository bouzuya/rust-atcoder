use proconio::input;

fn main() {
    input! {
        a: i64,
        b: i64,
        c: i64,
    };
    match (a >= 0, b >= 0) {
        (true, true) => {
            if a == b {
                println!("=");
            } else if a > b {
                println!(">");
            } else {
                println!("<");
            }
        }
        (false, false) => {
            if c % 2 == 0 {
                if a == b {
                    println!("=");
                } else if a > b {
                    println!("<");
                } else {
                    println!(">");
                }
            } else {
                if a == b {
                    println!("=");
                } else if a > b {
                    println!(">");
                } else {
                    println!("<");
                }
            }
        }
        (true, false) | (false, true) => {
            if c % 2 == 0 {
                let a = a.abs();
                let b = b.abs();
                if a == b {
                    println!("=");
                } else if a > b {
                    println!(">");
                } else {
                    println!("<");
                }
            } else {
                if a >= 0 {
                    println!(">");
                } else {
                    println!("<");
                }
            }
        }
    }
}
