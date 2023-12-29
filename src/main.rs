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
    // Me explique o guess.cmp(&numero_secreto)
    // O cmp() compara dois valores e retorna uma enumeração
// que diz se o valor foi menor, maior ou igual ao outro.
// A enumeração é Ordering e tem três valores possíveis:
// Less, Greater e Equal.
}

fn generate_stars(random_number: i32) -> String {
    let mut star = String::new();
    for _ in 0..random_number {
        star.push_str(" * ");
    }
}

fn main() {
    let random_number = rand::thread_rng().gen_range(1..=66);
    println!("Conte as estrelas!");
    // Eu quero que ele so receba input por 5 segundos
    // Se ele não responder, eu quero que o programa termine
    // Se ele responder, eu quero que o programa retorne uma string
    // com o numero de estrelas que ele contou
    println!("Por favor, insira seu palpite.");
    println!("Voce tem 5 segundos");

    let mut guess = String::new();
    
    io::stdin()
        .read_line(&mut guess)
        .expect("Falha ao ler a linha");

    println!("Você digitou: {guess}");
}

