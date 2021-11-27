use proconio::input;

fn main() {
    input! {
        n: usize,
        w: usize,
        mut ab: [(usize, usize); n],
    };
    ab.sort();
    ab.reverse();

    let mut sum_v = 0_usize;
    let mut sum_w = 0_usize;
    for (a, b) in ab {
        if sum_w + b <= w {
            sum_w += b;
            sum_v += a * b;
        } else {
            let b = w - sum_w;
            sum_w += b;
            sum_v += a * b;
            break;
        }
    }

    let ans = sum_v;
    println!("{}", ans);
}
