use proconio::input;

fn main() {
    input! {
        n: usize,
        d: usize,
        x: [[i64; d]; n],
    };
    let mut count = 0;
    for (i, x_i) in x.iter().enumerate() {
        for x_j in x.iter().skip(i + 1) {
            let mut d2 = 0;
            for (&x_ik, &x_jk) in x_i.iter().zip(x_j.iter()) {
                d2 += (x_ik - x_jk).pow(2);
            }
            for d in 1.. {
                if d * d == d2 {
                    count += 1;
                    break;
                }
                if d * d > d2 {
                    break;
                }
            }
        }
    }
    let ans = count;
    println!("{}", ans);
}
