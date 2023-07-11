use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        mut ab: [(usize, usize); n],
    }

    ab.sort();
    let mut sum = 0_usize;
    let mut count = 0_usize;
    for (a_i, b_i) in ab {
        if count + b_i > m {
            sum += a_i * (m - count);
            break;
        } else {
            sum += a_i * b_i;
            count += b_i;
        }
    }

    let ans = sum;
    println!("{}", ans);
}
