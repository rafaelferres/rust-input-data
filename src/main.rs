use std::io;

fn convertToInt(dataInput: &str) -> i32 {
    let x = dataInput.trim().parse::<i32>().unwrap();
    x
}

fn main() {
    let mut number1 = String::new();
    let mut number2 = String::new();
    io::stdin()
        .read_line(&mut number1)
        .expect("Erro ao ler number1");

    io::stdin()
        .read_line(&mut number2)
        .expect("Erro ao ler number2");

    let number1Convert: i32 = convertToInt(&number1);
    let number2Convert: i32 = convertToInt(&number2);

    if number1Convert > number2Convert {
        println!(
            "O número {} é maior que o número {}",
            number1.trim(),
            number2.trim()
        )
    } else {
        println!(
            "O número {} é menor ou igual que o número {}",
            number1.trim(),
            number2.trim()
        )
    }
}
