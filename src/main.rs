fn main(){
    let number = 4;

    let days = ["Lunes", "Martes", "Miercoles", "Jueves", "Viernes", "Sabado", "Domingo"];

    let res = days.get(number - 1);
    match res {
        Some(day) => println!("El dia es: {}", day),
        None      => println!("Error..."),
    }
}

