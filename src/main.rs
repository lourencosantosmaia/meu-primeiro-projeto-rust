use std::io::{self, Write};
use std::fs::{self, OpenOptions};

fn main() {
    println!("=== GESTÃO ESCOLAR RUST v2.0 ===");

    loop {
        println!("\nEscolha uma opção:");
        println!("1. Lançar nova nota");
        println!("2. Ver relatório completo");
        println!("3. Calcular média da turma");
        println!("4. Sair");

        let mut opcao = String::new();
        io::stdin().read_line(&mut opcao).expect("Erro ao ler opção");

        match opcao.trim() {
            "1" => lancar_nota(),
            "2" => gerar_relatorio(),
            "3" => calcular_estatisticas(),
            "4" => {
                println!("Encerrando sistema... Até logo, Professor Lourenço!");
                break;
            }
            _ => println!("⚠️ Opção inválida!"),
        }
    }
}

// ... (as funções lancar_nota e gerar_relatorio continuam as mesmas) ...

fn calcular_estatisticas() {
    let conteudo = fs::read_to_string("notas_alunos.txt").unwrap_or_default();
    let mut soma = 0.0;
    let mut contador = 0;

    for linha in conteudo.lines() {
        // Buscamos a nota entre "Nota: " e " | Status"
        if let Some(pos_nota) = linha.find("Nota: ") {
            let inicio = pos_nota + 6;
            if let Some(fim) = linha[inicio..].find(" |") {
                let nota_str = &linha[inicio..inicio + fim];
                if let Ok(nota) = nota_str.parse::<f64>() {
                    soma += nota;
                    contador += 1;
                }
            }
        }
    }

    if contador > 0 {
        let media = soma / contador as f64;
        println!("\n--- ESTATÍSTICAS ---");
        println!("Total de alunos: {}", contador);
        println!("Média geral da turma: {:.2}", media);
    } else {
        println!("⚠️ Nenhum dado para calcular.");
    }
}

fn lancar_nota() {
    println!("\nNome do aluno:");
    let mut nome = String::new();
    io::stdin().read_line(&mut nome).expect("Erro ao ler nome");
    let nome = nome.trim();

    println!("Nota de {}:", nome);
    let mut entrada_nota = String::new();
    io::stdin().read_line(&mut entrada_nota).expect("Erro ao ler nota");

    let nota: f64 = entrada_nota.trim().parse().unwrap_or(0.0);
    let status = if nota >= 7.0 { "APROVADO" } else { "REPROVADO" };
    
    let linha = format!("Aluno: {} | Nota: {} | Status: {}\n", nome, nota, status);

    let mut arquivo = OpenOptions::new()
        .create(true).append(true).open("notas_alunos.txt").expect("Erro ao abrir arquivo");

    arquivo.write_all(linha.as_bytes()).expect("Erro ao salvar");
    println!("✅ Registro salvo!");
}

fn gerar_relatorio() {
    println!("\n--- RELATÓRIO DE ALUNOS ---");
    match fs::read_to_string("notas_alunos.txt") {
        Ok(conteudo) => println!("{}", conteudo),
        Err(_) => println!("⚠️ Nenhum registro encontrado."),
    }
}