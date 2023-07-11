use proconio::input;
use proconio::marker::Usize1;

fn scoring(_d: usize, c: &Vec<i64>, s: &Vec<Vec<i64>>, t: &Vec<usize>) -> i64 {
    let mut last = vec![0; 26];
    let mut score = 0;
    for (i, &t_i) in t.iter().enumerate() {
        let d_i = i as i64 + 1;
        last[t_i] = d_i;
        score += s[i][t_i];
        for i in 0..26 {
            score -= c[i] * (d_i - last[i]);
        }
    }
    score
}

fn main() {
    input! {
        d: usize,
        c: [i64; 26],
        s: [[i64; 26]; d],
        mut t: [Usize1; d],
        m: usize,
        dq: [(Usize1, Usize1); m],
    };
    // let a = scoring(d, &c, &s, &t);
    for &(d_i, q_i) in dq.iter() {
        // let old = t[d_i];
        t[d_i] = q_i;
        let a_new = scoring(d, &c, &s, &t);
        // if a_new <= a {
        //     t[d_i] = old;
        // }
        println!("{}", a_new);
    }
}
