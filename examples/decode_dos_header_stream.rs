use std::{fs::File, io::{Read, Seek}};
use rust_dos_header::dos;
// This is a example to load file and read dos header,you need copy a exe file to running path and rename it to A.exe
// 这是一个加载文件并读取 DOS 头的示例，你需要将一个 exe 文件复制到运行路径并重命名为 A.exe

fn main(){
    // Open A.exe File
    // 打开 A.exe 文件
    let mut file = File::open("A.exe").unwrap();
    // Read DOS Header
    // 读取 DOS 头
    let dos_header = rust_dos_header::dos::decode_file(&file);
    // Print DOS Header
    // 打印 DOS 头
    println!("{:?}", dos_header);
    // Print DOS Header use print function
    // 使用 print 函数打印 DOS 头
    println!("{}",dos::print(dos_header.unwrap()).unwrap());
    // Seek to initial position
    // Seek 回到初始位置
    file.seek(std::io::SeekFrom::Start(0)).unwrap();
}