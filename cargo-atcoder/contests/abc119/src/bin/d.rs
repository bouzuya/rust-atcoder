use proconio::input;
use superslice::Ext;

fn main() {
    input! {
        a: usize,
        b: usize,
        q: usize,
        s: [usize; a],
        t: [usize; b],
        x: [usize; q],
    };

    let inf = 1 << 60;
    for x_i in x {
        let j = s.lower_bound(&x_i);
        let (s_l, s_r) = if j == 0 {
            (inf, s[j] - x_i)
        } else if j == a {
            (
                if x_i >= s[a - 1] {
                    x_i - s[a - 1]
                } else {
                    s[a - 1] - x_i
                },
                inf,
            )
        } else {
            (x_i - s[j - 1], s[j] - x_i)
        };

        let j = t.lower_bound(&x_i);
        let (t_l, t_r) = if j == 0 {
            (inf, t[j] - x_i)
        } else if j == b {
            (
                if x_i >= t[b - 1] {
                    x_i - t[b - 1]
                } else {
                    t[b - 1] - x_i
                },
                inf,
            )
        } else {
            (x_i - t[j - 1], t[j] - x_i)
        };

        println!(
            "{}",
            vec![
                s_l * 2 + t_r,
                s_l + t_r * 2,
                t_l * 2 + s_r,
                t_l + s_r * 2,
                s_l.max(t_l),
                s_r.max(t_r)
            ]
            .into_iter()
            .min()
            .unwrap()
        );
    }
}
