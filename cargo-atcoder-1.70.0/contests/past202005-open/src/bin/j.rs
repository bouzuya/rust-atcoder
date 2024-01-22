use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        a: [usize; m],
    };
    let mut c = vec![0; n];
    let mut ans = vec![];

    for a_i in a {
        if c[0] < a_i {
            c[0] = a_i;
            ans.push(0);
            continue;
        }
        let mut ok = 0;
        let mut ng = n;
        while ng - ok > 1 {
            let mid = (ok + ng) / 2;
            if c[mid] >= a_i {
                ok = mid;
            } else {
                ng = mid;
            }
        }
        if ng == n {
            ans.push(-1);
        } else {
            c[ng] = a_i;
            ans.push(ng as i64);
        }
    }
    for a in ans {
        println!("{}", if a == -1 { -1 } else { a + 1 });
    }
}
