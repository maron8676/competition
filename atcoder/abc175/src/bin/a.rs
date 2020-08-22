use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        s: String,
    }

    if s == "SSS" {
        println!("0");
    } else if s == "RSS" || s == "SRS" || s == "SSR" || s == "RSR" {
        println!("1");
    } else if s == "RRS" || s == "SRR" {
        println!("2");
    } else {
        println!("3");
    }
}
