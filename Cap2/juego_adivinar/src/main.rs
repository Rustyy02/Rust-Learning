use std::cmp::Ordering;
use std::io;
use rand::Rng;

fn main() {
    println!("Adivina el número!");

    let numero_secreto = rand::thread_rng().gen_range(1..=100);

    println!("El número secreto es: {numero_secreto}");

    loop {
        println!("Por favor ingresa tu número.");

        let mut adivinar = String::new();

        io::stdin()
            .read_line(&mut adivinar)
            .expect("Error al leer la línea");

        let adivinar: u32 = match adivinar.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Ingrese solo valores numéricos");
                continue;
            }
        };

        println!("Elegiste: {adivinar}");

        println!("Ingrese su número");
    
        match adivinar.cmp(&numero_secreto) {
            Ordering::Less => {
                println!("Muy bajo!!");
                break;
            }
            Ordering::Greater => {
                println!("Muy alto!!");
                break;
            }
            Ordering::Equal => {
                println!("Ganaste!");
                break;
            }
        }
    }
}
