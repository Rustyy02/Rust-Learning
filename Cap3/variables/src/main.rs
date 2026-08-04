// variables y mutabilidad

fn main() {
    let x = 5;

    let x = x + 1; // x hasta aca es 6

    {
        let x = x * 2; // aquí toma el valor de x anterior, o sea 6, por lo tanto el resultado es 12
        println!("El valor de x en este segmento es {x}");
    }

    println!("El valor de x en este segmento es {x}");

}