use proconio::input;
use proconio::marker::Usize1;

fn scoring(_d: usize, c: &Vec<i64>, s: &Vec<Vec<i64>>, t: &Vec<usize>) -> Vec<i64> {
    let mut scores = vec![];
    let mut last = vec![0; 26];
    let mut score = 0;
    for (i, &t_i) in t.iter().enumerate() {
        let d_i = i as i64 + 1;
        last[t_i] = d_i;
        score += s[i][t_i];
        for i in 0..26 {
            score -= c[i] * (d_i - last[i]);
        }
        scores.push(score);
    }
    scores
}

fn main() {
    input! {
        d: usize,
        c: [i64; 26],
        s: [[i64; 26]; d],
        t: [Usize1; d],
    };
    let a = scoring(d, &c, &s, &t);
    for &a_i in a.iter() {
        println!("{}", a_i);
    }
}
