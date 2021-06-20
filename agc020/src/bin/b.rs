use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [i64; n],
    };

    let min = {
        let mut n = 2;
        for &a_i in a.iter().rev().skip(1) {
            if a_i >= n {
                n = a_i;
            } else {
                n = (n + a_i - 1) / a_i * a_i;
            }
        }
        let mut m = n;
        for &a_i in a.iter() {
            m = m / a_i * a_i;
        }
        if m != 2 {
            println!("-1");
            return;
        }
        n
    };
    let max = {
        let mut n = 2 * 2 - 1;
        for &a_i in a.iter().rev().skip(1) {
            let g = n / a_i;
            n = a_i * g + a_i - 1;
        }
        n
    };

    println!("{} {}", min, max);
}
