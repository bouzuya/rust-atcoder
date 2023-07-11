use proconio::input;

fn main() {
    input! {
        a: usize,
        k: usize,
    };
    let g = 2_000_000_000_000;
    if k == 0 {
        println!("{}", g - a);
    } else {
        let mut count = 0_usize;
        let mut t = a;
        while t < g {
            match k.checked_mul(t) {
                None => break,
                Some(m) => match t.checked_add(1 + m) {
                    None => break,
                    Some(x) => t = x,
                },
            }
            count += 1;
        }
        println!("{}", count);
    }
}
