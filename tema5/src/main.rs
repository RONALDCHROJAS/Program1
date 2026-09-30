struct Numero {
    valor: u64,
}

impl Numero {
    // Constructor
    fn new(valor: u64) -> Self {
        Numero { valor }
    }

    fn es_par(&self) -> bool {
        self.valor % 2 == 0
    }

    fn cantidaddigitos(&self) -> u64 {
        if self.valor == 0 {
            return 1;
        }

        let mut num = self.valor;
        let mut cantidad = 0;

        while num > 0 {
            num /= 10;
            cantidad += 1;
        }

        cantidad
    }

    // Devuelve la posición del dígito, contando desde la izquierda
    fn dev_pos_dig(&self, digito: u64) -> u64 {
        if digito > 9 {
            return 0;
        }

        if self.valor == 0 {
            if digito == 0 {
                return 1;
            } else {
                return 0;
            }
        }

        let mut num = self.valor;
        let mut posicion = self.cantidaddigitos();

        while num > 0 {
            let digito_actual = num % 10;

            if digito_actual == digito {
                return posicion;
            }

            num /= 10;
            posicion -= 1;
        }

        0
    }

    // Elimina una aparición del dígito indicado
    fn eliminar_dig(&mut self, digito: u64) {
        let posicion = self.dev_pos_dig(digito);

        if posicion == 0 {
            println!("No se encontró el dígito.");
            return;
        }

        self.eliminar(posicion);
    }

    // Elimina el dígito ubicado en una posición
    fn eliminar(&mut self, posicion: u64) {
        let total = self.cantidaddigitos();

        if posicion == 0 || posicion > total {
            println!("Posición inválida.");
            return;
        }

        if total == 1 {
            self.valor = 0;
            return;
        }

        let mut peso = 1;

        for _ in 0..(total - posicion) {
            peso *= 10;
        }

        let parte_izquierda = self.valor / (peso * 10);
        let parte_derecha = self.valor % peso;

        self.valor = parte_izquierda * peso + parte_derecha;
    }

    fn binario(&self) -> u64 {
        if self.valor == 0 {
            return 0;
        }

        let mut num = self.valor;
        let mut binario = 0;
        let mut multiplicador = 1;

        while num > 0 {
            let residuo = num % 2;

            binario += residuo * multiplicador;
            multiplicador *= 10;
            num /= 2;
        }

        binario
    }

    fn hexadecimal(&self) -> String {
        if self.valor == 0 {
            return String::from("0");
        }

        let mut num = self.valor;
        let mut resultado = String::new();

        let simbolos = b"0123456789ABCDEF";

        while num > 0 {
            let residuo = (num % 16) as usize;
            resultado.insert(0, simbolos[residuo] as char);
            num /= 16;
        }

        resultado
    }
}

fn main() {
    let mut n = Numero::new(57104);

    println!("Valor original: {}", n.valor);
    println!("¿Es par?: {}", n.es_par());
    println!("Cantidad de dígitos: {}", n.cantidaddigitos());
    println!("Binario: {}", n.binario());
    println!("Hexadecimal: {}", n.hexadecimal());

    println!("Posición del dígito 1: {}", n.dev_pos_dig(1));

    n.eliminar_dig(1);

    println!("Después de eliminar el dígito 1: {}", n.valor);
}
