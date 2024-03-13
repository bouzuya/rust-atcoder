use proconio::input;

fn main() {
    input! {
        n: usize,
        q: [usize; n],
        a: [usize; n],
        b: [usize; n],
    };

    let mut max = 0_usize;
    for x in 0..=1_000_000 {
        let mut ok = true;
        let mut y = 1_000_000;
        for ((q_i, a_i), b_i) in q
            .iter()
            .copied()
            .zip(a.iter().copied())
            .zip(b.iter().copied())
        {
            if q_i < a_i * x {
                ok = false;
                break;
            }
            let q_i = q_i - a_i * x;
            if b_i > 0 {
                y = y.min(q_i / b_i);
            }
        }
        if ok {
            max = max.max(x + y);
        }
    }
    let ans = max;
    println!("{}", ans);
}
