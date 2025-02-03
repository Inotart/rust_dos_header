// Word 类型,这里单纯至少为了阅读理解，尽量保持于原版一致
pub type Word = u16;
// 结构体 DosHeader
#[repr(C)]
#[derive(Debug)]
pub struct DosHeader {
    pub e_magic: Word,      // DOS可执行文件标记
    pub e_cblp: Word,       // 堆栈段大小
    pub e_cp: Word,         // 文件中的重定位项数量
    pub e_crlc: Word,       // 重定位项大小
    pub e_cparhdr: Word,    // 转移地址
    pub e_minalloc: Word,   // 最小分配大小
    pub e_maxalloc: Word,   // 最大分配大小
    pub e_ss: Word,         // 初始堆栈段
    pub e_sp: Word,         // 初始堆栈指针
    pub e_csum: Word,       // 校验和
    pub e_ip: Word,         // 初始指令指针
    pub e_cs: Word,         // 初始代码段
    pub e_lfarlc: Word,     // 重定位表文件地址
    pub e_ovno: Word,       // 覆盖号
    pub e_res: [Word; 4],   // 保留字
    pub e_oemid: Word,      // OEM标识符
    pub e_oeminfo: Word,    // OEM信息
    pub e_res2: [Word; 10], // 保留字
    pub e_lfanew: u32,     // PE头文件地址
}

pub fn word_to_string(word: Word) -> Result<String, Box<dyn std::error::Error>> {
    let vec_u8: Vec<u8> = word.to_le_bytes().to_vec();
    Ok(String::from_utf8(vec_u8)?)
}
