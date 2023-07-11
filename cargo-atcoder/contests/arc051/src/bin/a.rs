use proconio::input;

fn main() {
    input! {
        x_1: i64,
        y_1: i64,
        r: i64,
        x_2: i64,
        y_2: i64,
        x_3: i64,
        y_3: i64,
    };

    let c_t = (x_1, y_1 + r);
    let c_r = (x_1 + r, y_1);
    let c_b = (x_1, y_1 - r);
    let c_l = (x_1 - r, y_1);
    let s_tl = (x_2, y_2);
    let s_tr = (x_3, y_2);
    let s_br = (x_3, y_3);
    let s_bl = (x_2, y_3);

    let red = !(((x_2..=x_3).contains(&c_t.0) && (y_2..=y_3).contains(&c_t.1))
        && ((x_2..=x_3).contains(&c_r.0) && (y_2..=y_3).contains(&c_t.1))
        && ((x_2..=x_3).contains(&c_b.0) && (y_2..=y_3).contains(&c_b.1))
        && ((x_2..=x_3).contains(&c_l.0) && (y_2..=y_3).contains(&c_l.1)));
    let blue = !(((s_tl.0 - x_1).pow(2) + (s_tl.1 - y_1).pow(2) <= r.pow(2))
        && ((s_tr.0 - x_1).pow(2) + (s_tr.1 - y_1).pow(2) <= r.pow(2))
        && ((s_br.0 - x_1).pow(2) + (s_br.1 - y_1).pow(2) <= r.pow(2))
        && ((s_bl.0 - x_1).pow(2) + (s_bl.1 - y_1).pow(2) <= r.pow(2)));
    println!("{}", if red { "YES" } else { "NO" });
    println!("{}", if blue { "YES" } else { "NO" });
}
