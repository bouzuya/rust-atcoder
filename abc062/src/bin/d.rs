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

    let mut l = n;
    let mut r = 2 * n;
    for _ in 0..n {
        let a_l = a[l];
        let a_r = a[r - 1];
        let &m_0 = pq_0.peek().unwrap();
        let &m_1 = pq_1.peek().unwrap();
        let score_0 = m_0 + a_l;
        let score_1 = m_1 - a_r;
        if score_0 >= score_1 {
            pq_0.pop();
            pq_0.push(-a_l);
            s_0 += score_0;
            l += 1;
        } else {
            pq_1.pop();
            pq_1.push(a_r);
            s_1 += score_1;
            r -= 1;
        }
    }

    let ans = s_0 + s_1;
    println!("{}", ans);
}
