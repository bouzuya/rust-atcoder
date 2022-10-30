use proconio::input;

fn main() {
    input! {
        mut x: usize,
        mut y: usize,
    };
    let mut ans = vec![];
    ans.push((x, y));
    while !(x == 1 && y == 1) {
        if x > y {
            let d = x - y;
            x = d;
            ans.push((x, y));
        } else {
            let d = y - x;
            y = d;
            ans.push((x, y));
        }
    }
    ans.pop();
    ans.reverse();
    println!("{}", ans.len());
    for (x, y) in ans {
        println!("{} {}", x, y);
    }
}
