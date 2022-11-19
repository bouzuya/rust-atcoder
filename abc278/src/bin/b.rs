use proconio::input;

fn main() {
    input! {
        mut h: usize,
        mut m: usize,
    };

    loop {
        let h1 = h / 10;
        let h2 = h % 10;
        let m1 = m / 10;
        let m2 = m % 10;

        if (0..=23).contains(&(h1 * 10 + m1)) && (0..=59).contains(&(h2 * 10 + m2)) {
            println!("{}{} {}{}", h1, h2, m1, m2);
            return;
        }

        m += 1;
        if m == 60 {
            m = 0;
            h += 1;
            if h == 24 {
                h = 0;
            }
        }
    }
}
