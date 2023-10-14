use proconio::input;

fn main() {
    input! {
        n: usize,
    };
    for x in 0_u32..=40 {
        for y in 0_u32..=40 {
            if let Some(x2) = 2_usize.checked_pow(x) {
                if let Some(y3) = 3_usize.checked_pow(y) {
                    if let Some(v) = x2.checked_mul(y3) {
                        if n == v {
                            println!("Yes");
                            return;
                        }
                    }
                }
            }
        }
    }
    println!("No");
}
