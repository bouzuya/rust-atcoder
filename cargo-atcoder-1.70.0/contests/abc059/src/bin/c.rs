use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [i64; n],
    };

    // +-+-...
    let mut ans1 = 0_i64;
    let mut sum = 0_i64;
    for (i, a_i) in a.iter().copied().enumerate() {
        sum += a_i;
        if i % 2 == 0 {
            if sum <= 0 {
                let count = (1 - sum).abs();
                ans1 += count;
                sum += count;
            }
        } else if sum >= 0 {
            let count = (-1 - sum).abs();
            ans1 += count;
            sum += -count;
        }
    }

    // -+-+... で最小にする
    let mut ans2 = 0_i64;
    let mut sum = 0_i64;
    for (i, a_i) in a.iter().copied().enumerate() {
        sum += a_i;
        if i % 2 == 0 {
            if sum >= 0 {
                let count = (-1 - sum).abs();
                ans2 += count;
                sum += -count;
            }
        } else if sum <= 0 {
            let count = (1 - sum).abs();
            ans2 += count;
            sum += count;
        }
    }

    println!("{}", ans1.min(ans2));
}
