use proconio::input;
use proconio::marker::Bytes;

fn match_char(c1: u8, c2: u8) -> bool {
    c1 == b'?' || c2 == b'?' || c1 == c2
}

fn main() {
    input! {
        a: Bytes,
        b: Bytes,
        c: Bytes,
    };

    let mut ans = a.len() + b.len() + c.len();
    ans = std::cmp::min(ans, f(&a, &b, &c));
    ans = std::cmp::min(ans, f(&a, &c, &b));
    ans = std::cmp::min(ans, f(&b, &a, &c));
    ans = std::cmp::min(ans, f(&b, &c, &a));
    ans = std::cmp::min(ans, f(&c, &a, &b));
    ans = std::cmp::min(ans, f(&c, &b, &a));
    println!("{}", ans);
}

fn f(av: &Vec<u8>, bv: &Vec<u8>, cv: &Vec<u8>) -> usize {
    use std::cmp::{max, min};

    let oabv = new_ov(&av, &bv);
    let obcv = new_ov(&bv, &cv);
    let oacv = new_ov(&av, &cv);

    let mut len = av.len() + bv.len() + cv.len();
    for ob in oabv
        .iter()
        .enumerate()
        .filter(|(_, &ok)| ok)
        .map(|(i, _)| i)
    {
        let r = max(av.len(), ob + bv.len());
        for oc in ob..=r {
            if (oc - ob >= obcv.len() || obcv[oc - ob]) && (oc >= oacv.len() || oacv[oc]) {
                len = min(len, max(r, oc + cv.len()));
            }
        }
    }
    len
}

fn new_ov(av: &Vec<u8>, bv: &Vec<u8>) -> Vec<bool> {
    let mut ov = vec![false; av.len() + 1];
    for i_okv in 0..ov.len() {
        let mut ok = true;
        for i_av in i_okv..std::cmp::min(av.len(), i_okv + bv.len()) {
            if !match_char(av[i_av], bv[i_av - i_okv]) {
                ok = false;
                break;
            }
        }
        ov[i_okv] = ok;
    }
    ov
}
