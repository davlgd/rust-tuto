mod cpu;

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

    customPrint!();
    customPrint!("You");
    customPrint!("Hurrah" => "Next INpact");

    let (cpu_log, cpu_phy) = cpu::get_cpu_count();
    println!("CPU Cores : {}, CPU Threads : {}", cpu_phy, cpu_log);
}