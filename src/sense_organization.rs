use serde::{Deserialize,Serialize};
use crate::other_structure;
use crate::tags;
use itertools::Itertools;
use crate::token_decode;
use crate::libs::*;
#[derive(Deserialize, Serialize, PartialEq, Debug, Default, Clone)]
pub  struct DefObject{//Defの要素。Vec<DefObject>でdefになる
    pub vd:Option<String>,
    pub sls:Option<Vec<String>>,
    pub sseq:Vec<Vec<SseqObject>>,
}
impl std::fmt::Display for DefObject{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut def_object=Vec::new();
        option_add(self.vd.clone(),&mut def_object);
        option_add_vector(self.sls.clone(),&mut def_object);
        def_object.push(clamp_vector_2d(&self.sseq,"(div-DefObject::Sseq::1<br>)","(div-DefObject::Sseq::2<br>)"));
        write!(f,"{}",clamp_vector(&def_object,"(div-DefObject)<br>"))
    }
}

#[derive(Deserialize, Serialize, PartialEq, Debug, Clone)]
#[serde(untagged)]
pub enum SseqObject{//Deserialize手動実装。Vec<Vec<SseqObject>>でsseqになる 
    Sense(tags::TagSense,Sense),
    Sen(tags::TagSen,Sen),
    Bs(tags::TagBs,Bs),
    Pseq(tags::TagPseq,Vec<PseqObject>),
}

impl std::fmt::Display for SseqObject{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self{
            Self::Sense(_,sense)=>write!(f,"{}",sense.to_string()),
            Self::Sen(_,sen)=>write!(f,"{}",sen.to_string()),
            Self::Bs(_,bs )=>write!(f,"{}",bs.to_string()),
            Self::Pseq(_,pseq)=>write!(f,"{}",clamp_vector(&pseq,"(div-SseqObject::Pseq)<br>"))
        }
    }
}

#[derive(Deserialize, Serialize, PartialEq, Debug, Default, Clone)]
pub struct Sense{
    pub dt:Vec<DtObject>,
    pub et:Option<Vec<other_structure::EtObject>>,
    pub ins:Option<Vec<other_structure::InsObject>>,
    pub lbs:Option<Vec<String>>,
    pub prs:Option<Vec<other_structure::Prs>>,
    pub sdsense:Option<SdsenseObject>,
    pub sgram:Option<String>,
    pub sls:Option<Vec<String>>,
    pub sn:Option<String>,
    pub vrs:Option<Vec<other_structure::VrsObject>>
}

impl std::fmt::Display for Sense{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut sense=Vec::new();
        option_add_change_last(self.sn.clone(),&mut sense,|s|{format_sn(s,true)});
        sense.push(clamp_vector(&self.dt,"(div-Sense::dt)"));
        option_add_vector(self.et.clone(),&mut sense);
        option_add_vector_change_last(self.ins.clone(),&mut sense,format_ins);
        option_add_vector(self.lbs.clone(),&mut sense);
        option_add_vector(self.prs.clone(),&mut sense);
        option_add(self.sdsense.clone(),&mut sense);
        option_add(self.sgram.clone(),&mut sense);
        option_add_vector(self.sls.clone(),&mut sense);
        option_add_vector(self.vrs.clone(),&mut sense);
        write!(f,"{}",clamp_vector(&sense,"(div-Sense)"))
        
    }
}
#[derive(Deserialize, Serialize, PartialEq, Debug, Clone)]
#[serde(untagged)]
pub enum DtObject{//Deserialize手動実装しないと
    Text(tags::TagText,String),
    Bnw(tags::TagBnw,other_structure::Bnw),
    Ca(tags::TagCa,other_structure::Ca),
    Ri(tags::TagRi,Vec<other_structure::RiObject>),
    Snote(tags::TagSnote,Vec<other_structure::SnoteObject>),
    Uns(tags::TagUns,Vec<Vec<other_structure::UnsObject>>),
    Vis(tags::TagVis,Vec<other_structure::VisT>),
}
impl std::fmt::Display for DtObject{
    fn fmt(&self,f: &mut std::fmt::Formatter<'_>)->Result<(),std::fmt::Error>{
        match self{
            DtObject::Text(_,text)=>write!(f,"{}",token_decode::change_string(text)),
            DtObject::Bnw(_,bnw)=>write!(f,"{}",bnw.to_string()),
            DtObject::Ca(_,ca)=>write!(f,"{}",ca.to_string()),
            DtObject::Ri(_,ri)=>write!(f,"{}",clamp_vector(&ri,"(div-DtObject::Ri)")),
            DtObject::Snote(_,snote)=>write!(f,"{}",snote.iter().map(ToString::to_string).join("(div-DtObject::Snote)")),
            DtObject::Uns(_,uns)=>write!(f,"{}",uns.iter().map(|u|{u.iter().map(ToString::to_string).join("(div-DtObject::Uns::1)")}).join("(div-DtObject::Uns::2)")),
            DtObject::Vis(_,vis)=>write!(f,"{}",format_vis(&vis,"(div-DtObject::Vis)"))
        }
    }
}

#[derive(Deserialize, Serialize, PartialEq, Debug, Default, Clone)]
pub struct SdsenseObject{
    pub sd:String,
    pub dt:Vec<DtObject>,
    pub et:Option<Vec<other_structure::EtObject>>,
    pub ins:Option<Vec<other_structure::InsObject>>,
    pub lbs:Option<Vec<String>>,
    pub prs:Option<Vec<other_structure::Prs>>,
    pub sgram:Option<String>,
    pub sls:Option<Vec<String>>,
    pub vrs:Option<Vec<other_structure::VrsObject>>
}

impl std::fmt::Display for SdsenseObject{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut sdsense_object=Vec::new();
        sdsense_object.push(self.sd.clone());
        sdsense_object.push(self.dt.iter().format(" ").to_string());
        option_add_vector(self.et.clone(),&mut sdsense_object);
        option_add_vector_change_last(self.ins.clone(),&mut sdsense_object,format_ins);
        option_add_vector(self.lbs.clone(),&mut sdsense_object);
        option_add_vector(self.prs.clone(),&mut sdsense_object);
        option_add(self.sgram.clone(),&mut sdsense_object);
        option_add_vector(self.sls.clone(),&mut sdsense_object);
        option_add_vector(self.vrs.clone(),&mut sdsense_object);
        write!(f,"{}",clamp_vector(&sdsense_object,"(div-SdSenseObject)"))
    }
}

#[derive(Deserialize, Serialize, PartialEq, Debug, Default, Clone)]
pub struct Sen{
    pub et:Option<Vec<other_structure::EtObject>>,
    pub ins:Option<Vec<other_structure::InsObject>>,
    pub lbs:Option<Vec<String>>,
    pub prs:Option<Vec<other_structure::Prs>>,
    pub sgram:Option<String>,
    pub sls:Option<Vec<String>>,
    pub sn:Option<String>,
    pub vrs:Option<Vec<other_structure::VrsObject>>
}

impl std::fmt::Display for Sen{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut sen=Vec::new();
        option_add_change_last(self.sn.clone(),&mut sen,|s|{format_sn(s,true)});
        option_add_vector(self.et.clone(),&mut sen);
        option_add_vector_change_last(self.ins.clone(),&mut sen,format_ins);
        option_add_vector(self.lbs.clone(),&mut sen);
        option_add_vector(self.prs.clone(),&mut sen);
        option_add(self.sgram.clone(),&mut sen);
        option_add_vector(self.sls.clone(),&mut sen);
        option_add_vector(self.vrs.clone(),&mut sen);
        write!(f,"{}",sen.iter().format("(div-Sen)").to_string())
    }
}

#[derive(Deserialize, Serialize, PartialEq, Debug, Default, Clone)]
pub struct Bs{
    pub sense:Sense
}

impl std::fmt::Display for Bs{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f,"{}",self.sense.to_string())
    }
}

#[derive(Deserialize, Serialize, PartialEq, Debug, Clone)]
#[serde(untagged)]
pub enum PseqObject{
    Bs(tags::TagBs,Bs),
    Sense(tags::TagSense,Sense)
}

impl std::fmt::Display for PseqObject{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self{
            PseqObject::Bs(_,bs)=>write!(f,"{}",bs.to_string()),
            PseqObject::Sense(_,sense )=>write!(f,"{}",sense.to_string())
        }
    }
}


