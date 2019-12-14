fn main() -> i32 {
    let mut a: &i32 = &10;
    if true {
        let b: i32 = 2;
        a = &b;
    }
    return *a;
}