use proconio::input;

fn main() {
    input! {
        t: usize,
        case: [usize; t],
    };

    for capital_n in case {
        let mut max = 1_usize;
        for len in 2..=18 {
            let mut ng = capital_n;
            let mut ok = 1;
            while ng - ok > 1 {
                let m = ok + (ng - ok) / 2;
                match m.to_string().repeat(len).parse::<usize>() {
                    Ok(x) if x <= capital_n => ok = m,
                    _ => ng = m,
                }
            }
            match ok.to_string().repeat(len).parse::<usize>() {
                Ok(n) if n <= capital_n => max = max.max(n),
                _ => {}
            }
        }
        println!("{}", max);
    }
}
