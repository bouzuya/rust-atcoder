use proconio::input;
use superslice::*;

fn l(s: &Vec<i64>, x: i64) -> usize {
    let i = s.lower_bound(&x);
    if i >= s.len() {
        i - 1
    } else {
        if s[i] == x {
            i
        } else {
            i.saturating_sub(1)
        }
    }
}

#[test]
fn test_l() {
    assert_eq!(l(&vec![1, 3, 5], 0), 0);
    assert_eq!(l(&vec![1, 3, 5], 1), 0);
    assert_eq!(l(&vec![1, 3, 5], 2), 0);
    assert_eq!(l(&vec![1, 3, 5], 3), 1);
    assert_eq!(l(&vec![1, 3, 5], 4), 1);
    assert_eq!(l(&vec![1, 3, 5], 5), 2);
    assert_eq!(l(&vec![1, 3, 5], 6), 2);
}

fn r(s: &Vec<i64>, x: i64) -> usize {
    let i = s.upper_bound(&x);
    if i >= s.len() {
        i - 1
    } else {
        if s[i.saturating_sub(1)] == x {
            i.saturating_sub(1)
        } else {
            i
        }
    }
}

#[test]
fn test_r() {
    assert_eq!(r(&vec![1, 3, 5], 0), 0);
    assert_eq!(r(&vec![1, 3, 5], 1), 0);
    assert_eq!(r(&vec![1, 3, 5], 2), 1);
    assert_eq!(r(&vec![1, 3, 5], 3), 1);
    assert_eq!(r(&vec![1, 3, 5], 4), 2);
    assert_eq!(r(&vec![1, 3, 5], 5), 2);
    assert_eq!(r(&vec![1, 3, 5], 6), 2);
}

fn main() {
    input! {
        a: usize,
        b: usize,
        q: usize,
        s: [i64; a],
        t: [i64; b],
        x: [i64; q],
    };
    for x_i in x {
        let i_sl = l(&s, x_i);
        let i_sr = r(&s, x_i);
        let i_tl = l(&t, x_i);
        let i_tr = r(&t, x_i);
        let i_sltl = l(&t, s[i_sl]);
        let i_sltr = r(&t, s[i_sl]);
        let i_srtl = l(&t, s[i_sr]);
        let i_srtr = r(&t, s[i_sr]);
        let i_tlsl = l(&s, t[i_tl]);
        let i_tlsr = r(&s, t[i_tl]);
        let i_trsl = l(&s, t[i_tr]);
        let i_trsr = r(&s, t[i_tr]);
        let ans = *vec![
            (x_i - s[i_sl]).abs() + (s[i_sl] - t[i_sltl]).abs(),
            (x_i - s[i_sl]).abs() + (s[i_sl] - t[i_sltr]).abs(),
            (x_i - s[i_sr]).abs() + (s[i_sr] - t[i_srtl]).abs(),
            (x_i - s[i_sr]).abs() + (s[i_sr] - t[i_srtr]).abs(),
            (x_i - t[i_tl]).abs() + (t[i_tl] - s[i_tlsl]).abs(),
            (x_i - t[i_tl]).abs() + (t[i_tl] - s[i_tlsr]).abs(),
            (x_i - t[i_tr]).abs() + (t[i_tr] - s[i_trsl]).abs(),
            (x_i - t[i_tr]).abs() + (t[i_tr] - s[i_trsr]).abs(),
        ]
        .iter()
        .min()
        .unwrap();
        println!("{}", ans);
    }
}
