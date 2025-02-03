
pub mod dos;

fn main() {
    println!("Rust Dos Header Decode Tool");
    println!("Rust Dos头解析工具");
    println!("作者:inotart");
    println!("版本:{:?}", env!("CARGO_PKG_VERSION"));
    println!("Github:https://github.com/inotart/rust_dos_header");
    println!("本程序使用 MIT 许可证发布");
    println!("本程序不包含任何恶意代码，请放心使用");
    println!("注:请不要直接运行本程序,这只是一个rust库");
    test()
}
fn test(){
    // 文件路径:rust_decode_exe.exe
    let file_path = "rust_decode_exe.exe";
    // 读取文件内容
    let file_content = std::fs::read(file_path).expect("无法读取文件");

    let dos_header = dos::decode(file_content);
    if !dos_header.is_ok() {
        println!("文件格式不正确");
        return;
    }
    let dos_header = dos_header.unwrap();
    println!("{:?}", dos_header);
    let e_lfanew = dos_header.e_lfanew ;
    println!("{}", dos::print(dos_header).unwrap());
    let file_content2 = std::fs::read(file_path).expect("无法读取文件");
    let text = String::new();
    // 读取 dos_header.e_lfanew 大小的文件内容（dos_header.e_lfanew是u32类型）
    let v = e_lfanew as usize;
    let file_content_slice = &file_content2[v..v+2];
    println!("{:?}", String::from_utf8(file_content_slice.to_vec()).unwrap());
    

}
