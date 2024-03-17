use std::fs::File;
use std::io::{BufReader, stdin};
use std::path::Path;

fn main() {

    let tamanho = 12;
    let mut soma = 0;
    let mut contador = 0;
    let mut linha = 1;
    let mut coluna = 11;
    let mut o: String = String::new();
    let path = Path::new("./data/matriz.json");
    let file = File::open(&path).unwrap();
    let reader = BufReader::new(file);
    let matriz: Vec<Vec<i32>> = serde_json::from_reader(reader).unwrap();

    println!("Matriz: {:?}", matriz);

    println!("Digite a operação desejada: ");

    print!("Soma S ou Média M => ");
    stdin().read_line(&mut o).unwrap();

    let o = o.trim();

    while linha < (tamanho - 1) {
        println!("linha: {}, Coluna {}", linha, coluna);

        soma += matriz[linha][coluna];
        contador += 1;

        linha += 1;

        match linha {
            1..=5 => coluna -= 1,
            6=> coluna += 0,
            _ => coluna += 1
        }
    }

    if o == "M" {
        println!("Média: {}", soma / contador);
    } else {
        println!("Soma: {}", soma);
    }

}
