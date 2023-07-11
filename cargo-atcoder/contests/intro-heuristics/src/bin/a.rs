use proconio::input;

#[allow(dead_code)]
fn scoring(d: usize, c: &Vec<i64>, s: &Vec<Vec<i64>>, t: &Vec<usize>) -> i64 {
    let mut last = vec![0; 26];
    let mut score = 0;
    for i in 0..d {
        let d_i = i as i64 + 1;
        let t_i = t[i];
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
    };
    let mut t = vec![0; d];
    for i in 0..d {
        let d_i = i + 1;
        let mut max_x_i = 0;
        let mut max_x = scoring(d_i, &c, &s, &t);
        for j in 1..26 {
            t[i] = j;
            let x = scoring(d_i, &c, &s, &t);
            if x > max_x {
                max_x = x;
                max_x_i = j;
            }
        }
        t[i] = max_x_i + 1;
    }
    // println!("{}", scoring(d, &c, &s, &t));
    for &t_i in t.iter() {
        println!("{}", t_i);
    }
}
