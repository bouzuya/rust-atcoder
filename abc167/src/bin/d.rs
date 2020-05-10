use proconio::input;
use proconio::marker::Usize1;

fn main() {
    input! {
        n: usize,
        k: usize,
        a: [Usize1; n],
    };

    if k <= n {
        let mut ca = 0;
        for _ in 0..k {
            ca = a[ca];
        }
        println!("{}", ca + 1);
        return;
    }

    let mut c = vec![0; n];
    let mut ca = 0;
    loop {
        c[ca] += 1;
        if c[ca] == 3 {
            break;
        }
        ca = a[ca];
    }
    let mut start_count = 0;
    let mut loop_count = 0;
    for &c_i in c.iter() {
        match c_i {
            0 => {}
            1 => start_count += 1,
            2 => loop_count += 1,
            3 => loop_count += 1,
            _ => unreachable!(),
        }
    }
    let mut ca = 0;
    for _ in 0..start_count {
        ca = a[ca];
    }
    let rk = (k - start_count) % loop_count;
    for _ in 0..rk {
        ca = a[ca];
    }
    let ans = ca + 1;
    println!("{}", ans);
}
