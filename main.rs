use std::io::{self, Write};

const N: usize = 100;

struct Cadena {
    longitud: usize,
    caracteres: [char; N],
}

impl Cadena {
    // Constructor
    fn new() -> Self {
        Cadena {
            longitud: 0,
            caracteres: ['\0'; N],
        }
    }

    // Metodo para obtener la longitud
    fn obtener_longitud(&self) -> usize {
        self.longitud
    }

    // Metodo para adicionar caracteres
    fn add_char(&mut self, c: char) {
        if self.longitud < N {
            self.caracteres[self.longitud] = c;
            self.longitud += 1;
        }
    }

    // Metodo para devolver un caracter dada la posicion
    fn obtener_char(&self, pos: usize) -> char {
        if pos > 0 && pos <= self.longitud {
            self.caracteres[pos - 1]
        } else {
            '\0'
        }
    }

    // Metodo para contar la cantidad de apariciones de un caracter
    fn contar_apariciones(&self, c: char) -> usize {
        let mut contador: usize = 0;

        for i in 0..self.longitud {
            if self.caracteres[i] == c {
                contador += 1;
            }
        }

        contador
    }

    // Metodo que devuelve el caracter mas repetido
    fn char_mas_repetido(&self) -> char {
        if self.longitud == 0 {
            return '\0';
        }

        let mut max_char = self.caracteres[0];
        let mut max_cont = 0;

        for i in 0..self.longitud {
            let car = self.caracteres[i];
            let mut cont = 0;

            for j in 0..self.longitud {
                if self.caracteres[j] == car {
                    cont += 1;
                }
            }

            if cont > max_cont {
                max_cont = cont;
                max_char = car;
            }
        }

        max_char
    }

    // Metodo para invertir la cadena
    fn invertir(&mut self) {
        if self.longitud <= 1 {
            return;
        }

        let mut izq = 0;
        let mut der = self.longitud - 1;

        while izq < der {
            let temp = self.caracteres[izq];
            self.caracteres[izq] = self.caracteres[der];
            self.caracteres[der] = temp;

            izq += 1;
            der -= 1;
        }
    }

    // Metodo para contar vocales y consonantes
    fn contar_vocales_consonantes(&self) -> (usize, usize) {
        let mut vocal: usize = 0;
        let mut consonante: usize = 0;

        for i in 0..self.longitud {
            let car = self.caracteres[i];

            let letra = (car >= 'a' && car <= 'z') || (car >= 'A' && car <= 'Z');

            if letra {
                let esvocal = car == 'a'
                    || car == 'e'
                    || car == 'i'
                    || car == 'o'
                    || car == 'u'
                    || car == 'A'
                    || car == 'E'
                    || car == 'I'
                    || car == 'O'
                    || car == 'U';

                if esvocal {
                    vocal += 1;
                } else {
                    consonante += 1;
                }
            }
        }

        (vocal, consonante)
    }

    // Metodo para eliminar caracteres duplicados consecutivos
    // Ej: aaabbbccdfd = abcdfd
    fn eliminar_repetidos_consecutivos(&self) -> Cadena {
        let mut cad = Cadena::new();

        for i in 0..self.longitud {
            if i == 0 || self.caracteres[i] != self.caracteres[i - 1] {
                cad.add_char(self.caracteres[i]);
            }
        }

        cad
    }

    // Metodo para eliminar un caracter de una posicion dada
    fn eliminar_car(&mut self, p: usize) {
        if p > 0 && p <= self.longitud {
            let pos = p - 1;

            for i in pos..self.longitud - 1 {
                self.caracteres[i] = self.caracteres[i + 1];
            }

            self.longitud -= 1;
        }
    }

    // Metodo para obtener una subcadena
    // Ej: Hola como va -> inicio = 3 fin = 9 => la como
    fn subcadena(&self, inicio: usize, fin: usize) -> Cadena {
        let mut subcad = Cadena::new();

        if inicio < 1 || fin > self.longitud || inicio > fin {
            return subcad;
        }

        for i in (inicio - 1)..fin {
            subcad.add_char(self.caracteres[i]);
        }

        subcad
    }

    // =========================================================
    // OPCION 10
    // Eliminar palabras que contengan una vocal repetida
    //
    // Ej:
    // ESTA ES UNA PEQUEÑA PRUEBA
    // Resultado:
    // ESTA ES UNA PRUEBA
    // =========================================================
    fn eliminar_palabras_vocal_repetida(&self) -> Cadena {
        let mut resultado = Cadena::new();
        let mut i = 0;

        while i < self.longitud {
            // Saltar espacios
            while i < self.longitud && self.caracteres[i] == ' ' {
                i += 1;
            }

            if i >= self.longitud {
                break;
            }

            // Guardamos donde comienza la palabra
            let inicio = i;

            // Avanzamos hasta encontrar un espacio
            while i < self.longitud && self.caracteres[i] != ' ' {
                i += 1;
            }

            // Aqui termina la palabra
            let fin = i;

            // Verificar si alguna vocal esta repetida
            let mut tiene_repetida = false;
            let mut j = inicio;

            while j < fin {
                let c = self.caracteres[j];

                // Verificamos si el caracter es una vocal
                let es_vocal = c == 'a'
                    || c == 'e'
                    || c == 'i'
                    || c == 'o'
                    || c == 'u'
                    || c == 'A'
                    || c == 'E'
                    || c == 'I'
                    || c == 'O'
                    || c == 'U';

                if es_vocal {
                    let mut contador = 0;
                    let mut k = inicio;

                    // Contamos cuantas veces aparece esa vocal
                    while k < fin {
                        if self.caracteres[k] == c {
                            contador += 1;
                        }

                        k += 1;
                    }

                    // Si aparece 2 o mas veces,
                    // eliminamos toda la palabra
                    if contador >= 2 {
                        tiene_repetida = true;
                        break;
                    }
                }

                j += 1;
            }

            // Si la palabra NO tiene vocal repetida,
            // la copiamos al resultado
            if !tiene_repetida {
                // Agregamos espacio antes de la palabra
                // excepto cuando es la primera
                if resultado.longitud > 0 {
                    resultado.add_char(' ');
                }

                let mut k = inicio;

                while k < fin {
                    resultado.add_char(self.caracteres[k]);
                    k += 1;
                }
            }
        }

        resultado
    }

    // =========================================================
    // OPCION 11
    // Eliminar todos los espacios
    //
    // Ej:
    // Hola como va
    // Resultado:
    // Holacomova
    // =========================================================
    fn eliminar_espacios(&self) -> Cadena {
        let mut resultado = Cadena::new();

        for i in 0..self.longitud {
            if self.caracteres[i] != ' ' {
                resultado.add_char(self.caracteres[i]);
            }
        }

        resultado
    }

    // Metodo para limpiar la cadena
    fn limpiar(&mut self) {
        self.longitud = 0;
        self.caracteres = ['\0'; N];
    }

    // Metodo para mostrar
    fn mostrar(&self) {
        for i in 0..self.longitud {
            print!("{}", self.caracteres[i]);
        }

        println!();
    }
}

// =========================================================
// Entrada de datos
// =========================================================

fn leer_linea() -> String {
    let mut entrada = String::new();

    io::stdin().read_line(&mut entrada).expect("Error al leer");

    entrada.trim().to_string()
}

fn leer_numero() -> Option<usize> {
    leer_linea().parse::<usize>().ok()
}

// =========================================================
// MENU
// =========================================================

fn mostrar_menu(c: &Cadena) {
    let mut preview = String::new();

    for i in 0..c.longitud {
        preview.push(c.caracteres[i]);
    }

    if preview.is_empty() {
        preview = String::from("(vacia)");
    }

    println!();
    println!("╔══════════════════════════════════════════╗");
    println!("║              CADENAS - POO              ║");
    println!("║ Cadena: {:<32}║", preview);
    println!("╠══════════════════════════════════════════╣");
    println!("║ 1. Ingresar nueva cadena                ║");
    println!("║ 2. Mostrar cadena                       ║");
    println!("║ 3. Longitud                             ║");
    println!("║ 4. Obtener caracter (posicion)          ║");
    println!("║ 5. Cantidad repeticiones (char)         ║");
    println!("║ 6. Invertir cadena                      ║");
    println!("║ 7. Vocales y consonantes                ║");
    println!("║ 8. Eliminar caracter (pos)              ║");
    println!("║ 9. Subcadena                            ║");
    println!("║ 10. Eliminar palabras con vocal repetida║");
    println!("║ 11. Eliminar espacios                   ║");
    println!("║ 12. Eliminar repetidos consecutivos    ║");
    println!("╠══════════════════════════════════════════╣");
    println!("║ Q. Salir                                ║");
    println!("╚══════════════════════════════════════════╝");

    print!("Opcion: ");
    io::stdout().flush().expect("Error al mostrar menu");
}

// =========================================================
// MAIN
// =========================================================

fn main() {
    println!("====================================");
    println!("      CADENAS - POO                 ");
    println!("      Programacion I                ");
    println!("====================================");

    let mut c = Cadena::new();

    loop {
        mostrar_menu(&c);

        let opcion = leer_linea();

        match opcion.as_str() {
            // =================================================
            // 1. INGRESAR CADENA
            // =================================================
            "1" => {
                println!("Ingresa la cadena:");

                let entrada = leer_linea();

                c.limpiar();

                for ch in entrada.chars() {
                    c.add_char(ch);
                }

                println!("Cadena cargada ({} caracteres)", c.obtener_longitud());
            }

            // =================================================
            // 2. MOSTRAR
            // =================================================
            "2" => {
                print!("Cadena: ");
                c.mostrar();
            }

            // =================================================
            // 3. LONGITUD
            // =================================================
            "3" => {
                println!("Longitud: {}", c.obtener_longitud());
            }

            // =================================================
            // 4. OBTENER CARACTER
            // =================================================
            "4" => {
                println!("Ingresa la posicion (1 = izquierda):");

                match leer_numero() {
                    Some(pos) if pos >= 1 && pos <= c.obtener_longitud() => {
                        println!("Caracter en posicion {}: '{}'", pos, c.obtener_char(pos));
                    }

                    Some(_) => {
                        println!("Posicion fuera de rango.");
                    }

                    None => {
                        println!("Posicion invalida.");
                    }
                }
            }

            // =================================================
            // 5. CONTAR APARICIONES
            // =================================================
            "5" => {
                println!("Ingresa el caracter:");

                let entrada = leer_linea();

                match entrada.chars().next() {
                    Some(car) => {
                        let cantidad = c.contar_apariciones(car);

                        println!("El caracter '{}' aparece {} vez/veces", car, cantidad);
                    }

                    None => {
                        println!("No ingresaste ningun caracter.");
                    }
                }
            }

            // =================================================
            // 6. INVERTIR
            // =================================================
            "6" => {
                if c.obtener_longitud() == 0 {
                    println!("La cadena esta vacia");
                } else {
                    c.invertir();

                    println!("La cadena invertida es:");

                    c.mostrar();
                }
            }

            // =================================================
            // 7. VOCALES Y CONSONANTES
            // =================================================
            "7" => {
                if c.obtener_longitud() == 0 {
                    println!("La cadena esta vacia");
                } else {
                    let (vocal, consonante) = c.contar_vocales_consonantes();

                    println!("Numero de vocales: {}", vocal);

                    println!("Numero de consonantes: {}", consonante);
                }
            }

            // =================================================
            // 8. ELIMINAR CARACTER POR POSICION
            // =================================================
            "8" => {
                if c.obtener_longitud() == 0 {
                    println!("La cadena esta vacia");
                } else {
                    println!("Ingresa la posicion a eliminar:");

                    match leer_numero() {
                        Some(pos) if pos > 0 && pos <= c.obtener_longitud() => {
                            c.eliminar_car(pos);

                            println!("Resultado:");

                            c.mostrar();
                        }

                        Some(_) => {
                            println!("Posicion fuera de rango.");
                        }

                        None => {
                            println!("Invalido.");
                        }
                    }
                }
            }

            // =================================================
            // 9. SUBCADENA
            // =================================================
            "9" => {
                if c.obtener_longitud() == 0 {
                    println!("La cadena esta vacia");
                } else {
                    println!("Ingresa la posicion de inicio:");

                    match leer_numero() {
                        Some(inicio) if inicio > 0 && inicio <= c.obtener_longitud() => {
                            println!("Ingresa la posicion del fin:");

                            match leer_numero() {
                                Some(fin) if fin >= inicio && fin <= c.obtener_longitud() => {
                                    let subca = c.subcadena(inicio, fin);

                                    println!("La subcadena [{} - {}] es:", inicio, fin);

                                    subca.mostrar();

                                    println!("Longitud: {}", subca.longitud);
                                }

                                Some(_) => {
                                    println!("Fin invalido.");
                                }

                                None => {
                                    println!("Posicion invalida.");
                                }
                            }
                        }

                        Some(_) => {
                            println!("Inicio fuera de rango.");
                        }

                        None => {
                            println!("Posicion invalida.");
                        }
                    }
                }
            }

            // =================================================
            // 10. ELIMINAR PALABRAS CON VOCAL REPETIDA
            // =================================================
            "10" => {
                if c.obtener_longitud() == 0 {
                    println!("La cadena esta vacia.");
                } else {
                    let nueva = c.eliminar_palabras_vocal_repetida();

                    println!("Resultado:");

                    nueva.mostrar();
                }
            }

            // =================================================
            // 11. ELIMINAR ESPACIOS
            // =================================================
            "11" => {
                if c.obtener_longitud() == 0 {
                    println!("La cadena esta vacia.");
                } else {
                    let nueva = c.eliminar_espacios();

                    println!("Resultado:");

                    nueva.mostrar();
                }
            }

            // =================================================
            // 12. ELIMINAR REPETIDOS CONSECUTIVOS
            // =================================================
            "12" => {
                if c.obtener_longitud() == 0 {
                    println!("La cadena esta vacia.");
                } else {
                    let nueva = c.eliminar_repetidos_consecutivos();

                    println!("Resultado:");

                    nueva.mostrar();
                }
            }

            // =================================================
            // SALIR
            // =================================================
            "q" | "Q" => {
                println!("Hasta luego.");

                break;
            }

            _ => {
                println!("Opcion no valida.");
            }
        }
    }
}
