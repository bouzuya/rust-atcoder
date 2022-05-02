use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        a: [usize; n],
    };
    let mut ans = vec![false; k + 1];
    ans[0] = false;
    for i in 1..=k {
        let mut win = false;
        for a_i in a.iter().copied() {
            if i >= a_i {
                if !ans[i - a_i] {
                    win = true;
                }
            } else {
                break;
            }
        }
        ans[i] = win;
    }
    let ans = ans[k];
    println!("{}", if ans { "First" } else { "Second" });
}
