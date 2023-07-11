use proconio::input;

fn main() {
    input! {
        n: u64,
    };
    match n {
        0 => unreachable!(),
        1 => {
            println!("1");
            return;
        }
        2 => {
            println!("1");
            return;
        }
        3 => {
            println!("2");
            return;
        }
        _ => {
            let mut ok = 1_u64;
            let mut ng = n + 1;
            while ng - ok > 1 {
                let m = ok + (ng - ok) / 2;
                if m.checked_mul(m + 1).is_some() && m * (m + 1) / 2 <= n + 1 {
                    ok = m;
                } else {
                    ng = m;
                }
            }
            let ans = 1 + n - ok;
            println!("{}", ans);
        }
    }
}
