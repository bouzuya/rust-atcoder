use proconio::input;

fn main() {
    input! {
        mut a: usize,
        mut b: usize,
        k: usize,
    };
    let mut is_a = true;
    for _ in 0..k {
        let (mut c, mut d) = if is_a { (a, b) } else { (b, a) };
        if c % 2 != 0 {
            c -= 1;
        }
        let h = c / 2;
        c -= h;
        d += h;
        if is_a {
            a = c;
            b = d;
        } else {
            a = d;
            b = c;
        }
        is_a = !is_a;
    }
    println!("{} {}", a, b);
}
