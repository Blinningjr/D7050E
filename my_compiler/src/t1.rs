

fn tio(i: i32) -> i32 {
    if (i < 50) {
        return tio(i + 1);
    } 
    else{
        return i;       
    }
}

fn main() {
    let mut a: i32 = 0; 
    tio(a);
    return &1;
}
