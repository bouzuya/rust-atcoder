use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [i64; 3 * n],
    };
    let mut pq_0 = std::collections::BinaryHeap::new();
    let mut pq_1 = std::collections::BinaryHeap::new();
    let mut s_0 = 0_i64;
    let mut s_1 = 0_i64;
    for i in 0..n {
        let a_i = a[i];
        pq_0.push(-a_i);
        s_0 += a_i;

        let a_j = a[3 * n - i - 1];
        pq_1.push(a_j);
        s_1 -= a_j;
    }

    let mut c_0 = vec![s_0];
    for i in n..2 * n {
        let a_l = a[i];
        pq_0.push(-a_l);
        s_0 += a_l;
        let m_0 = pq_0.pop().unwrap();
        s_0 -= -m_0;
        c_0.push(s_0);
    }

    let mut c_1 = vec![s_1];
    for i in (n..2 * n).rev() {
        let a_r = a[i];
        pq_1.push(a_r);
        s_1 -= a_r;
        let m_1 = pq_1.pop().unwrap();
        s_1 += m_1;
        c_1.push(s_1);
    }

    let mut ans = -1_000_000_000_000_000_i64;
    for (&s_0, &s_1) in c_0.iter().zip(c_1.iter().rev()) {
        ans = std::cmp::max(ans, s_0 + s_1);
    }

    println!("{}", ans);
}
