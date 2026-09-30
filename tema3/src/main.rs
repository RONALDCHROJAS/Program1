struct Numero {
    valor: u64,
}

impl Numero {
    fn new(valor: u64) -> Self {
        Numero { valor }
    }

    fn cantdig(&self) -> u64 {
        let mut cantidad = 0;
        let mut numero = self.valor;

        while numero > 0 {
            numero /= 10;
            cantidad += 1;
        }

        cantidad
    }

    fn contardig(&self, buscar: u64) -> u64 {
        let mut numero = self.valor;
        let mut contador = 0;

        while numero > 0 {
            let digito = numero % 10;

            if digito == buscar {
                contador += 1;
            }

            numero /= 10;
        }

        contador
    }

    fn impares(&self) -> u64 {
        let mut resultado = 0;
        let mut digito = 1;

        while digito <= 9 {
            let cantidad = self.contardig(digito);

            let mut i = 0;

            while i < cantidad {
                resultado = resultado * 10 + digito;
                i += 1;
            }

            digito += 2;
        }

        resultado
    }

    fn pares(&self) -> u64 {
        let mut resultado = 0;
        let mut digito = 2;

        while digito <= 8 {
            let cantidad = self.contardig(digito);

            let mut i = 0;

            while i < cantidad {
                resultado = resultado * 10 + digito;
                i += 1;
            }

            digito += 2;
        }

        resultado
    }
}

fn main() {
    println!("=============================");
    println!("Struct Numero");
    println!("=============================");

    let n = Numero::new(12345);

    println!("La instancia es {}", n.valor);
    println!("La cantidad de numeros es {}", n.cantdig());

    let impares = n.impares();
    let pares = n.pares();

    println!("{}{}",impares,pares);
}