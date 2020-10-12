use proconio::input;

fn main() {
    input! {
        n: usize,
        p: [i64; n],
    };
    let ans = p
        .windows(3)
        .map(|pw| {
            if (pw[0] < pw[1] && pw[1] < pw[2]) || (pw[0] > pw[1] && pw[1] > pw[2]) {
                1
            } else {
                0
            }
        })
        .sum::<usize>();
    println!("{}", ans);
}
