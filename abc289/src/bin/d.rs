use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
        m: usize,
        b: [usize; m],
        x: usize,
    };
    let mut mochi = vec![false; x + 1];
    for b_i in b {
        mochi[b_i] = true;
    }
    let mut ans = vec![false; x + 1];
    ans[0] = true;
    for i in 0..x {
        if !ans[i] {
            continue;
        }
        if mochi[i] {
            continue;
        }
        for a_j in a.iter().copied() {
            if i + a_j > x {
                continue;
            }
            if mochi[i + a_j] {
                continue;
            }
            ans[i + a_j] = true;
        }
    }
    println!("{}", if ans[x] { "Yes" } else { "No" });
}
