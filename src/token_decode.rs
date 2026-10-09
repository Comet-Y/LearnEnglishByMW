use crate::text_decoration::*;
use crate::libs::*;
use anyhow::Result;
enum DecodeRtTokens{
    CS(String),//Common String
    ST(StartTokens),
    ET(EndTokens),
    SGT(SingleTokens),
    CRT(CrossReferenceTokens),
    DT(DataSenseToken),
}

enum CrossReferenceTokens{
    ALink(String),
    DLink(String,Option<String>),
    ILink(String,Option<String>),
    EtLink(String,Option<String>),
    Mat(String,Option<String>),
    Sx(String,Option<String>,Option<String>),
    Dxt(String,String,String),
}
impl std::str::FromStr for CrossReferenceTokens{
    type Err=();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let splitted:Vec<&str>=s.split("|").collect();
        match splitted.first(){
            Some(&"a_link")=>Ok(Self::ALink(splitted[1].to_string())),
            Some(&"d_link")=>Ok(Self::DLink(splitted[1].to_string(),splitted.get(2).map(ToString::to_string))),
            Some(&"i_link")=>Ok(Self::ILink(splitted[1].to_string(),splitted.get(2).map(ToString::to_string))),
            Some(&"et_link")=>Ok(Self::EtLink(splitted[1].to_string(),splitted.get(2).map(ToString::to_string))),
            Some(&"mat")=>Ok(Self::Mat(splitted[1].to_string(),splitted.get(2).map(ToString::to_string))),
            Some(&"sx")=>Ok(Self::Sx(splitted[1].to_string(),splitted.get(2).map(ToString::to_string),splitted.get(3).map(ToString::to_string))),
            Some(&"dxt")=>Ok(Self::Dxt(splitted[1].to_string(),splitted[2].to_string(),splitted[3].to_string())),
            _=>Err(())
        }
    }
}

impl std::fmt::Display for CrossReferenceTokens{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self{
            Self::ALink(a)=>write!(f,"{}",a),
            Self::DLink(a,ob )=>write!(f,"{} {}",a,ob.clone().unwrap_or("".to_string())),
            Self::ILink(a,ob )=>write!(f,"{} {}",a,ob.clone().unwrap_or("".to_string())),
            Self::EtLink(a,ob )=>write!(f,"{} {}",a,ob.clone().unwrap_or("".to_string())),
            Self::Mat(a,ob )=>write!(f,"{} {}",a,ob.clone().unwrap_or("".to_string())),
            Self::Sx(a,ob ,oc )=>write!(f,"{} {} {}",a,ob.clone().unwrap_or("".to_string()),oc.clone().unwrap_or("".to_string())),
            Self::Dxt(a,b ,c )=>write!(f,"{} {} {}",a,b,c),
        }
    }
}
enum StartTokens{
    B,
    Inf,
    It,
    Sc,
    Sup,
    Gloss,
    Parahw,
    Phrase,
    Qword,
    Wi,
    Dx,
    Dxdef,
    Dxety,
    Ma
}

impl std::str::FromStr for StartTokens{
    type Err=();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s{
            "b"=>Ok(Self::B),
            "inf"=>Ok(Self::Inf),
            "it"=>Ok(Self::It),
            "sc"=>Ok(Self::Sc),
            "sup"=>Ok(Self::Sup),
            "gloss"=>Ok(Self::Gloss),
            "parahw"=>Ok(Self::Parahw),
            "phrase"=>Ok(Self::Phrase),
            "qword"=>Ok(Self::Qword),
            "wi"=>Ok(Self::Wi),
            "dx"=>Ok(Self::Dx),
            "dx_def"=>Ok(Self::Dxdef),
            "dx_ety"=>Ok(Self::Dxety),
            "ma"=>Ok(Self::Ma),
            _=>Err(())
        }
    }
}

impl StartTokens{
    fn decorate_string(&self,s:String)->String{
        match self{
            Self::B=>bold(&s),
            Self::Inf=>format!("<sub>{}</sub>",s),//下付き文字にしたい
            Self::It=>italic(&s),
            Self::Sc=>s,//小さい大文字にしたい
            Self::Sup=>format!("<sup>{}</sup>",s),//上つき文字にしたい
            Self::Gloss=>text_color(&s,(200,0,0,100)),
            Self::Parahw=>bold(&s),
            Self::Phrase=>text_color(&s,(200,0,0,100)),
            Self::Qword=>format!("\"{}\"",s),
            Self::Wi=>bold(&s),
            Self::Dx=>text_color(&s,(200,0,0,100)),
            Self::Dxdef=>text_color(&s,(200,100,0,100)),
            Self::Dxety=>text_color(&s,(200,0,0,100)),
            Self::Ma=>text_color(&s,(200,0,0,100)),

    }
}
}
enum EndTokens{
    B,
    Inf,
    It,
    Sc,
    Sup,
    Gloss,
    Parahw,
    Phrase,
    Qword,
    Wi,
    Dx,
    Dxdef,
    Dxety,
    Ma
}

impl std::str::FromStr for EndTokens{
    type Err=();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s{
            "/b"=>Ok(Self::B),
            "/inf"=>Ok(Self::Inf),
            "/it"=>Ok(Self::It),
            "/sc"=>Ok(Self::Sc),
            "/sup"=>Ok(Self::Sup),
            "/gloss"=>Ok(Self::Gloss),
            "/parahw"=>Ok(Self::Parahw),
            "/phrase"=>Ok(Self::Phrase),
            "/qword"=>Ok(Self::Qword),
            "/wi"=>Ok(Self::Wi),
            "/dx"=>Ok(Self::Dx),
            "/dx_def"=>Ok(Self::Dxdef),
            "/dx_ety"=>Ok(Self::Dxety),
            "/ma"=>Ok(Self::Ma),
            _=>Err(())
        }
    }
}
enum SingleTokens{
    Bc,
    Ldquo,
    Rdquo,
}
impl std::str::FromStr for SingleTokens{
    type Err=();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s{
            "bc"=>Ok(Self::Bc),
            "ldquo"=>Ok(Self::Ldquo),
            "rdquo"=>Ok(Self::Rdquo),
            _=>Err(())
        }
    }
}

impl std::fmt::Display for SingleTokens{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self{
            Self::Bc=>write!(f,"{}",bold(":")),
            Self::Ldquo=>write!(f,"\""),
            Self::Rdquo=>write!(f,"\"")
        }
    }
}


#[derive(Debug)]
enum DataSenseToken{
    Dt(Option<String>,Option<String>,Option<String>,Option<String>)
}

impl std::str::FromStr for DataSenseToken{
    type Err=();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let splitted:Vec<&str>=s.split("|").collect();
        if splitted[0]=="ds"{
            Ok(Self::Dt(splitted.get(1).map(ToString::to_string),
                    splitted.get(2).map(ToString::to_string),
                    splitted.get(3).map(ToString::to_string),
                    splitted.get(4).map(ToString::to_string)))
        }   else{
            Err(())
        }
    }
}

impl std::fmt::Display for DataSenseToken{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self{
            Self::Dt(vd,bold_sn,lower_sn,parenthese_sn)=>{
                let verb_dividers=match vd.clone().unwrap_or("none".to_string()).as_str(){
                    "t"=>"&nbsp;transitive&nbsp;",
                    "i"=>"&nbsp;intransitive&nbsp;",
                    _=>""
                };
                let mut sense_number=Vec::new();
                option_add(bold_sn.clone(),&mut sense_number);
                option_add(lower_sn.clone(),&mut sense_number);
                option_add(parenthese_sn.clone(),&mut sense_number);
                write!(f,"{}{}",verb_dividers,format_sn(&clamp_vector(&sense_number,""),false))
            }
        }
    }
}
impl std::str::FromStr for DecodeRtTokens{
    type Err=();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Ok(crt)=s.parse::<CrossReferenceTokens>(){
            return Ok(Self::CRT(crt));
        }
        if let Ok(st)=s.parse::<StartTokens>(){
            return Ok(Self::ST(st));
        }
        if let Ok(et)=s.parse::<EndTokens>(){
            return Ok(Self::ET(et));
        }
        if let Ok(sgt)=s.parse::<SingleTokens>(){
            return Ok(Self::SGT(sgt));
        }
        if let Ok(dt)=s.parse::<DataSenseToken>(){
            return Ok(Self::DT(dt));
        }
        Ok(Self::CS(s.to_string()))
    }
}

fn read_str(str:&str)->Vec<DecodeRtTokens>{
    let mut in_parenthese=false;
    let splitted:Vec<&str>=str.split(r#"{"#).collect::<Vec<&str>>().iter().map(|s|{s.split("}").collect::<Vec<&str>>()}).collect::<Vec<Vec<&str>>>().concat();
    let mut ret=Vec::new();
    for s in splitted{
        if !in_parenthese{
            ret.push(DecodeRtTokens::CS(s.to_string()));
        }
        else{
            ret.push(s.parse::<DecodeRtTokens>().unwrap());
        }
        in_parenthese=!in_parenthese;
    }
    ret
}


fn to_string(drt:Vec<DecodeRtTokens>)->String{
        let mut ret=String::new();
        let mut starts:Vec<StartTokens>=Vec::new();
        for d in drt{
            match d{
                DecodeRtTokens::CS(mut s)=>{
                    for start in &starts{
                        s=start.decorate_string(s);
                    }
                    ret.push_str(&s);
                },
                DecodeRtTokens::ST(st)=>{
                    starts.push(st);
                },
                DecodeRtTokens::ET(_et)=>{
                    starts.pop();
                },
                DecodeRtTokens::SGT(sgt)=>{
                    ret.push_str(&sgt.to_string());
                },
                DecodeRtTokens::CRT(crt)=>{
                    ret.push_str(&crt.to_string());
                },
                DecodeRtTokens::DT(dt)=>{
                    ret.push_str(&dt.to_string());
                },
            }
        }

        ret

}

pub fn change_string(str:&str)->String{
    // println!("before:{}",str);
    let after=to_string(read_str(str));
    // println!("after:{}",after);
    after

}