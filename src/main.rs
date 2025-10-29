fn main(){
    let mut counter = 10;
    let msg = String::from("Liftoff!!!!!");

    while counter >= 0 {
        println!("{}", counter);
        if counter == 0 { println!("{}", msg); }
        counter -= 1;
    }
}

