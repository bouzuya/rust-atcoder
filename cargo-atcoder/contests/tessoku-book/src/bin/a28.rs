use proconio::input;

fn main() {
    input! {
        n: usize,
        ta: [(char, usize); n],
    };
    let mut ans = 0_usize;
    for (t, a) in ta {
        ans = match t {
            '+' => ans + a,
            '-' => ans + 10000 - a,
            '*' => ans * a,
            _ => unreachable!(),
        };
        ans %= 10000;
        println!("{}", ans);
    }
}
