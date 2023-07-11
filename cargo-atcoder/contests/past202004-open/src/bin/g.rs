use proconio::input;

fn main() {
    input! { q: usize };
    let mut deque = std::collections::VecDeque::new();
    for _ in 0..q {
        input! { t: usize };
        match t {
            1 => {
                input! { c: char, x: usize };
                deque.push_back((c, x));
            }
            2 => {
                input! { mut d: usize };
                let mut count = vec![0; 26];
                while let Some((c, x)) = deque.pop_front() {
                    if d <= x {
                        count[(c as u8 - 'a' as u8) as usize] += d;
                        deque.push_front((c, x - d));
                        break;
                    }
                    count[(c as u8 - 'a' as u8) as usize] += x;
                    d -= x;
                }
                let ans = count.iter().map(|&c_i| c_i.pow(2)).sum::<usize>();
                println!("{}", ans);
            }
            _ => unreachable!(),
        }
    }
}
