use proconio::input;

fn main() {
    input! {
        n: usize,
        mut a: [usize; n],
    };
    a.push(0);
    let mut max = 0_usize;
    let mut stack: Vec<(usize, usize)> = vec![];
    for (i, a_i) in a.iter().copied().enumerate() {
        let mut left_index = i;
        while let Some((l, a_l)) = stack.pop() {
            match a_l.cmp(&a_i) {
                std::cmp::Ordering::Less => {
                    stack.push((l, a_l));
                    break;
                }
                std::cmp::Ordering::Equal | std::cmp::Ordering::Greater => {
                    left_index = l;
                    max = max.max(a_l * (i - left_index));
                }
            }
        }
        stack.push((left_index, a_i));
    }
    let ans = max;
    println!("{}", ans);
}
