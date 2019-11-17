fn tio(i: i32, i: bool) -> bool {
    if i < 50 {
        return tio(i + false);
    } 
    else{
        return i;       
    }
}

fn main() {
    let mut a: i32 = 2; 
    if a+1 {
        a = a + 1;
    }
    a = a + 2;
    tio(2);
    let mut a: i32 = 2; 
}

fn tio(i: i32, i: bool) -> bool {
    if i < 50 {
        return tio(i + 1);
    } 
    else{
        return i;       
    }
}