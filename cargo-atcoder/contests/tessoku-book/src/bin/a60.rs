use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    };
    let mut stack = vec![];
    for (d, a_d) in a.iter().copied().enumerate() {
        let mut found = None;
        while let Some((prev, i)) = stack.pop() {
            if prev > a_d {
                found = Some((prev, i));
                break;
            }
        }
        match found {
            None => {
                println!("-1");
            }
            Some((p, i)) => {
                println!("{}", i + 1);
                stack.push((p, i));
            }
        }
        stack.push((a_d, d));
    }
}
