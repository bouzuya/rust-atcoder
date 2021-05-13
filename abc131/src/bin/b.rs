use proconio::input;

fn main() {
    input! {
        n: i64,
        l: i64,
    };
    let mut sum = 0_i64;
    for i in 1..=n {
        sum += l + i - 1;
    }

    let mut min_d = None;
    for j in 1..=n {
        let mut s = 0_i64;
        for i in 1..=n {
            if i == j {
                continue;
            }
            s += l + i - 1;
        }
        let d = (sum - s).abs();
        min_d = Some(match min_d {
            None => (d, s),
            Some((min_d, min_s)) => {
                if d < min_d {
                    (d, s)
                } else {
                    (min_d, min_s)
                }
            }
        });
    }

    let ans = min_d.unwrap().1;
    println!("{}", ans);
}
