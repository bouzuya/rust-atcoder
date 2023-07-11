use proconio::input;

fn main() {
    input! {
        n: usize,
    };
    let p = {
        let n = 55555;
        let mut p = vec![];
        let mut b = vec![true; n + 1];
        for i in 2.. {
            if i * i > n {
                for j in i..=n {
                    if b[j] {
                        p.push(j);
                    }
                }
                break;
            }
            if b[i] {
                p.push(i);
                for j in (i + i..=n).step_by(i) {
                    b[j] = false;
                }
            }
        }
        p
    };
    let ans = p.iter().copied().filter(|p_i| p_i % 5 == 1).take(n);
    for a in ans {
        println!("{}", a);
    }
}
