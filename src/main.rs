fn main(){
    let a = 1;
    let b = 2;
    let c = 3;

    let res = max_numb(a, b,  c);

    println!("The largerst number is: {}", res);
}

fn max_numb(a: i32, b: i32, c: i32) -> i32 {
    if a > b {
        if a > c { a } else { c }
    } else {
        if b > c { b } else { c }
    }
}


