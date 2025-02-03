use super::structs::{self, word_to_string};

pub fn print(header: structs::DosHeader) -> Result<String, Box<dyn std::error::Error>> {
    let mut words = String::new();
    words.push_str("DosHeader { ");
    add_word(&mut words, "e_magic", header.e_magic, true)?;
    add_word_number(&mut words, "e_cblp", header.e_cblp, true)?;
    add_word_number(&mut words, "e_cp", header.e_cp, true)?;
    add_word_number(&mut words, "e_crlc", header.e_crlc, true)?;
    add_word_number(&mut words, "e_cparhdr", header.e_cparhdr, true)?;
    add_word_number(&mut words, "e_minalloc", header.e_minalloc, true)?;
    add_word_number(&mut words, "e_maxalloc", header.e_maxalloc, true)?;
    add_word_number(&mut words, "e_ss", header.e_ss, true)?;
    add_word_number(&mut words, "e_sp", header.e_sp, true)?;
    add_word_number(&mut words, "e_csum", header.e_csum, true)?;
    add_word_number(&mut words, "e_ip", header.e_ip, true)?;
    add_word_number(&mut words, "e_cs", header.e_cs, true)?;
    add_word_number(&mut words, "e_lfarlc", header.e_lfarlc, true)?;
    add_word_number(&mut words, "e_ovno", header.e_ovno, true)?;
    add_word_list_number(&mut words, "e_res", header.e_res.to_vec(), true)?;
    add_word(&mut words, "e_oemid", header.e_oemid, true)?;
    add_word(&mut words, "e_oeminfo", header.e_oeminfo, true)?;
    add_word_list_number(&mut words, "e_res2", header.e_res2.to_vec(), true)?;
    // e_lfanew 是一个u32数值,不需要转换
    words.push_str("e_lfanew");
    words.push_str(": ");
    words.push_str(&header.e_lfanew.to_string());
    words.push_str("}");

    Ok(words)    
}
fn add_word(words:&mut String, name:&str, value:u16,is_add:bool)-> Result<(), Box<dyn std::error::Error>>{
    words.push_str(name);
    words.push_str(": ");
    words.push_str(&word_to_string(value)?);
    if is_add{
        words.push_str(", ");
    } else {
        words.push_str(" ");
    }
    Ok(())
}
fn add_word_number(words:&mut String, name:&str, value:u16,is_add:bool)-> Result<(), Box<dyn std::error::Error>>{
    words.push_str(name);
    words.push_str(": ");
    words.push_str(value.to_string().as_str());
    if is_add{
        words.push_str(", ");
    } else {
        words.push_str(" ");
    }
    Ok(())
}

fn add_word_list_number(words:&mut String, name:&str, value:Vec<u16>,is_add:bool)-> Result<(), Box<dyn std::error::Error>>{
    words.push_str(name);
    words.push_str(": ");
    words.push_str("[");
    // 循环列表,并获取顺序
    let max = value.len()-1;
    for (i, v) in value.iter().enumerate(){
        words.push_str(v.to_string().as_str());
        // 判断是否是最后一个值
        if i != max{
            words.push_str(", ");
        }
    }
    words.push_str("]");
    if is_add{
        words.push_str(", ");
    } else {
        words.push_str(" ");
    }
    Ok(())
}