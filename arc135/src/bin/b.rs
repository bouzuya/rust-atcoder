use proconio::input;

fn main() {
    input! {
        n: usize,
        s: [i64; n],
    };
    let mut p = (0, 0);
    let mut c = (0, 0, s[0]);
    for (i, s_i) in s.iter().copied().enumerate() {
        let p2 = s_i - (p.0 + p.1);
        p.0 = p.1;
        p.1 = p2;
        match (i + 2) % 3 {
            0 => c.0 = c.0.min(p2),
            1 => c.1 = c.1.min(p2),
            2 => c.2 = c.2.min(p2),
            _ => unreachable!(),
        }
    }
    let ans = -c.0 + -c.1 <= c.2;
    println!("{}", if ans { "Yes" } else { "No" });
    if ans {
        let c0 = -c.0;
        let c1 = -c.1;
        println!("{}", c0);
        println!("{}", c1);
        let mut p = (c0, c1);
        for s_i in s {
            let p2 = s_i - (p.0 + p.1);
            println!("{}", p2);
            p.0 = p.1;
            p.1 = p2;
        }
    }
}
