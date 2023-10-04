use std::{io::Write, process::exit};
use rand::Rng;

fn retira_caracter_aleatorio(s: &mut String) -> char{
    let digito_index = rand::thread_rng().gen_range(0..s.len());
    let c = s.chars().nth(digito_index).unwrap();
    s.remove(digito_index);
    c

    // s.remove(rand::thread_rng().gen_range(0..s.len()))
}

fn sorteia_senha(simbolos: &String) -> String{
    let mut senha = String::new();
    let mut s = simbolos.clone();
    while s.len() > 0 {
        senha.push(retira_caracter_aleatorio(&mut s));
    }
    senha
}

fn possiveis_simbolos(n_simbolos: u8) -> Result<String, String>{
    let mut digitos = String::new();
    for c in 'A'..'Z'{
        digitos.push(c);
        if digitos.len() == usize::from(n_simbolos) {
            return Ok(digitos);
        }
    }
    Err("n_digitos muito grande".to_string())
}

fn valida_entrada_usuario(simbolos: &String, palpite: &str) -> Result<String, String>{
    if simbolos.len() > palpite.len(){
        return Err(format!("A string tem um número de caracteres menor que o desejado ({}).", simbolos.len()));
    }
    if simbolos.len() < palpite.len(){
        return Err(format!("A string tem um número de caracteres maior que o desejado ({}).", simbolos.len()));
    }

    // Verifica se palpite tem os caracteres corretos
    let mut palpite_clone = String::from(palpite);
    for (_, c) in simbolos.chars().enumerate(){
        let t = palpite_clone.len();
        palpite_clone = palpite_clone.replace(c, "");
        match t - palpite_clone.len(){
            0 => {
                return Err(format!("Está faltando o caractere '{}'.", c));
            },
            1 => {},
            _ => {
                return Err(format!("A string tem caracter repetido ({}).", c));
            }
        }
    }
    Ok(String::from(palpite))
}

const N_DIGITOS: u8 = 4;

fn main() {
    // Lista os possíveis simbolos
    let simbolos = match possiveis_simbolos(N_DIGITOS) {
        Ok(v) => v,
        Err(message) => {
            println!("Erro: {}", message);
            exit(1);
        }
    };

    // Sortei a sequencia não repetida
    let senha = sorteia_senha(&simbolos);
    println!("{}",senha);

    'loop_principal: for tentativa in 1..12{
        let mut palpite = String::new();
        loop {
            print!("Entre com sua tentativa (#{}): ",tentativa);
            std::io::stdout().flush().unwrap();
            palpite.clear();
            std::io::stdin().read_line(&mut palpite).unwrap();
            match valida_entrada_usuario(&simbolos, palpite.trim()){
                Err(msg) => {
                    println!("{}",msg);
                }
                Ok(palpite) => {
                    let mut no_lugar: u8 = 0;
                    let mut fora_do_lugar: u8 = 0;
                    for index in 0..senha.len(){
                        if senha.chars().nth(index) == palpite.chars().nth(index){
                            no_lugar += 1;
                        }else {
                            fora_do_lugar += 1;
                        }
                    }
                    if no_lugar == N_DIGITOS{
                        println!("Parabéns!!! Você arcertou a senha com {} tentativa(s)!!!", tentativa);
                        break 'loop_principal;
                    }
                    print!("  No lugar: {}", no_lugar);
                    println!("  Fora do lugar: {}\n", fora_do_lugar);
                break;
                },
            }
        }
    }

}
