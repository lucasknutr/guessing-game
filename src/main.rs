use std::io;

fn main() {
    println!("Adivinhe o número!");

    println!("Por favor, insira seu palpite.");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Falha ao ler a linha");

    println!("Você digitou: {guess}");

    let mut dablio = String::new();
    io::stdin()
        .read_line(&mut dablio)
        .expect("Falha ao ler a linha");
    println!("Você digitou: {dablio}");
}
