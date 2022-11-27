fn main() {
    let width = 4;
    let height = 7;
    let depth = 10;

    let area = area_of(width, height);

    println!("Area is {}", area);

    println!("Volume is {}", volume_of(width, height, depth));

}

fn area_of(x: i32, y: i32) -> i32 {
    return x * y; // run 'cargo clippy' linter command to improving the code
}

fn volume_of(a: i32, b: i32, c: i32) -> i32  {
    a * b * c// this is a tail expression !!!, note the absence of 'return' and of ';'
} 

