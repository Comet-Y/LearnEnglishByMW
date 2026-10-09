pub fn bold(str:&str)->String{
    format!("<b>{}</b>",str)
}
pub fn header(str:&str,header_id:u8)->String{
    assert!(header_id<=5);
    format!("<h{}>{}</h{}>",header_id,str,header_id)
}
pub fn italic(str:&str)->String{
    format!("<i>{}</i>",str)
}
pub fn text_color(str:&str,rgba:(u8,u8,u8,u8))->String{
    format!("<span style=\"color:rgb({} {} {} / {}%);\">{}</span>",rgba.0,rgba.1,rgba.2,rgba.3,str)
}