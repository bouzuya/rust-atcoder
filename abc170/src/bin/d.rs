use proconio::input;

fn main() {
    input! {
        n: usize,
        mut a: [usize; n],
    };
    let max = 1_000_000_usize;
    a.sort();

    let mut count = vec![0; max + 1];
    for a_i in a.iter().copied() {
        count[a_i] += 1;
    }

    let mut ans = 0_usize;
    let mut ok = vec![false; max + 1];
    for a_i in a {
        if !ok[a_i] {
            if count[a_i] == 1 {
                ans += 1;
            }
            for j in (a_i..=max).step_by(a_i) {
                ok[j] = true;
            }
        }
    }

    println!("{}", ans);
}
