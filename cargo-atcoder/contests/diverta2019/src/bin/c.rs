use proconio::input;

fn main() {
    input! {
        n: usize,
        s: [String; n]
    };
    let mut c_sb = 0_i64;
    let mut c_ea = 0_i64;
    let mut c_sbea = 0_i64;
    let mut c_ab = 0_i64;
    for s_i in s.iter() {
        let sb = s_i.starts_with('B');
        let ea = s_i.ends_with('A');
        if sb && ea {
            c_sbea += 1;
        } else if sb {
            c_sb += 1;
        } else if ea {
            c_ea += 1;
        }
        c_ab += s_i.matches("AB").collect::<Vec<_>>().len() as i64;
    }
    let ans = c_ab
        + std::cmp::max(c_sbea - 1, 0)
        + if c_sbea > 0 {
            let mut c = 0;
            if c_sb > 0 {
                c_sb -= 1;
                c += 1;
            }
            if c_ea > 0 {
                c_ea -= 1;
                c += 1;
            }
            c + std::cmp::min(c_sb, c_ea)
        } else {
            std::cmp::min(c_sb, c_ea)
        };

    println!("{}", ans);
}
