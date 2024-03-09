use proconio::input;

fn main() {
    let mut ans = vec![];
    loop {
        input! {
            a_i: usize
        }

        ans.push(a_i);
        if a_i == 0 {
            break;
        }
    }
    ans.reverse();
    for a in ans {
        println!("{}", a);
    }
}
