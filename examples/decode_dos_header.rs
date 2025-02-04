use std::{fs::File, io::Read};
use rust_dos_header::dos;
// This is a example to load file and read dos header,you need copy a exe file to running path and rename it to A.exe
// 这是一个加载文件并读取 DOS 头的示例，你需要将一个 exe 文件复制到运行路径并重命名为 A.exe
// The DEMO is not good enough, but it can run
// 这个示例还不够好，但它可以运行
// Because the demo load file all date, waste a lot of memory。
// 因为示例加载文件所有数据，浪费大量没有用的内存
fn main(){
    // Open A.exe File
    // 打开 A.exe 文件
    let mut file = File::open("A.exe").unwrap();
    // Read A.exe Data
    // 读取 A.exe 数据
    let mut data = Vec::new();
    file.read_to_end(&mut data).unwrap();
    // Read DOS Header
    // 读取 DOS 头
    let dos_header = rust_dos_header::dos::decode(data);
    // Print DOS Header
    // 打印 DOS 头
    println!("{:?}", dos_header);
    // Print DOS Header use print function
    // 使用 print 函数打印 DOS 头
    println!("{}",dos::print(dos_header.unwrap()).unwrap());
}