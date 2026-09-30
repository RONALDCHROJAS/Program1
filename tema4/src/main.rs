struct Numero {
    valor: u64,
}

impl Numero {

    // Cuenta la cantidad de dígitos
    fn cantidad_digitos(&self) -> u64 {
        let mut n = self.valor;
        let mut cantidad = 0;

        // Caso especial: 0 tiene un dígito
        if n == 0 {
            return 1;
        }

        // Vamos eliminando un dígito
        while n > 0 {
            cantidad += 1;
            n /= 10;
        }

        cantidad
    }


    // Rota los dígitos hacia la izquierda N veces
    fn rotar_izquierda(&mut self, veces: u64) {

        let cantidad = self.cantidad_digitos();

        // Calculamos la potencia de 10
        // Ejemplo: 12345 tiene 5 dígitos
        // 10^(5-1) = 10000

        let mut potencia = 1;
        let mut i = 1;

        while i < cantidad {
            potencia *= 10;
            i += 1;
        }


        // Repetimos la rotación N veces
        let mut vuelta = 0;

        while vuelta < veces {

            // Sacamos el primer dígito
            let primero = self.valor / potencia;

            // Quitamos el primer dígito
            let resto = self.valor % potencia;

            // Ponemos el primer dígito al final
            self.valor = resto * 10 + primero;

            vuelta += 1;
        }
    }
}


fn main() {

    // Creamos el número
    let mut n = Numero {
        valor: 12345,
    };

    // Rotar 3 veces hacia la izquierda
    n.rotar_izquierda(2);

    // Mostrar resultado final
    println!("Resultado: {}", n.valor);
}