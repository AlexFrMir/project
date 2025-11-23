use std::io;
use rand::Rng;

fn main() {
    println!("Adivina el número!");

   let secret_number = rand::thread_rng().gen_range(1..=100);

   println!("El número secreto es: {secret_number}");

    println!("Por favor, ingresa tu suposición.");

    let mut guess = String::new();

    io::stdin()
     .read_line(&mut guess)
     .expect("Falló al leer la línea");

    println!("Has adivinado: {guess}");
}