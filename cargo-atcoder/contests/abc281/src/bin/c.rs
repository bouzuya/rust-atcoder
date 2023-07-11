use proconio::input;

fn main() {
    input! {
        n: usize,
        t: usize,
        a: [usize; n],
    };
    let sum = a.iter().sum::<usize>();
    let t = t % sum;

    let mut ans = (0, 0);
    let mut sum_t = 0_usize;
    for (i, a_i) in a.iter().copied().enumerate() {
        sum_t += a_i;
        if t <= sum_t {
            sum_t -= a_i;
            ans = (i + 1, t - sum_t);
            break;
        }
    }

    println!("{} {}", ans.0, ans.1);
}
