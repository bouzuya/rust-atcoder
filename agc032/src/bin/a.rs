use proconio::input;

fn main() {
    input! {
        n: usize,
        b: [usize; n],
    };
    let mut ans = std::collections::VecDeque::new();
    let mut bd = b.into_iter().collect::<std::collections::VecDeque<_>>();
    for _ in 0..n {
        let mut found = None;
        for (j, &b_i) in bd.iter().enumerate().rev() {
            if j + 1 == b_i {
                found = Some(j);
                ans.push_front(b_i);
                break;
            }
        }
        match found {
            Some(j) => {
                bd.remove(j);
            }
            None => {
                println!("-1");
                return;
            }
        }
    }
    for &a_i in ans.iter() {
        println!("{}", a_i);
    }
}
