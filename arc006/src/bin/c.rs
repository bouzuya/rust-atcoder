use proconio::input;

fn main() {
    input! {
        n: usize,
        w: [i64; n],
    };
    let mut b = vec![false; n];
    for i in 0.. {
        let mut c = None;
        for (j, &w_j) in w.iter().enumerate() {
            if b[j] {
                continue;
            }
            match c {
                None => {
                    b[j] = true;
                    c = Some(w_j);
                }
                Some(w_c) => {
                    if w_j <= w_c {
                        b[j] = true;
                        c = Some(w_j);
                    }
                }
            }
        }
        if c.is_none() {
            println!("{}", i);
            break;
        }
    }
}
