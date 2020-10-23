use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [i64; 1 << n],
    };
    let mut ans = vec![0; 1 << n];
    let mut dc = std::collections::VecDeque::new();
    for (i, &a_i) in a.iter().enumerate() {
        dc.push_back((a_i, i));
    }
    let mut dn = std::collections::VecDeque::new();
    for i in 1..=n {
        for _ in 0..dc.len() / 2 {
            let (l, i_l) = dc.pop_front().unwrap();
            let (r, i_r) = dc.pop_front().unwrap();
            dn.push_back(if l > r { (l, i_l) } else { (r, i_r) });
            if l > r {
                ans[i_r] = i;
            } else {
                ans[i_l] = i;
            }
        }
        dc = dn;
        dn = std::collections::VecDeque::new();
    }
    let (_, i) = dc.pop_front().unwrap();
    ans[i] = n;
    for &a_i in ans.iter() {
        println!("{}", a_i);
    }
}
