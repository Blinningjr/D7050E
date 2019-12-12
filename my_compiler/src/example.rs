fn ten(num: &mut i32) -> () {
    if *num != 10 {
        *num = (*num%5 * 2) + 1;
        ten(num);
    }
}

fn main() -> i32 {
    let mut answer: i32 = 9;
    ten(&mut answer);
    if answer == 10 {
        return -answer;
    } else {
        return answer;
    }
}