/* uso de constantes */
const PI: f32 = 3.14;
static mut GLOBAL: u8 = 1;

fn main() {
    variaveis();
    escopo();
    funcoes();
    condicional();
    repeticoes();
    match_statement();
    ownership();
    pattern_matching();
    errors();
}

/* manipulação de erros */
fn errors(){
    match erros_resultado(0) {
        Ok(s) => println!("String de sucesso: {}", s),
        Err(numero) => println!("Codigo de erro {}", numero),
    }
}

/* uso de Result para tratar erros
Para disparar erros de irreparáveis utilize panic!(string)
*/
fn erros_resultado(value:i8) -> Result<String, u8>{
    if value % 2 == 0 {
        Ok(String::from("deu certo!"))
    } else {
        Err(42)
    }
}

/* Uso de pattern matching */
fn pattern_matching(){
    for i in 1..21{
        println!("{} : {}", i, match i {
            1 => "Pouco",
            2 | 3 => "Um pouquinho",
            4..=10 => "Um bocado",
            _ if i % 2 == 0 => "Uma boa quantidade",
            _ => "Muito"
        });
    }
}

/* conceito de ownership (referência e borrowing)
uma variável só pode ter um dono
 */
fn ownership(){
    //Imutavel
    let uma_string = String::from("Yan Imutável");
    rouba_imutavel(&uma_string);
    println!("{}", uma_string);

    //Mutável
    let mut outra_string = String::from("Yan mutável");
    rouba_mutavel(&mut outra_string);
    println!("{}", outra_string);
}

/* recebendo a referência do valor na heap.
o parâmetro da função é imutável
*/
fn rouba_imutavel(string:&String){
    println!("{}", string);
}

/* recebendo a referência do valor na heap.
o parâmetro da função é mutável
*/
fn rouba_mutavel(string:&mut String){
    string.push_str(" Justino");
    println!("{}", string);
}

/* Uso de match statement */
fn match_statement(){
    let linguagem = "PHP";

    let proposito = match linguagem {
        "PHP" => "Web",
        "Kotlin" => "Android",
        "Python" => "Data sience",
        _ => "Desconhecido"
    };

    println!("O proposito de {} é {}", linguagem, proposito);
}

/* uso de loops */
fn repeticoes() {
    let multiplicador:u8 = 5;
    let mut contador:u8 = 0;

    while contador < 10 {
        contador += 1;
        println!("{} x {} = {}", multiplicador, contador, multiplicador * contador);
    }

    contador = 0;
    // 'loop' é um iterador infinito
    loop{
        contador += 1;
        println!("{} x {} = {}", multiplicador, contador, multiplicador * contador);

        // uso de break. Existe também o continue
        if contador == 10 { break };
    }

    // for em um range! {inclusive}..{exclusive}
    for i in 1..11 {
        println!("{} x {} = {}", multiplicador, i, multiplicador * i);
    }
}

/* uso de condicional 'if' */
fn condicional() {
    let idade: u8 = 14;
    let responsavel_autorizou = false;
    let maior_de_idade = idade >= 18;

    let autorizacao = if idade > 18 {
        "AUTORIZADO"
    } else if !maior_de_idade && responsavel_autorizou {
        "AUTORIZADO PELO RESONPONSÁVEL"
    } else {
        "NÃO AUTORIZADO"
    };

    let condicao = if maior_de_idade { "maior" } else { "menor" };

    println!("É {} de idade: {}", condicao, autorizacao);
}

/* chamando funções */
fn funcoes() {
    println!("soma = {}", soma(1, 1));
}

/* uso de funções */
fn soma(a: i32, b: i32) -> i32 {
    println!("{} + {} = {}", a, b, a + b);
    // em rust o ';' é suprime a propagação do valor de uma expressão
    // Quando remove o ';' vc utiliza a expressão retornando um valor
    a + b
}

/* uso de escopo */
fn escopo() {
    let a = 123;

    {
        let b = 456;
        println!("dentro, b = {}", b);

        let a = 777;
        println!("dentro, a = {}", a);
    }

    println!("fora, a = {}", a);
}

/* declaração de variáveis e tipos */
fn variaveis() {
    println!("PI = {}", PI);

    unsafe {
        println!("variaval GLOBAL = {}", GLOBAL);
    }

    let variavel: i32 = 300;
    println!(
        "variavel = {}, tamanho = {} bytes",
        variavel,
        std::mem::size_of_val(&variavel)
    );

    let decimal: f32 = 2.5;
    println!("decimal = {}", decimal);

    let booleana = false;
    println!(
        "booleana = {}, Tamanho booleana = {}",
        booleana,
        std::mem::size_of_val(&booleana)
    );

    let letra = 'C';
    println!(
        "letra = {}, Tamanho letra = {}",
        letra,
        std::mem::size_of_val(&letra)
    );
}
