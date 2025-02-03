use super::structs;
pub fn encode(data: &structs::DosHeader) -> Vec<u8> {
    let mut bytes = Vec::new();
    bytes.extend_from_slice(&data.e_magic.to_le_bytes());
    bytes.extend_from_slice(&data.e_cblp.to_le_bytes());
    bytes.extend_from_slice(&data.e_cp.to_le_bytes());

    bytes.extend_from_slice(&data.e_crlc.to_le_bytes());
    bytes.extend_from_slice(&data.e_cparhdr.to_le_bytes());
    bytes.extend_from_slice(&data.e_minalloc.to_le_bytes());
    bytes.extend_from_slice(&data.e_maxalloc.to_le_bytes());
    bytes.extend_from_slice(&data.e_ss.to_le_bytes());
    bytes.extend_from_slice(&data.e_sp.to_le_bytes());
    bytes.extend_from_slice(&data.e_csum.to_le_bytes());
    bytes.extend_from_slice(&data.e_ip.to_le_bytes());
    bytes.extend_from_slice(&data.e_cs.to_le_bytes());
    bytes.extend_from_slice(&data.e_lfarlc.to_le_bytes());
    bytes.extend_from_slice(&data.e_ovno.to_le_bytes());
    for i in 0..data.e_res.len() {
        bytes.extend_from_slice(&data.e_res[i].to_le_bytes());
    }
    bytes.extend_from_slice(&data.e_oemid.to_le_bytes());
    bytes.extend_from_slice(&data.e_oeminfo.to_le_bytes());
    for i in 0..data.e_res2.len() {
        bytes.extend_from_slice(&data.e_res2[i].to_le_bytes());
    }
    bytes.extend_from_slice(&data.e_lfanew.to_le_bytes());
    
    return bytes;
}