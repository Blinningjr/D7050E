fn tio(i: i32) -> i32 {
    if i < 1 {
        return tio(i + 1);
    } 
    else{
        return i;       
    }
}

fn main() -> None {
    let a = 1; 
    tio(1);
}