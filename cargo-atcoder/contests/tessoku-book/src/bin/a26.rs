use proconio::input;

fn main() {
    input! {
        q: usize,
        x: [usize; q],
    };
    let mut ps = vec![true; 300_000 + 1];
    ps[0] = false;
    ps[1] = false;
    for i in 2..ps.len() {
        if !ps[i] {
            continue;
        }
        for j in ((i + i)..ps.len()).step_by(i) {
            ps[j] = false;
        }
    }

    for x_i in x {
        let ans = ps[x_i];
        println!("{}", if ans { "Yes" } else { "No" });
    }
}
