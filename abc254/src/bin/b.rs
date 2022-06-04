use proconio::input;

fn main() {
    input! {
        n: usize,
    };
    let mut p = vec![];
    for _ in 0..n {
        let mut c = vec![1];
        for j in 0..p.len() {
            if j + 1 < p.len() {
                c.push(p[j] + p[j + 1]);
            } else {
                c.push(1);
            }
        }

        for (i, c_i) in c.iter().copied().enumerate() {
            print!("{}{}", c_i, if i == c.len() - 1 { '\n' } else { ' ' });
        }
        p = c;
    }
}
