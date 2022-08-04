fn main() {
    array();
    array_for();
    matriz();
    eh_fim_de_semana(DiaDaSemana::Quarta);
    cores();
    conteudo_opcional();
    vectors();
    conta_corrente();
}

/* uso de array */
fn array() {
    let notas: [f32; 4] = [10f32, 8f32, 9.5, 6.0];
    let inteiro: usize = 0;

    print!("A nota é {}", notas[inteiro]);

    println!(
        "Nota1 = {}, Nota2 = {}, Nota3 = {}, Nota4 = {}",
        notas[0], notas[1], notas[2], notas[3]
    );
}

/* iterando array */
fn array_for() {
    let notas: [f32; 4] = [10f32, 8f32, 9.5, 6.0];

    // iterando itens
    for nota in notas {
        println!("A nota é = {}", nota);
    }

    // iterando itens com suporte de índice
    for indice in 0..notas.len() {
        println!("A nota {} é = {}", indice + 1, notas[indice]);
    }
}

// uso de matrizes multi-dimensionais
fn matriz() {
    let matriz: [[f32; 3]; 2] = [[0.0, 1.2, 0.1], [1.3, 0.3, 1.4]];

    for linha in matriz {
        for item in linha {
            println!("Item = {}", item);
        }
    }
}

/* uso de enums */
#[allow(dead_code)]
enum DiaDaSemana {
    Domingo,
    Segunda,
    Terca,
    Quarta,
    Quinta,
    Sexta,
    Sabado,
}

/* tratando enum */
fn eh_fim_de_semana(dia_da_semana: DiaDaSemana) {
    let result = match dia_da_semana {
        DiaDaSemana::Domingo | DiaDaSemana::Sabado => true,
        _ => false,
    };

    println!("É fim de semana? {}", result);
}

/* uso de enums com função */
#[allow(dead_code)]
enum Color {
    Red,
    Green,
    Blue,
    RbgColor(u8, u8, u8),
    CymkColor {
        cyan: u8,
        magenta: u8,
        yellow: u8,
        black: u8,
    },
}

fn cores() {
    let cor = Color::RbgColor(12, 5, 32);

    println!(
        "Cor = {}",
        match cor {
            Color::Red => "vermelho",
            Color::Green => "verde",
            Color::Blue => "blue",
            Color::RbgColor(0, 0, 0)
            | Color::CymkColor {
                cyan: _,
                magenta: _,
                yellow: _,
                black: 255,
            } => "preta",
            Color::RbgColor(_, green, _) => {
                println!("{}", green);
                "RGB desconhecido"
            }
            Color::CymkColor {
                cyan: _,
                magenta: _,
                yellow: _,
                black: _,
            } => "CYMK desconhecido",
        }
    );
}

fn conteudo_opcional() {
    let conteudo_arquivo = ler_arquivo(String::from(""));

    match &conteudo_arquivo {
        Some(value) => println!("{}", value),
        None => println!("Arquivo não encontrado"),
    };

    if let Some(value) = conteudo_arquivo {
        println!("Agora tenho certeza de que ha o valor {}", value)
    }
}

fn ler_arquivo(_caminho_arquivo: String) -> Option<String> {
    Some(String::from("conteudo do arquivo"))
    //None
}

/* uso de vetores (remete a um List<T> do C#) */
fn vectors() {
    // criando de forma dinâmica
    #[allow(unused)]
    let mut notas: Vec<f32> = Vec::new();

    // criando usando uma macro
    //notas = vec![11.0, 8.8, 6.5];

    // criando informando a capacidade
    // para evitar realocação de memória
    notas = Vec::with_capacity(4);
    notas.push(10.0);
    notas.push(8.8);
    notas.push(6.5);

    println!("Capacidade = {}", notas.capacity());

    notas.push(6.8);
    println!("Capacidade = {}", notas.capacity());

    println!("{:?}", notas);
    println!("Nota {}", notas[0]);

    // O 'n' de Some(n) na verdade é uma referência para um item
    // dentro do vetor. Para desvicular ele basta usar o '*'
    // sem o '*' dá erro de compilação de imcompatibilidade de tipos
    println!(
        "Nota 6 = {}",
        match notas.get(7) {
            Some(n) => *n,
            None => 0.0,
        }
    );

    if let Some(nota) = notas.pop() {
        println!("Último valor = {}", nota);
        println!("{:?}", notas);
    }

    for nota in &notas {
        println!("Nota = {}", nota);
    }

    while let Some(nota) = notas.pop() {
        println!("Valor removido = {}", nota);
    }

    println!("{:?}", notas);
}

/* uso do struct */
#[derive(Debug)]
#[allow(dead_code)]
struct Conta {
    titular: Titular,
    saldo: f64,
}

impl Conta {

    fn sacar(&mut self, valor:f64) {
        self.saldo -= valor;
    }

}

#[derive(Debug)]
#[allow(dead_code)]
struct Titular {
    nome: String,
    sobrenome: String,
}

fn conta_corrente() {
    let mut conta: Conta = Conta {
        titular: Titular {
            nome: String::from("Yan"),
            sobrenome: String::from("Justino"),
        },
        saldo: 100.00,
    };

    println!("Dados da conta = {:?}", conta);

    conta.sacar(50.0);

    println!("Dados da conta = {:?}", conta);
}
