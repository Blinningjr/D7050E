fn tio(i: i32) -> i32 {
    let a = 200; 
    if i < 10 {
        tio(i + 1)
        } 
    else{
        i
    }
}

fn main() -> None {
    let a = 100; 
    tio(1);
}
