mod reader;
fn main() {
    let text = reader::read_file("hello.txt");
    println!("{}", text)
}
