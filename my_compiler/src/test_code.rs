fn tio(i: i32) -> i32 {
    if i < 0 {
        return tio(i + 1);
    } 
    else{
        return i;       
    }
    999
}

fn main() -> None {
    let a = 1; 
    tio(1);
}
