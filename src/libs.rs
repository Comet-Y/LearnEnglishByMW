use crate::token_decode;
use itertools::Itertools;
use crate::other_structure;
use crate::text_decoration::*;
pub fn option_add_vector<T>(vdata:Option<Vec<T>>,vstring:&mut Vec<String>)->bool
where T:std::fmt::Display
{
    if let Some(vdata)=vdata{
        vstring.push(token_decode::change_string(&vdata.iter().format(" ").to_string()));
        true
    }else{
        false
    }
}

pub fn option_add<T>(data:Option<T>,vstring:&mut Vec<String>)->bool
where T:std::fmt::Display{
    if let Some(data)=data{
        vstring.push(token_decode::change_string(&data.to_string()));
        true
    }else {
        false
    }
}

pub fn format_sn(sn:&str,to_bold:bool)->String{
    let splitted=sn.split(" ").collect::<Vec<&str>>();
    let mut sn_format:(Option<&str>,Option<&str>,Option<&str>)=(None,None,None);
    for s in splitted{
        if s.chars().all(|c| c.is_numeric()){
            sn_format.0=Some(s);
        }
        else if s.chars().all(|c| c.is_alphabetic()){
            sn_format.1=Some(s);
        }
        else{
            sn_format.2=Some(s);
        }
    }
    if to_bold{
        bold(&format!("<table style=\"display:inline-table; vertical-align:-6px;border-collapse:collapse;table-layout:fixed;\"><tr><td style=\"width:1em;margin:0;padding:0;\">{}</td><td style=\"width:1em;margin:0;padding:0;\">{}</td><td  style=\"width:1em;margin:0;padding:0;\">{}</td></table>",sn_format.0.unwrap_or(""),sn_format.1.unwrap_or(""),sn_format.2.unwrap_or("")))
    }else{
    format!("<table style=\"display:inline-table; vertical-align:-6px;border-collapse:collapse;table-layout:fixed;\"><tr><td style=\"width:1em;margin:0;padding:0;\">{}</td><td style=\"width:1em;margin:0;padding:0;\">{}</td><td  style=\"width:1em;margin:0;padding:0;\">{}</td></table>",sn_format.0.unwrap_or(""),sn_format.1.unwrap_or(""),sn_format.2.unwrap_or(""))
    }
    // bold(&format!("{}{}{}",sn_format.0.unwrap_or("-"),sn_format.1.unwrap_or(if gone>=1 {"-"}else{"&nbsp;&nbsp;"}),sn_format.2.unwrap_or("&nbsp;&nbsp;&nbsp;&nbsp;")))
}
pub fn format_ins(ins:& str)->String{
    if ins==""{
        ins.to_string()
    }else{
        ins.chars().skip(1).collect::<String>()
    }
}

pub fn change_last<T,F>(vdata:&mut Vec<T>,f:F)
where F:Fn(&T)->T{
    let len=vdata.len();
    vdata[len-1]=f(&vdata[len-1]);
}

pub fn option_add_change_last<T,F>(data:Option<T>,vdata:&mut Vec<String>,f:F)
where 
T:std::fmt::Display,
F:Fn(&str)->String
    {
    if option_add(data,vdata){
        let len=vdata.len();
        vdata[len-1]=f(&vdata[len-1]);
    }
    }

pub fn option_add_vector_change_last<T,F>(data:Option<Vec<T>>,vdata:&mut Vec<String>,f:F)
where 
T:std::fmt::Display,
F:Fn(&str)->String
    {
    if option_add_vector(data,vdata){
        let len=vdata.len();
        vdata[len-1]=f(&vdata[len-1]);
    }
    }

pub fn option_output<T,F>(data:Option<T>,f:F)->Option<String>
where T:std::fmt::Display,
      F:Fn(&str)->String{
        if let Some(data)=data{
            Some(f(&data.to_string()))
        }
        else{
            None
        }

}

pub fn option_output_vector<T,F>(data:Option<Vec<T>>,f:F,div:&str)->Option<String>
where T:std::fmt::Display,
        F:Fn(&str)->String
        {
        if let Some(data)=data{
            Some(f(&clamp_vector(&data,div)))
        }else{
            None
        }
    }
    
pub fn append_em_dash(str:&str)->String{
    format!("—{}",str)
}

pub fn clamp_vector<T>(vdata:&Vec<T>,div:&str)->String
where T:std::fmt::Display{
    vdata.iter().format(&extract_tag(div)).to_string()
}
pub fn clamp_vector_2d<T>(vdata:&Vec<Vec<T>>,div1:&str,div2:&str)->String
    where T:std::fmt::Display{
    clamp_vector(&vdata.iter().map(|s|{clamp_vector(s,div1)}).collect_vec(),div2)
}
pub fn extract_tag(str:&str)->String{
    str.split(|c|c=='<'||c=='>').fold(Vec::new(),|mut v:Vec<String>,s|{v.push(if v.len()%2==0{s.split(|c|c=='&'||c==';').skip(1).step_by(2).map(|s|{format!("&{};",s)}).collect::<String>()}else{format!("<{}>",s)}); v}).join("")
}
pub fn clamp_vector_change<T,F>(vdata:&Vec<T>,div:&str,f:F)->String
where T:std::fmt::Display,
      F:Fn(&str)->String{
    f(&clamp_vector(&vdata,div))
}

pub fn format_vis(vis:&Vec<other_structure::VisT>,div:&str)->String{
    clamp_vector_change(vis,div,|s|{format!("&lt;{}&gt;",s)})
}
