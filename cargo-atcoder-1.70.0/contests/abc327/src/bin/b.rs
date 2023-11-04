use proconio::input;

fn main() {
    input! {
        b: usize,
    };
    for a in 1_usize.. {
        if let Some(x) = a.checked_pow(a as u32) {
            if x == b {
                println!("{}", a);
                return;
            }
        } else {
            break;
        }
    }
    println!("-1");
}
