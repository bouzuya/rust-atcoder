use proconio::input;

fn main() {
    input! {
        n: usize,
        h: [i64; n],
    };

    let mut ans = vec![];
    let mut stack = vec![];
    for &h_i in h.iter() {
        ans.push(stack.len());
        while let Some(x) = stack.pop() {
            if x > h_i {
                stack.push(x);
                break;
            }
        }
        stack.push(h_i);
    }
    for a in ans {
        println!("{}", a);
    }
}
