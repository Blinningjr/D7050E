fn ten(num: i32) -> () {
    *num = 11;
}


fn main() -> i32 {
    let mut answer: i32 = 0;
    ten(answer);
    return *answer;
}
