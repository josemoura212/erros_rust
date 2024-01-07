use std::fs::File;
use std::io::{self, Read};

fn main() {
    // funcao_com_panic(0);

    let resultado = std::panic::catch_unwind(|| {
        let a = funcao_com_panic(10);

        Ok::<i32, &str>(a)
    });

    match resultado {
        Ok(valor) => println!("Tudo certo! {}", valor.unwrap()),
        Err(_) => println!("Deu ruim!"),
    }

    println!("\n====================\n");

    let resultado = ler_arquivo(r"C:\Projetos\OneDrive\Rust\erros\src\main.rs");

    match resultado {
        Ok(conteudo) => println!("Conteúdo do arquivo: {}", conteudo),
        Err(erro) => println!("Erro ao ler arquivo: {}", erro),
    }
    println!("\n====================\n");

    let resultado_divisao = dividir(100.0, 25.0);

    match resultado_divisao {
        Some(valor) => println!("Resultado divisao: {}", valor),
        None => println!("Não foi possivel fazer a divisão"),
    }
}

fn funcao_com_panic(valor: i32) -> i32 {
    if valor == 0 {
        panic!("Vai dar ruim!");
    }
    valor
}

fn ler_arquivo(caminho: &str) -> Result<String, io::Error> {
    let mut arquivo = File::open(caminho)?;

    let mut conteudo = String::new();

    let _ = arquivo.read_to_string(&mut conteudo);

    Ok(conteudo)
}

fn dividir(dividendo: f64, divisor: f64) -> Option<f64> {
    if divisor == 0.0 {
        None
    } else {
        Some(dividendo / divisor)
    }
}
