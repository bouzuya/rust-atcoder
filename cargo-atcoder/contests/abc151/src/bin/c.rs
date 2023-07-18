use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        m: usize,
        ps: [(Usize1, String); m],
    };

    let mut ac = vec![false; n];
    let mut wa = vec![0_usize; n];
    for (p_i, s_i) in ps {
        match s_i.as_str() {
            "AC" => {
                ac[p_i] = true;
            }
            "WA" => {
                if ac[p_i] {
                    continue;
                }
                wa[p_i] += 1;
            }
            _ => unreachable!(),
        }
    }

    let sum_ac = ac.iter().copied().filter(|&x| x).count();
    let sum_wa = ac
        .iter()
        .copied()
        .zip(wa.iter().copied())
        .filter(|&(ac_i, _)| ac_i)
        .map(|(_, wa_i)| wa_i)
        .sum::<usize>();
    println!("{} {}", sum_ac, sum_wa);
}
