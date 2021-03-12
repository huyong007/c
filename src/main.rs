fn main() {
    println!("Hello, world!");
    tellme('h');
}

fn tellme(name: char) {
    println!("{},你好", name);
    let mut x = 8;
    x = 9;
    println!("{}", x)
}
