use proconio::input;
use proconio::marker::Chars;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct C(M, N);

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum M {
    S,
    H,
    D,
    C,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum N {
    NA,
    N2,
    N3,
    N4,
    N5,
    N6,
    N7,
    N8,
    N9,
    N10,
    NJ,
    NQ,
    NK,
}

fn is_ok(h: &[C]) -> bool {
    let h_m = h.iter().map(|&C(m, _)| m).collect::<Vec<_>>();
    let h_n = h.iter().map(|&C(_, n)| n).collect::<Vec<_>>();
    h.len() == 5
        && h_m.iter().all(|&m| m == h[0].0)
        && [N::N10, N::NJ, N::NQ, N::NK, N::NA]
            .iter()
            .all(|n| h_n.contains(&n))
}

fn main() {
    input! {
        s: Chars
    };
    let mut is_m = true;
    let mut m = M::S;
    let mut t = vec![];
    for c in s.iter() {
        if is_m {
            m = match c {
                'S' => M::S,
                'D' => M::D,
                'H' => M::H,
                'C' => M::C,
                _ => unreachable!(),
            };
            is_m = false;
        } else {
            let n = match c {
                'A' => N::NA,
                '2' => N::N2,
                '3' => N::N3,
                '4' => N::N4,
                '5' => N::N5,
                '6' => N::N6,
                '7' => N::N7,
                '8' => N::N8,
                '9' => N::N9,
                '1' => {
                    continue;
                }
                '0' => N::N10,
                'J' => N::NJ,
                'Q' => N::NQ,
                'K' => N::NK,
                _ => unreachable!(),
            };
            t.push(C(m, n));
            is_m = true;
        }
    }
    let mut ans: Option<Vec<C>> = None;
    for &mark in [M::S, M::D, M::H, M::C].iter() {
        let mut h = vec![];
        let mut a = vec![];
        for &C(m, n) in t.iter() {
            if m == mark && [N::N10, N::NJ, N::NQ, N::NK, N::NA].contains(&n) {
                h.push(C(m, n));
            } else {
                a.push(C(m, n));
            }
            if is_ok(&h) {
                break;
            }
        }
        if is_ok(&h) {
            match ans {
                None => ans = Some(a),
                Some(x) if x.len() > a.len() => ans = Some(a.into_iter().collect::<Vec<C>>()),
                Some(_) => {}
            }
        }
    }
    let v = ans.unwrap();
    if v.is_empty() {
        println!("0");
    } else {
        for &C(m, n) in v.iter() {
            print!(
                "{}{}",
                match m {
                    M::S => 'S',
                    M::D => 'D',
                    M::H => 'H',
                    M::C => 'C',
                },
                match n {
                    N::NA => "A",
                    N::N2 => "2",
                    N::N3 => "3",
                    N::N4 => "4",
                    N::N5 => "5",
                    N::N6 => "6",
                    N::N7 => "7",
                    N::N8 => "8",
                    N::N9 => "9",
                    N::N10 => "10",
                    N::NJ => "J",
                    N::NQ => "Q",
                    N::NK => "K",
                }
            );
        }
        println!();
    }
}
