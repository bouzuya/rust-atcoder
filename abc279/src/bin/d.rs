use proconio::input;

fn f(a: usize, b: usize, g: usize) -> Option<f64> {
    if (g - 1).checked_mul(b).is_none() {
        return None;
    }

    if (g - 1) * b > a {
        return None;
    }

    Some(((g - 1) * b) as f64 + a as f64 / (g as f64).sqrt())
}

fn main() {
    input! {
        a: usize,
        b: usize,
    };

    // let mut min = a;
    // for g in 2.. {
    //     let t = (g - 1) as f64 * b + a / (g as f64).sqrt();
    //     if t < min {
    //         min = t;
    //     } else {
    //         break;
    //     }
    // }
    // println!("{}", min);

    let mut l = 2_usize;
    let mut r = 1_000_000_000_000_000_usize;
    for _ in 0..1000 {
        let ll = (l * 2 + r) / 3;
        let rr = (l + r * 2) / 3;

        if ll == rr {
            break;
        }

        match f(a, b, rr) {
            None => r = rr,
            Some(fr) => {
                let fl = f(a, b, ll).unwrap();
                if fl < fr {
                    r = rr;
                } else {
                    l = ll;
                }
            }
        }
    }

    let mut min = a as f64;
    for g in l..=r {
        if let Some(x) = f(a, b, g) {
            if x < min {
                min = x;
            }
        }
    }
    let ans = min;
    println!("{}", ans);
}
