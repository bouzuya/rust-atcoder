use proconio::input;

// c
// b
// a
fn f1(x: usize, y: usize, a: usize, b: usize, c: usize) -> bool {
    ((a + x - 1) / x) + ((b + x - 1) / x) + ((c + x - 1) / x) <= y
}

// b c
// a c
fn f2(x: usize, y: usize, a: usize, b: usize, c: usize) -> bool {
    let c_x = (c + y - 1) / y;
    if c_x >= x {
        return false;
    }
    let x = x - c_x;
    ((a + x - 1) / x) + ((b + x - 1) / x) <= y
}

fn main() {
    input! {
        x: usize,
        y: usize,
        a: usize,
        b: usize,
        c: usize,
    };

    if a >= x * y || b >= x * y || c >= x * y {
        println!("No");
        return;
    }

    if f1(x, y, a, b, c) || f1(y, x, a, b, c) {
        println!("Yes");
        return;
    }

    if f2(x, y, a, b, c)
        || f2(x, y, a, c, b)
        || f2(x, y, b, c, a)
        || f2(y, x, a, b, c)
        || f2(y, x, a, c, b)
        || f2(y, x, b, c, a)
    {
        println!("Yes");
        return;
    }

    println!("No");
}
