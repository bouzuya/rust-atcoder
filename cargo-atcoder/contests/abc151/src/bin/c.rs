use proconio::input;
use proconio::marker::Usize1;

fn main() {
    input! {
        n: usize,
        m: usize,
        ps: [(Usize1, String); m],
    };
    let mut status = vec![(false, 0); n];
    for (p_i, s_i) in ps.iter() {
        if status[*p_i].0 {
            continue;
        }
        match s_i.as_str() {
            "AC" => status[*p_i].0 = true,
            "WA" => status[*p_i].1 += 1,
            _ => unreachable!("invalid S"),
        }
    }
    let mut count_ac = 0;
    let mut count_penalty = 0;
    for (ac, wa) in status {
        if ac {
            count_ac += 1;
            count_penalty += wa;
        }
    }
    println!("{} {}", count_ac, count_penalty);
}
