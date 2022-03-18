use proconio::input;

fn main() {
    input! {
        l: usize,
        r: usize,
    };

    let n = 1_000_000;
    let mut ps = vec![];
    let mut b1 = vec![true; n + 1];
    b1[0] = false;
    b1[1] = false;
    for i in 2.. {
        if i * i > n {
            for j in i..=n {
                if b1[j] {
                    ps.push(j);
                }
            }
            break;
        }
        if b1[i] {
            ps.push(i);
            for j in (i + i..=n).step_by(i) {
                b1[j] = false;
            }
        }
    }

    //        2 3   5
    //    0 1 2 3 4 5 6
    //             [5 6]
    // 2: 0 1 0 1 0 1 0
    // 3: 0 1 2 0 1 2 0
    // 5: 0 1 2 3 4 0 1
    //
    let mut b2 = vec![true; r - l + 1];
    for p in ps.iter().copied() {
        let o = (p - (l % p)) % p;
        for q in (o..r - l + 1).step_by(p) {
            b2[q] = false;
        }
    }
    for (i, b2_i) in b2.iter_mut().enumerate() {
        if l + i <= 1_000_000 {
            *b2_i = b1[l + i];
        }
    }

    let ans = b2.into_iter().filter(|b| *b).count();
    println!("{}", ans);
}
