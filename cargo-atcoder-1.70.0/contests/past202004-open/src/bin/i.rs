use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; 2_usize.pow(n as u32)],
    };
    let mut ans = vec![0_usize; 2_usize.pow(n as u32)];
    let mut ai = a
        .iter()
        .copied()
        .enumerate()
        .collect::<Vec<(usize, usize)>>();
    for round in 1..=n {
        let mut next = vec![];
        for i in (0..ai.len()).step_by(2) {
            if ai[i].1 < ai[i + 1].1 {
                ans[ai[i].0] = round;
                next.push(ai[i + 1]);
            } else {
                ans[ai[i + 1].0] = round;
                next.push(ai[i]);
            }
        }
        ai = next;
        if ai.len() == 1 {
            ans[ai[0].0] = round;
        }
    }
    for a in ans {
        println!("{}", a);
    }
}
