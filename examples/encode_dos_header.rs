// This is encode dos header DEMO,The demo is encode dos header to bytes
// 这是编码DOS头部的DEMO，该DEMO将DOS头部编码为字节
use rust_dos_header::dos;
fn main() {
    let dos_header = dos::structs::DosHeader { e_magic: 23117, e_cblp: 144, e_cp: 3, e_crlc: 0, e_cparhdr: 4, e_minalloc: 0, e_maxalloc: 65535, e_ss: 0, e_sp: 184, e_csum: 0, e_ip: 0, e_cs: 0, e_lfarlc: 64, e_ovno: 0, e_res: [0, 0, 0, 0], e_oemid: 0, e_oeminfo: 0, e_res2: [0, 0, 0, 0, 0, 0, 0, 0, 0, 0], e_lfanew: 248 };
    let bytes = &dos::encode(&dos_header); // encode dos header to bytes
    println!("{:?}", bytes);
    // Tip: You can use this bytes to write to a file
    // 提示：你可以使用这些字节写入文件
    // Tip: The program does not include the assembly code of Dos, you need to write it yourself, pay attention to the position of the PE header
    // 提示: 该程序不会包括 Dos 的汇编代码部分,需自行写入,注意PE头位置
}