use proconio::input;

fn main() {
    input! {
        n: usize,
        d: usize,
        x: [[i64; d]; n],
    };

    let mut count = 0_usize;
    for i in 0..n {
        for j in i + 1..n {
            if i == j {
                continue;
            }

            let mut dist = 0_i64;
            for k in 0..d {
                dist += (x[i][k] - x[j][k]).pow(2);
            }

            for k in 1.. {
                if k * k > dist {
                    break;
                }

                if k * k == dist {
                    count += 1;
                    break;
                }
            }
        }
    }

    let ans = count;
    println!("{}", ans);
}
