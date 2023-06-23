use proconio::input;

fn main() {
    input! {
        r: usize,
        n: usize,
        m: usize,
        l: usize,
        s: [usize; l],
    };

    let mut rounds = 1_usize;
    let mut sum = 0_usize;
    let mut times = 0_usize;
    for s_i in s {
        sum += s_i;
        times += 1;
        if rounds > r || sum > n || times > m {
            println!("No");
            return;
        }
        if sum == n || times == m {
            rounds += 1;
            sum = 0;
            times = 0;
        }
    }
    let ans = rounds == r + 1 && sum == 0 && times == 0;
    println!("{}", if ans { "Yes" } else { "No" });
}
