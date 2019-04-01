mod reader;
fn main() {
    let text = reader::ReadFile("hello.txt");
    println!("{}", text)
}
