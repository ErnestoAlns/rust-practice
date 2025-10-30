//    Usa match con rangos para imprimir:
//    0–12 → "Niño"
//    13–17 → "Adolescente"
//    18–64 → "Adulto"
//    65..=u8::MAX → "Adulto mayor"


fn main(){
    let edad = 24;
    let stat = match &edad {
        0..=12 => "Niño",
        13..=17 => "Adolescente",
        18..=64 => "Adulto",
        65..=100 => "Adulto mayor",
        _ => "Error no valido",
    };

    println!("Tu edad es: {}, entonces eres un {}", edad, stat);
}

