fn main() {

    let msg = "Hello World"; 

    let n: i32 = 123;

    println!("{} ! {}", msg, n);

    let msg = "Salve Modo"; 

    {
        let msg = "Salve"; 
        let n: i32 = 456;
        println!("{} ! {}", msg, n);
    }

    let enigma: i32;

    if true {
        enigma = 1;
    } else {
        enigma = 0;
    }

    println!("{} ! {}", msg, enigma);
}

