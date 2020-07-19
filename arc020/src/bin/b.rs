use proconio::input;

fn main() {
    input! {
        n: i64,
        c: i64,
        a: [i64; n]
    };

    let mut ans = c * n + 1;
    for c_e in 1..=10 {
        for c_o in 1..=10 {
            if c_e == c_o {
                continue;
            }
            let mut e = 0;
            let mut o = 0;
            for (i, &a_i) in a.iter().enumerate() {
                if i % 2 == 0 {
                    e += if a_i == c_e { 0 } else { c };
                } else {
                    o += if a_i == c_o { 0 } else { c };
                }
            }
            ans = std::cmp::min(ans, e + o);
        }
    }
    println!("{}", ans);
}
