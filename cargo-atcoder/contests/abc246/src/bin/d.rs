use proconio::input;

fn main() {
    input! {
        n: usize,
    };

    if n == 0 {
        println!("0");
        return;
    }

    let mut max_a = 1;
    for a in 1.. {
        if a * a * a >= n {
            max_a = a;
            break;
        }
    }

    let f = |a: usize, b: usize| a.pow(3) + a.pow(2) * b + a * b.pow(2) + b.pow(3);
    let mut max_b = max_a;
    let mut min = 1 << 60;
    for a in 0..=max_a {
        for b in (a..=max_b).rev() {
            let x = f(a, b);
            if x >= n {
                min = min.min(x);
            } else {
                max_b = b;
                break;
            }
        }
    }
    let ans = min;
    println!("{}", ans);
}
