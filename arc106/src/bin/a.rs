use proconio::input;

fn main() {
    input! {
        n: usize,
    };
    let mut a = 1;
    let mut x = 3_usize;
    loop {
        let mut b = 1;
        let mut y = 5_usize;
        loop {
            if x + y == n {
                println!("{} {}", a, b);
                return;
            }
            if y.checked_mul(5).is_none() || x + y * 5 > n {
                break;
            }
            b += 1;
            y *= 5;
        }
        if x.checked_mul(3).is_none() || x * 3 > n {
            break;
        }
        a += 1;
        x *= 3;
    }
    println!("-1");
}
