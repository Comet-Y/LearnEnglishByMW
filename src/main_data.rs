use serde::{Deserialize,Serialize};
use crate::other_structure;
use crate::sense_organization;
use crate::text_decoration::*;
use crate::libs::*;

#[derive(Deserialize, Serialize, PartialEq, Debug, Default, Clone)]
pub struct Data{
    pub meta:other_structure::Meta,//メタ情報 Entry metadata //ok
    pub hom:Option<i32>,//同じつづりで違う意味の言葉を見分けるためのid. Homograph //ok
    pub hwi:other_structure::Hwi,//見出し語。発音もあるかも Headword information //ok
    pub ahws:Option<Vec<other_structure::Ahw>>,//Alternate headwords //ok
    pub vrs:Option<Vec<other_structure::VrsObject>>,//Variants //ok
    pub fl:Option<String>,//nounとかadjectiveとか. Functional label //ok
    pub lbs:Option<Vec<String>>,//注釈みたいな.普通キャピタライズされるとか General labels  //ok  
    pub sls:Option<Vec<String>>,//使われる分野 Subject/Status labels //ok
    pub ins:Option<Vec<other_structure::InsObject>>,//活用形 Inflections //ok
    pub cxs:Option<Vec<other_structure::CxsObject>>,//headword より有名なスペリング Cognate cross-reference //ok
    pub def:Option<Vec<sense_organization::DefObject>>,//意味 Definition section //ok
    pub uros:Option<Vec<other_structure::UrosObject>>,//ok
    pub dros:Option<Vec<other_structure::DrosObject>>,//ok
    pub dxnls:Option<Vec<String>>,//ok
    pub usages:Option<Vec<other_structure::UsagesObject>>,//ok
    pub syns:Option<Vec<other_structure::SynsObject>>,//ok
    pub quotes:Option<Vec<other_structure::QuotesObject>>,//ok
    pub art:Option<other_structure::Art>,//ok
    pub table:Option<other_structure::Table>,//ok
    pub et:Option<Vec<other_structure::EtObject>>,//ok
    pub date:Option<String>,//ok
    pub shortdef:Option<Vec<String>>,//
}   


impl Data{
    pub fn head_word(&self)->String{
        let mut head_word=self.hwi.hw.clone();
        if let Some(hom)=self.hom{
            head_word=format!("{}:{}",head_word,hom);
        }
        let mut other_header_data=Vec::new();//functional labelと発音
        option_add_change_last(self.fl.clone(),&mut other_header_data,|s|{italic(s)});
        option_add_vector_change_last(self.hwi.prs.clone(),&mut other_header_data,format_prs);

        format!("{} {}",header(&head_word,2),bold(&clamp_vector(&other_header_data,"(div-head_word-other)&nbsp;")))
    }

    pub fn definition(&self)->Option<String>{
        option_output_vector(self.def.clone(),|s|{format!("{}:<br>{}<br>",bold("definition"),s)},"(div-def-output)<br>")
    }

    pub fn alternative_head_words(&self)->Option<Vec<String>>{
        if let Some(ahws)=&self.ahws{
                Some(ahws.iter().map(|ahw| {ahw.hw.clone()}).collect())
        }
        else{
            None
        }
    }

    pub fn variants(&self)->Option<Vec<other_structure::VrsObject>>{
        self.vrs.clone()
    }

    pub fn lbs(&self)->Option<String>{
        option_output_vector(self.lbs.clone(),|s|{format!("{}{}",bold("note&nbsp;:&nbsp"),s)},"(div-lbs-output)")
    }

    pub fn sls(&self)->Option<String>{
        option_output_vector(self.sls.clone(),|s|{format!("{}&nbsp;:&nbsp;{}",bold("used in"),s)},"(div-sls-output)")
    }

    pub fn ins(&self)->Option<String>{
        option_output_vector(self.ins.clone(),|s|{format!("{}&nbsp;:&nbsp;{}",bold("Inflections"),format_ins(s))},"(div-ins-output)")
    }
    pub fn cxs(&self)->Option<String>{

        option_output_vector(self.cxs.clone(),|s|{format!("{}",s)},"(div-cxs-output)")
    }

    pub fn uros(&self)->Option<String>{
        option_output_vector(self.uros.clone(),|s|{format!("{}",s)},"(div-uros-output)<br>")
    }

    pub fn dros(&self)->Option<String>{
        option_output_vector(self.dros.clone(),|s|{format!("{}",s)},"(div-dros-output)<br>")
    }

    pub fn dxnls(&self)->Option<String>{
        option_output_vector(self.dxnls.clone(),|s|{format!("{}",s)},"(div-dxnl-output)")
    }

    pub fn usages(&self)->Option<String>{
        option_output_vector(self.usages.clone(),|s|{format!("{}&nbsp;:&nbsp;{}",bold("Usages:"),s)},"(div-usages-output)")
    }

    pub fn syns(&self)->Option<String>{
        option_output_vector(self.syns.clone(),|s|{format!("{}",s)},"(div-syns-output)<br>")
    }

    pub fn quotes(&self)->Option<String>{
        option_output_vector(self.quotes.clone(),|s|{format!("{}&nbsp;:&nbsp;{}",bold("Quotes"),s)},"(div-quotes-output)<br>")
    }

    pub fn art(&self)->Option<String>{
        option_output(self.art.clone(),|s|{format!("{}&nbsp;:&nbsp;{}",bold("Artwork"),s)})
    }

    pub fn table(&self)->Option<String>{
        option_output(self.table.clone(),|s|{format!("{}",s)})
    }

    pub fn et(&self)->Option<String>{
        option_output_vector(self.et.clone(),|s|{format!("{}&nbsp;:&nbsp;{}",bold("Etymology"),s)},"(div-et-output)")
    }

    pub fn date(&self)->Option<String>{
        option_output(self.date.clone(),|s|{format!("{}&nbsp;:&nbsp;{}",bold("First Known Use"),s)})
    }

    pub fn shortdef(&self)->Option<String>{
        option_output_vector(self.shortdef.clone(),|s|{format!("{}&nbsp;&nbsp;<br>{}",bold("Short Definition"),s)},"(div-shortdef-output)<br>")
    }

    pub fn uros_and_dros(&self)->Option<String>{
        let mut uros_and_dros_v=Vec::new();
        option_add(self.uros(),&mut uros_and_dros_v);
        option_add(self.dros(),&mut uros_and_dros_v);
        if uros_and_dros_v.is_empty(){
            None
        }else{
            Some(format!("{}{}",bold("Related Words and Phrases<br>"),clamp_vector(&uros_and_dros_v,"(div-uros_and_dros-output)")))
        }
    }
    pub fn all(&self)->String{
        let mut product=Vec::new();
        
        product.push(self.head_word());
        option_add(self.definition(),&mut product);
        option_add_vector(self.alternative_head_words(),&mut product);
        option_add_vector(self.variants(),&mut product);
        option_add(self.lbs(),&mut product);
        option_add(self.sls(),&mut product);
        option_add(self.ins(),&mut product);
        option_add(self.cxs(),&mut product);
        option_add(self.uros_and_dros(),&mut product);
        option_add(self.dxnls(),&mut product);
        option_add(self.usages(),&mut product);
        option_add(self.syns(), &mut product);
        option_add(self.quotes(),&mut product);
        // option_add(self.art(),&mut product);
        option_add(self.table(),&mut product);
        option_add(self.et(),&mut product);
        option_add(self.date(),&mut product);
        // option_add(self.shortdef(),&mut product);
        clamp_vector_change(&product,"(div-main)<br>",|s|{format!("{}",s)})

    }
}
    
