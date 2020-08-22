use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        x: i64,
        k: i64,
        d: i64
    }

    let abs_x = x.abs();
    if abs_x / d >= k {
        println!("{}", abs_x - d * k)
    } else {
        let is_even = (k - abs_x / d) % 2 == 0;
        let remains = abs_x % d;
        if is_even {
            println!("{}", remains);
        } else {
            println!("{}", d - remains)
        }
    }
}
