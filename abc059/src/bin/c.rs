use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [i64; n],
    };

    let a_e = {
        let mut res = 0_i64;
        let mut sum = 0_i64;
        for (i, &a_i) in a.iter().enumerate() {
            sum += a_i;
            if i % 2 == 0 {
                if sum <= 0 {
                    res += 1 - sum;
                    sum = 1;
                }
            } else {
                if sum >= 0 {
                    res += 1 + sum;
                    sum = -1;
                }
            }
        }
        res
    };

    let a_o = {
        let mut res = 0_i64;
        let mut sum = 0_i64;
        for (i, &a_i) in a.iter().enumerate() {
            sum += a_i;
            if i % 2 == 0 {
                if sum >= 0 {
                    res += 1 + sum;
                    sum = -1;
                }
            } else {
                if sum <= 0 {
                    res += 1 - sum;
                    sum = 1;
                }
            }
        }
        res
    };

    let ans = std::cmp::min(a_e, a_o);
    println!("{}", ans);
}
