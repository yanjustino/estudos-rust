/* uso de constantes */
const PI:f32 = 3.14;
static mut GLOBAL:u8 = 1;

fn main(){
    escopo();
    sombra();
} 

/* uso de escopo */
fn sombra(){
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
fn escopo(){
    println!("PI = {}", PI);

    unsafe{
        println!("variaval GLOBAL = {}", GLOBAL);
    }

    let variavel:i32 = 300;
    println!("variavel = {}, tamanho = {} bytes", variavel, std::mem::size_of_val(&variavel));

    let decimal:f32 = 2.5;
    println!("decimal = {}", decimal);

    let mut booleana = false;
    println!("booleana = {}, Tamanho booleana = {}", booleana, std::mem::size_of_val(&booleana));

    let letra = 'C';
    println!("letra = {}, Tamanho letra = {}", letra, std::mem::size_of_val(&letra));    
}