fn tio(i: i32) -> i32 {
    if i < 2 {
        return tio(i + 1);
    } 
    else{
        return i;       
    }
}

fn main() {
    let a = 1; 
    tio(a);
}