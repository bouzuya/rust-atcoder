use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        disk: [usize; m],
    };
    let mut p = (0..=n).collect::<Vec<usize>>();
    let mut c = (0..=n).collect::<Vec<usize>>();
    for d in disk {
        let p_d = p[d];
        c[p_d] = c[0];
        p[c[0]] = p_d;
        c[0] = d;
        p[c[0]] = 0;
    }

    for c_i in c.iter().skip(1) {
        println!("{}", c_i);
    }
}
