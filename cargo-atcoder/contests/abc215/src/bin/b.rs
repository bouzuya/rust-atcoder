use proconio::input;

fn main() {
    input! {
        n: usize,
    };
    for k in 0_u32.. {
        match 2_usize.checked_pow(k) {
            Some(x) if x <= n => continue,
            _ => {
                println!("{}", k.saturating_sub(1));
                return;
            }
        }
    }
}
