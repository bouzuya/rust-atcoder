use proconio::input;

fn add(
    pq: &mut std::collections::BinaryHeap<(i64, (usize, usize, usize))>,
    set: &mut std::collections::BTreeSet<(usize, usize, usize)>,
    a: &Vec<i64>,
    b: &Vec<i64>,
    c: &Vec<i64>,
    i_a: usize,
    i_b: usize,
    i_c: usize,
) {
    if i_a >= a.len() || i_b >= b.len() || i_c >= c.len() {
        return;
    }
    let i = (i_a, i_b, i_c);
    if set.insert(i) {
        pq.push((a[i_a] + b[i_b] + c[i_c], i));
    }
}

fn main() {
    input! {
        x: usize,
        y: usize,
        z: usize,
        k: i64,
        mut a: [i64; x],
        mut b: [i64; y],
        mut c: [i64; z],
    };
    a.sort_by_key(|&a_i| std::cmp::Reverse(a_i));
    b.sort_by_key(|&b_i| std::cmp::Reverse(b_i));
    c.sort_by_key(|&c_i| std::cmp::Reverse(c_i));
    let mut set = std::collections::BTreeSet::new();
    let mut pq = std::collections::BinaryHeap::new();
    add(&mut pq, &mut set, &a, &b, &c, 0, 0, 0);
    for _ in 0..k {
        let (ans, (i_a, i_b, i_c)) = pq.pop().unwrap();
        add(&mut pq, &mut set, &a, &b, &c, i_a + 1, i_b, i_c);
        add(&mut pq, &mut set, &a, &b, &c, i_a, i_b + 1, i_c);
        add(&mut pq, &mut set, &a, &b, &c, i_a, i_b, i_c + 1);
        println!("{}", ans);
    }
}
