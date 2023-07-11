use proconio::{input, marker::Usize1};

// 貪欲法
#[allow(dead_code)]
fn f1(pqr: &[(usize, usize, usize)]) -> Vec<char> {
    let mut x = vec![0_i64; 20];
    let score = |x: &[i64]| -> usize { x.iter().filter(|a_i| a_i == &&0).count() };
    let a = |x: &mut Vec<i64>, p: usize, q: usize, r: usize| {
        x[p] += 1;
        x[q] += 1;
        x[r] += 1;
    };
    let b = |x: &mut Vec<i64>, p: usize, q: usize, r: usize| {
        x[p] -= 1;
        x[q] -= 1;
        x[r] -= 1;
    };
    let mut ret = vec![];
    for (p, q, r) in pqr.iter().copied() {
        a(&mut x, p, q, r);
        let score_a = score(&x);
        b(&mut x, p, q, r);
        b(&mut x, p, q, r);
        let score_b = score(&x);
        a(&mut x, p, q, r);
        if score_a > score_b {
            a(&mut x, p, q, r);
            ret.push('A');
        } else {
            b(&mut x, p, q, r);
            ret.push('B');
        }
    }
    ret
}

// ビームサーチ
#[allow(dead_code)]
fn f2(pqr: &[(usize, usize, usize)]) -> Vec<char> {
    #[derive(Clone, Debug)]
    struct S {
        score: usize,
        x: Vec<i64>,
        op: char,
        index: usize,
    }
    impl S {
        fn next(&self, index: usize, op: char, p: usize, q: usize, r: usize) -> S {
            let mut x = self.x.clone();
            match op {
                'A' => {
                    x[p] += 1;
                    x[q] += 1;
                    x[r] += 1;
                }
                'B' => {
                    x[p] -= 1;
                    x[q] -= 1;
                    x[r] -= 1;
                }
                _ => unreachable!(),
            }
            let score = x.iter().filter(|a_i| a_i == &&0).count();
            Self {
                score,
                x,
                op,
                index,
            }
        }
    }
    impl Default for S {
        fn default() -> Self {
            Self {
                score: 0,
                x: vec![0_i64; 20],
                op: 'A',
                index: 0,
            }
        }
    }
    // ビーム幅
    let width = 10_000_usize;
    let mut beam = vec![vec![]; pqr.len() + 1];
    beam[0].push(S::default());
    for (i, (p, q, r)) in pqr.iter().copied().enumerate() {
        let mut next = vec![];
        for (j, b) in beam[i].iter().enumerate() {
            next.push(b.next(j, 'A', p, q, r));
            next.push(b.next(j, 'B', p, q, r));
        }
        next.sort_by_key(|a| a.score);
        beam[i + 1].extend(next.into_iter().rev().take(width));
    }

    let mut ret = vec![];
    let mut index = 0;
    for b in beam.iter().rev().take(pqr.len()) {
        ret.push(b[index].op);
        index = b[index].index;
    }
    ret.reverse();

    // assert
    let mut s = S::default();
    for (i, (p, q, r)) in pqr.iter().copied().enumerate() {
        s = s.next(0, ret[i], p, q, r);
    }
    assert_eq!(s.score, beam[pqr.len()][0].score);

    ret
}

fn main() {
    input! {
        t: usize,
        pqr: [(Usize1, Usize1, Usize1); t],
    };
    // let ans = f1(&pqr); // 37454
    let ans = f2(&pqr); // 46073
    for a in ans {
        println!("{}", a);
    }
}
