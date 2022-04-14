fn main() {
    println!("Hello, world!");

    /* variabile immutabile */
    let v: i32 = 123;
    //v = -5;
    /* variabile mutabile */
    let mut w = v;
    w = -5;

    /* altro modo per specificare il tipo */
    let x = 1.3278;         //floating point a 64bit
    let y = 1.3278f32;      //floating point a 32bit
    let z: f32 = 1.3278;    //floating point a 32bit

    /* tupla */
    let t: (i32, bool) = (123, false);
    /* tupla mut */
    let mut u = (2.14, 2.71); //sono f64
    u.0 = u.0 + 1.0;
    println!("pigreco = {}", u.0);
    println!("{:?}", t);

    /* riferimenti */
    let r1 = &v;
    //*r1 = 1; v è immutabile     
    println!("v = {}", v); //qua è prestato però credo che me lo faccia leggere perchè tanto è immutabile
    

    println!("w = {}", w);
    {
        let r2 = &mut w;
        //println!("w = {}", w); prestato, non si può usare manco per leggere perchè è mut
        *r2 = 1;    
        println!("r2 = {}", r2);
        println!("w = {}", w);
    }
    //println!("r2 = {}", r2); morto
    println!("w = {}", w);

    /* Box<T> */
    let mut b = Box::new( (5, 2) ); //quel (5, 2) è una tupla
    //memoria dinamica
    //5 e 2 sono interi, nello HEAP
    //b è un riferimento, che è nello STACK, alle variabili "boxate"
    (*b).1 = 7;

    println!("b = {:?}", *b); 
    println!("b = {:?}", b); 

    /* Array */
    //NON TUPLE, quindi contenuto OMOgeneo
    let a: [i32; 5] = [1, 5, 78, 99, 2];
    let b = [0; 5];     //inizializzato a [0, 0, 0, 0, 0]

    let l = b.len();
    let e = a[3];

    /* Slice */
    let s2 = &a[0..2];

    /* STRINGHE */
    let hello: &str = "str"; //TIPO STR

    //let mut s: String = "ciaone"; //non così... "ciaone" è di tipo &str
    let mut s1 = String::new();
    let mut s2 = String::from("hello");
    s2.push_str(" string"); //ci sono centinaia di metodi
    println!("{}", s2);

    /* FUNZIONI */
    fn print_number(lol: i32) {
        println!("lol is: {}", lol)
    }
    print_number(5);

    fn add_numbers(x: i32, y: i32) -> i32 {
        x+y
    }
    println!("{}", add_numbers(2, 1));

}
