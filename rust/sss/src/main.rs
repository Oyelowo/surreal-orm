
use hello_macro::HelloMacro;



#[derive(HelloMacro)]
struct Pancakes;


fn main() {
    Pancakes::hello_macro();
     println!("Hello, world!");
}