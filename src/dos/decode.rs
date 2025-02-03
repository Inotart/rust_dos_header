use super::structs;


/*
解析文件中的 dos数据
输入类型:Vec[u8]*/
pub fn decode(date: Vec<u8>) -> Result<structs::DosHeader, Box<dyn std::error::Error>> {
    Ok(structs::DosHeader {
        e_magic: read_u16_by_vec_u8(&date, 0),
        e_cblp: read_u16_by_vec_u8(&date, 2),
        e_cp: read_u16_by_vec_u8(&date, 4),
        e_crlc: read_u16_by_vec_u8(&date, 6),
        e_cparhdr: read_u16_by_vec_u8(&date, 8),
        e_minalloc: read_u16_by_vec_u8(&date, 10),
        e_maxalloc: read_u16_by_vec_u8(&date, 12),
        e_ss: read_u16_by_vec_u8(&date, 14),
        e_sp: read_u16_by_vec_u8(&date, 16),
        e_csum: read_u16_by_vec_u8(&date, 18),
        e_ip: read_u16_by_vec_u8(&date, 20),
        e_cs: read_u16_by_vec_u8(&date, 22),
        e_lfarlc: read_u16_by_vec_u8(&date, 24),
        e_ovno: read_u16_by_vec_u8(&date, 26),
        e_res: read_move_vec_u18_list_by_vec_u8(&date, 28, 4)
            .try_into()
            .map_err(|_| {
                Box::new(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    "Conversion error",
                ))
            })?,

        e_res2: read_move_vec_u18_list_by_vec_u8(&date, 36, 10)
            .try_into()
            .map_err(|_| {
                Box::new(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    "Conversion error",
                ))
            })?,
        e_oemid: read_u16_by_vec_u8(&date, 56),
        e_oeminfo: read_u16_by_vec_u8(&date, 58),
        e_lfanew: u32::from_le_bytes([date[60], date[61], date[62], date[63]]),
    })
}
fn read_u16_by_vec_u8(raw_date: &Vec<u8>, start: usize) -> u16 {
    return u16::from_le_bytes([raw_date[start], raw_date[start + 1]]);
}
fn read_move_vec_u18_list_by_vec_u8(raw_date: &Vec<u8>, start: usize, len: usize) -> Vec<u16> {
    let mut result = Vec::new();
    for i in 0..len {
        result.push(read_u16_by_vec_u8(raw_date, start + i * 2));
    }
    return result;
}
