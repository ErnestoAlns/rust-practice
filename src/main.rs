fn main(){
    let couter = 5;
    printer(&couter);
}

fn printer(a: & i32){ 
    let mut x = 0;
    loop{
        println!("{}", x);
        x += 1;
        if x > *a { break; }
    }
}
