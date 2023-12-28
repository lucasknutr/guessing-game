use std::io;
use rand::Rng;

fn main() {
    println!("Adivinhe o número!");

    println!("Por favor, insira seu palpite.");
    
    let numero_secreto = rand::thread_rng().gen_range(1..=100);

    println!("O número secreto é: {}", numero_secreto);

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Falha ao ler a linha");

    println!("Você digitou: {guess}");

    match guess.cmp(&numero_secreto) {
        Ordering::Less => println!("Muito pequeno!"),
        Ordering::Greater => println!("Muito grande!"),
        Ordering::Equal => println!("Você ganhou!"),
    }
}
