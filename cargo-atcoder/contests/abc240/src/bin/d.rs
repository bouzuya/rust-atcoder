use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    };

    let mut ans = vec![];
    let mut size = 0_usize;
    let mut stack = vec![];
    for a_i in a {
        if let Some((x, count)) = stack.pop() {
            if x == a_i {
                if count + 1 == a_i {
                    size -= count;
                } else {
                    stack.push((x, count + 1));
                    size += 1;
                }
            } else {
                stack.push((x, count));
                stack.push((a_i, 1_usize));
                size += 1;
            }
        } else {
            stack.push((a_i, 1_usize));
            size += 1;
        }
        ans.push(size);
    }

    for a in ans {
        println!("{}", a);
    }
}
