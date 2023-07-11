use proconio::input;
use proconio::marker::Bytes;

fn main() {
    input! {
        n: usize,
        k: usize,
        c: usize,
        cv: Bytes,
    };

    let mut lv = vec![n; k];
    for i in 0..n {
        if cv[i] == b'o' {
            lv[0] = i;
            break;
        }
    }
    for i in 1..k {
        for j in lv[i - 1] + 1 + c..n {
            if cv[j] == b'o' {
                lv[i] = j;
                break;
            }
        }
    }
    // println!("lv: {:?}", lv);

    let mut rv = vec![n; k];
    for i in (0..n).rev() {
        if cv[i] == b'o' {
            rv[k - 1] = i;
            break;
        }
    }
    for i in (0..k - 1).rev() {
        for j in (0..rv[i + 1] - c).rev() {
            if cv[j] == b'o' {
                rv[i] = j;
                break;
            }
        }
    }
    // println!("rv: {:?}", rv);

    for i in 0..k {
        if lv[i] == rv[i] {
            println!("{}", lv[i] + 1);
        }
    }
}
