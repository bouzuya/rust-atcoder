use proconio::input;

fn main() {
    input! {
        q: usize,
    };

    let mut stack = vec![];
    for _ in 0..q {
        input! {
            t: usize
        }
        match t {
            1 => {
                input! {
                    x: String
                }
                stack.push(x);
            }
            2 => {
                println!("{}", stack.last().unwrap());
            }
            3 => {
                stack.pop();
            }
            _ => unreachable!(),
        }
    }
}
