use proconio::input;

fn main() {
    input! {
        n: usize,
        mut a: [i64; n],
    };
    a.sort();
    let mut a = a.into_iter().collect::<std::collections::VecDeque<i64>>();
    let mut b = std::collections::VecDeque::<i64>::new();
    b.push_front(a.pop_front().unwrap());
    b.push_back(a.pop_back().unwrap());
    while !a.is_empty() {
        let a_f = *a.front().unwrap();
        let a_b = *a.back().unwrap();
        let b_f = *b.front().unwrap();
        let b_b = *b.back().unwrap();
        let v_afbf = (a_f - b_f).abs();
        let v_abbf = (a_b - b_f).abs();
        let v_afbb = (a_f - b_b).abs();
        let v_abbb = (a_b - b_b).abs();
        if v_afbf >= v_abbf && v_afbf >= v_afbb && v_afbf >= v_abbb {
            b.push_front(a.pop_front().unwrap());
        } else if v_abbf >= v_afbf && v_abbf >= v_afbb && v_abbf >= v_abbb {
            b.push_front(a.pop_back().unwrap());
        } else if v_afbb >= v_afbf && v_afbb >= v_abbf && v_afbb >= v_abbb {
            b.push_back(a.pop_front().unwrap());
        } else if v_abbb >= v_afbf && v_abbb >= v_abbf && v_abbb >= v_afbb {
            b.push_back(a.pop_back().unwrap());
        } else {
            unreachable!();
        }
    }

    let ans = b
        .iter()
        .enumerate()
        .skip(1)
        .map(|(i, &b_i)| (b_i - b[i - 1]).abs())
        .sum::<i64>();
    println!("{}", ans);
}
