struct Numero {
    valor :u64 ,

}
impl Numero {
    //constuctor
    fn new(valor :u64)-> Self{
        Numero {valor}
    }
    fn espar(&self) -> bool{
        self.valor % 2 == 0
    }
    fn cantdigitos(&self)->u64 {
        let mut n :u64 =self.valor;
        let mut cantidad :u64  = 0;
        while n > 0{
            cantidad +=1;
            n=n/10;
        }
        cantidad
    }

    fn esmayor(&self,x:u64)-> bool {

        self.valor>x
    }
    //metodo que devuelva la suma de todos los numeros naturales del valor,incluido el valor EJ:
    //valor =5: 1+2+3+4+5 =15
    fn sumnat(&self)-> u64 {
        let mut suma :u64= 0;
        for i in 1..=self.valor{
            suma = suma + i ;

        }
        suma
    }
        
        //1.-Metodo que devuelve potencia propia: valor elevado a valor 
        //2.-metodo que devuelve si el valor es multiplo de "n"
        //3.-Metodo que devuelva la cantidad de digitos impares 

      fn potenpropia(&self) -> u64 {
        let mut resultado :u64 = 1;
        for _ in 0..self.valor {
            resultado = resultado * self.valor;

        }
        resultado
          
      }
      
      fn esmultiplo(&self, n: u64)-> bool {
        self.valor % n == 0
          
      }

      fn cantidadimpares(&self) -> u64 {
    let mut n: u64 = self.valor;
    let mut cantidad: u64 = 0;

    while n > 0 {
        let digito = n % 10;
        if digito % 2 != 0 {
            cantidad += 1;
        }
        n = n / 10;
    }

    cantidad
}



}
fn main() {
    //La instacia
    let n =Numero ::new(5) ;
        //llamar a metodos 
    println!("el valor actual de la instacia n es: {}",n.valor);
    println!("el valor par? {}", n.espar());
    println!("La cantidad de digitos del valor de la instacia es: {}",n.cantdigitos());
    println!("El valor , es mayor que el numero x? {}", n.esmayor(230));
    println!("La suma natural del valor es : {}",n.sumnat());
    println!("la potencia propia es : {}", n.potenpropia());
    println!("el multiplo de 4 : {}", n.esmultiplo(4));
    println!(" cantidad de numeros impares es: {}", n.cantidadimpares());
}