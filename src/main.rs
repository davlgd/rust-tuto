macro_rules! customPrint {
    () => {
        println!("Hello, World!");
    };
    ($dest:expr) => {
        println!("Hello, {}!", $dest);
    };
    ($act:expr => $dest:expr) => {
        println!("{} for {}!", $act, $dest);
    };
}

fn main() {
    println!("{}, {crowd} {1}!", "Hello", 2*21, crowd = "World");
}