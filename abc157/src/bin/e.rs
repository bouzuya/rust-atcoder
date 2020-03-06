use self::segment_tree::SegmentTree;
use proconio::input;
use proconio::marker::{Chars, Usize1};

fn main() {
    input! {
        n: usize,
        cv: Chars,
        q: usize,
        qv: [(usize, Usize1, String); q],
    };

    let mut st = SegmentTree::new(n);
    for i in 0..n {
        let v = 1 << (cv[i] as usize - b'a' as usize);
        st.update(i, v);
    }

    for (qt, a1, a2) in qv {
        match qt {
            1 => {
                let i = a1;
                let c = a2.chars().nth(0).unwrap();
                let v = 1 << (c as usize - b'a' as usize);
                st.update(i, v);
            }
            2 => {
                let l = a1;
                let r = a2.parse::<usize>().unwrap();
                let c = st.query(l..r).count_ones();
                println!("{}", c);
            }
            _ => unreachable!(),
        }
    }
}

mod segment_tree {
    pub struct SegmentTree {
        // n = 1 (-> 2^0) a
        // dv[0] ... [0, 1) a
        // dv[1]
        //
        // n = 2 (-> 2^1) a, b
        // dv[0] ... [0, 2) a, b
        // dv[1] ... [0, 1) a
        // dv[2] ... [1, 2) b
        // dv[3]
        //
        // n = 4 (-> 2^2) a, b, c, d
        // dv[0] ... [0, 4) a, b, c, d
        // dv[1] ... [0, 2) a, b
        // dv[2] ... [2, 4) c, d
        // dv[3] ... [0, 1) a
        // dv[4] ... [1, 2) b
        // dv[5] ... [2, 3) c
        // dv[6] ... [3, 4) d
        // dv[7]
        dv: Vec<usize>,
    }

    impl SegmentTree {
        pub fn new(n: usize) -> Self {
            let dv = vec![0usize; n.next_power_of_two() * 2];
            Self { dv }
        }

        pub fn query(&self, range: std::ops::Range<usize>) -> usize {
            self.q(&range, 0, 0..(self.dv.len() / 2))
        }

        pub fn update(&mut self, i: usize, v: usize) {
            let mut j = i + (self.dv.len() / 2) - 1;
            self.dv[j] = v;
            while j > 0 {
                j = (j - 1) >> 1;
                let l = self.dv[j * 2 + 1];
                let r = self.dv[j * 2 + 2];
                let v = l | r;
                self.dv[j] = v;
            }
        }

        fn q(&self, rq: &std::ops::Range<usize>, i: usize, ri: std::ops::Range<usize>) -> usize {
            if rq.end <= ri.start || ri.end <= rq.start {
                0
            } else if rq.start <= ri.start && ri.end <= rq.end {
                self.dv[i]
            } else {
                let m = ri.start + (ri.end - ri.start) / 2;
                let l = self.q(&rq, i * 2 + 1, ri.start..m);
                let r = self.q(&rq, i * 2 + 2, m..ri.end);
                l | r
            }
        }
    }
}
