
pub fn greet() {

    let info = (1, 2.0, 3, 4.0); // tuple is limited to 12 items !!! 

    let table: [i32; 4] = [1, 2, 3, 4]; // array is limited to 32 items, prefer Vec<> !!!

    println!("greet !!! from helper module with info tuple {:?}", info); // tuple can contain items of different type

    println!("first item of the tuble is {}", info.0);

    for item in table.iter().enumerate() {
        println!("table [{}] = {}", item.0, item.1);
    }

    for item in table.iter().enumerate() {
        let (ndx, val) = item;
        println!("table [{}] = {}", ndx, val);
    }

    for (ndx, val) in table.iter().enumerate() {
        println!("table [{}] = {}", ndx, val);
    }

}

