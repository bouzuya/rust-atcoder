use proconio::input;

fn main() {
    input! {
        n: usize,
        q: usize,
        x: [usize; q],
    };
    let mut pos = (0..=n).collect::<Vec<usize>>();
    let mut val = (0..=n).collect::<Vec<usize>>();
    for x_i in x {
        let p_x = pos[x_i];
        let (p_l, p_r) = if p_x == n {
            (p_x - 1, p_x)
        } else {
            (p_x, p_x + 1)
        };
        let (v_l, v_r) = (val[p_l], val[p_r]);
        val.swap(p_l, p_r);
        pos.swap(v_l, v_r);
    }
    for a in val.into_iter().skip(1) {
        println!("{}", a);
    }
}
