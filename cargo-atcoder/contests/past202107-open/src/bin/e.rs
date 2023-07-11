use proconio::input;

fn main() {
    input! {
        n: usize,
    };
    for k in 1..=30 {
        let mut x = 1_usize;

        for i in 0..30 {
            if i == k {
                match x.checked_add(1) {
                    Some(x_plus_1) => {
                        x = x_plus_1;
                    }
                    None => continue,
                }
            }
            match x.checked_mul(3) {
                Some(next) => {
                    x = next;
                }
                None => break,
            }
        }
        if 30 == k {
            match x.checked_add(1) {
                Some(x_plus_1) => {
                    x = x_plus_1;
                }
                None => continue,
            }
        }
        if x == n {
            println!("{}", k);
            return;
        }
    }
    println!("-1");
}
