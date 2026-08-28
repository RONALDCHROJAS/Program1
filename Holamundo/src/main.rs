fn main() {
    println!("Hola, mundo!");
    let mut x: i16 = 5;
    // i8, i16, i32, i64.. enteros positivos
    // u8, u16, u32, u64, usize.. enteros positivos y negativos.
    // f8. f16, f32, f64.. Números flotantes (o con decimales)
    // booleanos.. true o false
    //char.. "t", (puede permitir emoticones)
    let y = 7;
    println!("El valor de x es: {}", x);
    println!("El valor de y es: {}", y);
    let suma = x + y;
    println!("La suma de x+y es: {}", suma);

    x = 7;
    println!("El nuevo valor de x ahora es: {}", x);
    //Verificar si una persona ya puede votar: Ser mayor de edad.

    let edad = 18;
    if edad > 17 {
        println!("Ya podés votar choquito");
    } else {
        println!("Lo siento, no podes votar, sos muy cachorro..");
    }

    let condicion = false;
    let resultado = if condicion {5} else {7};
    println!("El resultado es: {}", resultado);

    //LOOP
    //Hacer un loop para cuente la cantidad de intentos de ingreso de una contraseña:
    //intentos = 0 - controlar que no se acabaron esos 3 intentos, en cada intento deben mostrar
    //el número de intento que lleva, si se acaban los intentos, debe mostrar mensaje que fue bloqueado.

    let mut intentos = 0;
    loop {
        intentos += 1;
        println!("Este es tu inteno #: {}", intentos);

        if intentos == 3 {
            println!("Cuenta Bloqueada");
            break;
        }
    }

    //WHILE
    //FOR
    //Imprimir la tabla del 5. 5, 10, 15, 20.. etc. usar un "for" para que recorra hasta el 10, y multiplique x 5.
    for i in 1..=10 {
        println!("5 x {} = {}", i, 5*i);
        println!("");
    }

    
}
