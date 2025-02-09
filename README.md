# rust_dos_header
基于 rust 实现的 dos 头解析器依赖库
## 介绍
基于 rust 实现的 dos 头解析器依赖库，可以解析 dos 头信息，包括 e_magic、e_cblp、e_cp、e_crlc、e_cparhdr、e_minALLOC、e_maxALLOC、e_ss、e_sp、e_csum、e_ip、e_cs、e_lfarlc、e_ovno、e_res、e_oemID、e_oeminfo、e_lfanew 等字段。

## 安装
```shell
cargo add rust_dos_header
```

## 使用
```rust
use rust_dos_header::dos::DosHeader;
fn main() {
    let dos_header = DosHeader::new();
    let file_path = "rust_decode_exe.exe";
    // 读取文件内容
    let file_content = std::fs::read(file_path).expect("无法读取文件");

    let dos_header = dos::decode(file_content);
    if !dos_header.is_ok() {
        println!("文件格式不正确");
        return;
    }
    let dos_header = dos_header.unwrap();
    println!("{}", dos::print(dos_header).unwrap());
}
```
