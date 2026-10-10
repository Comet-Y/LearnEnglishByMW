use serde::{Deserialize,Serialize};
use crate::sense_organization;
use crate::tags;
use crate::libs::*;
use crate::token_decode;
use crate::text_decoration::*;
use itertools::Itertools;
#[derive(Deserialize, Serialize, PartialEq, Debug, Default, Clone)]
pub struct Meta{
    pub id:String,
    pub uuid:String,
    pub sort:String,
    pub src:String,
    pub section:String,
    pub stems:Vec<String>,
    pub offensive:bool,
}
#[derive(Deserialize, Serialize, PartialEq, Debug, Default, Clone)]
pub struct Hwi{
    pub hw:String,
    pub prs:Option<Vec<Prs>>,
    pub psl:Option<String>
}
#[derive(Deserialize, Serialize, PartialEq, Debug, Default, Clone)]
pub struct Ahw{
    pub hw:String,
    pub prs:Option<Vec<Prs>>,
    pub psl:Option<String>
}

#[derive(Deserialize, Serialize, PartialEq, Debug, Default, Clone)]
pub struct InsObject{//Insの補助
    #[serde(rename="if")]
    pub if_:Option<String>,
    pub ifc:Option<String>,
    pub il:Option<String>,
    pub prs:Option<Vec<Prs>>,
    pub spl:Option<String>
}

impl std::fmt::Display for InsObject{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut ins_object=Vec::new();
        if let Some(il)=self.il.clone(){
            ins_object.push(il);
        }
        else{
            ins_object.push(",".to_string());
        }
        option_add(self.if_.clone(),&mut ins_object);
        write!(f,"{}",&clamp_vector(&ins_object,"(div-InsObject)"))
    }
}

#[derive(Deserialize, Serialize, PartialEq, Debug, Default, Clone)]
pub struct CxsObject{
    pub cxl:Option<String>,
    pub cxtis:Vec<CxsCxtisObject>
}
impl std::fmt::Display for CxsObject{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut cxs_object=Vec::new();
        if let Some(cxl)=self.cxl.clone(){
            cxs_object.push(italic(&cxl));
        }
        cxs_object.push(clamp_vector(&self.cxtis,"(div-CxsObject::Cxtis)&nbsp;"));
        write!(f,"{}",clamp_vector(&cxs_object,"(div-CxsObject)&nbsp;"))

    }
}
#[derive(Deserialize, Serialize, PartialEq, Debug, Default, Clone)]
pub struct CxsCxtisObject{
    pub cxl:Option<String>,
    pub cxr:Option<String>,
    pub cxt:Option<String>,
    pub cxn:Option<String>,
}

impl std::fmt::Display for CxsCxtisObject{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut cxs_cxtis_object=Vec::new();
        if let Some(cxn)=self.cxn.clone(){
            cxs_cxtis_object.push(format_sn(&cxn,true));
        }
        option_add(self.cxl.clone(),&mut cxs_cxtis_object);
        option_add(self.cxt.clone(),&mut cxs_cxtis_object);
        write!(f,"{}",cxs_cxtis_object.iter().format("(div-CxsCxtisObject)"))
    }
}
#[derive(Deserialize, Serialize, PartialEq, Debug, Default, Clone)]
pub struct Prs{
    pub mw:Option<String>,
    pub l:Option<String>,
    pub l2:Option<String>,
    pub pun:Option<String>,
    pub sound:Option<Sound>,
}
impl std::fmt::Display for Prs{
    fn fmt(&self,f: &mut std::fmt::Formatter<'_>)->Result<(),std::fmt::Error>{
        write!(f,"{}{}",self.mw.clone().unwrap_or("".to_string()),self.pun.clone().unwrap_or("".to_string()))
    }   
}


#[derive(Deserialize, Serialize, PartialEq, Debug, Default, Clone)]
#[serde(rename="reference=ref")]
pub struct Sound{
    pub audio:String,
    pub reference:Option<String>,
    pub stat:String,
}

#[derive(Deserialize, Serialize, PartialEq, Debug, Default, Clone)]
pub struct VrsObject{
    pub va:String,
    pub vl:Option<String>,
    pub prs:Option<Vec<Prs>>,
    pub spl:Option<String>
}
impl std::fmt::Display for VrsObject{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut vrs_object=Vec::new();
        option_add(self.vl.clone(),&mut vrs_object);
        vrs_object.push(self.va.clone());
        option_add_vector_change_last(self.prs.clone(),&mut vrs_object,format_prs);
        option_add(self.spl.clone(),&mut vrs_object);
        write!(f,"{}",clamp_vector(&vrs_object,"(div-VrsObject)&nbsp;"))
    }
}



#[derive(Deserialize, Serialize, PartialEq, Debug, Default, Clone)]
pub struct VisT{
    pub t:String,
    pub aq:Option<Aq>,
}
impl std::fmt::Display for VisT{
    fn fmt(&self,f:&mut std::fmt::Formatter<'_>)->Result<(),std::fmt::Error>{
        write!(f,"&lt;{}&gt;&nbsp;{}",&self.t,token_decode::change_string(&self.aq.clone().unwrap_or(Aq::default()).to_string()))
    }
}

#[derive(Deserialize, Serialize, PartialEq, Debug, Default, Clone)]
pub struct Aq{
    pub auth:Option<String>,
    pub source:Option<String>,
    pub aqdate:Option<String>,
    pub subsource:Option<AqSubsource>

}
impl std::fmt::Display for Aq{
    fn fmt(&self,f:&mut std::fmt::Formatter<'_>)->Result<(),std::fmt::Error>{
        let mut aq=Vec::new();
        if let Some(auth)=self.auth.clone(){
            aq.push(format!("author:{}",auth));
        }
        option_add(self.source.clone(),&mut aq);
        option_add(self.subsource.clone(),&mut aq);
        write!(f,"{}",clamp_vector(&aq,"(div-Aq)&nbsp;"))
    }
}

#[derive(Deserialize, Serialize, PartialEq, Debug, Default, Clone)]
pub struct AqSubsource{
    pub source:Option<String>,
    pub aqdate:Option<String>
}

impl std::fmt::Display for AqSubsource{
    fn fmt(&self,f:&mut std::fmt::Formatter<'_>)->Result<(),std::fmt::Error>{
        let mut aq_subsource=Vec::new();
        option_add(self.source.clone(),&mut aq_subsource);
        option_add(self.aqdate.clone(),&mut aq_subsource);
        write!(f,"{}",token_decode::change_string(&aq_subsource.iter().format("(div-AqSubsource)").to_string()))
    }
}
#[derive(Deserialize, Serialize, PartialEq, Debug, Clone)]
#[serde(untagged)]
pub enum RiObject{
    Riw(tags::TagRiw,RiRiw),
    Text(tags::TagText,String),
}

impl std::fmt::Display for RiObject{
    fn fmt(&self,f:&mut std::fmt::Formatter<'_>)->Result<(),std::fmt::Error>{
        match self{
            RiObject::Riw(tags::TagRiw::Riw,riw)=>{write!(f,"{}",riw.to_string())}
            RiObject::Text(tags::TagText::Text,text)=>write!(f,"{}",text)
        }
    }
}

#[derive(Deserialize, Serialize, PartialEq, Debug, Default, Clone)]
pub struct RiRiw{
    pub rie:String,
    pub prs:Option<Vec<Prs>>
} 

impl std::fmt::Display for RiRiw{
    fn fmt(&self,f:&mut std::fmt::Formatter<'_>)->Result<(),std::fmt::Error>{
        let mut ri_riw=Vec::new();
        ri_riw.push(self.rie.clone());
        option_add_vector_change_last(self.prs.clone(),&mut ri_riw,format_prs);
        write!(f,"{}",clamp_vector(&ri_riw,"(div-RiRiw)"))
    }
}

#[derive(Deserialize, Serialize, PartialEq, Debug, Default, Clone)]
pub struct Bnw{
    pub pname:Option<String>,
    pub sname:Option<String>,
    pub altname:Option<String>,
    pub prs:Option<Vec<Prs>>
}

impl std::fmt::Display for Bnw{
    fn fmt(&self,f: &mut std::fmt::Formatter<'_>)->Result<(),std::fmt::Error>{
        let mut bnw:Vec<String>=Vec::new();
        option_add_change_last(self.pname.clone(),&mut bnw,|s|{format!("first name:{}",s)});
        option_add_change_last(self.sname.clone(),&mut bnw,|s|{format!("sur name:{}",s)});
        option_add_change_last(self.altname.clone(),&mut bnw,|s|{format!("alternate name:{}",s)});
        option_add_vector_change_last(self.prs.clone(),&mut bnw,format_prs);
        write!(f,"{}",clamp_vector(&bnw,"(div-Bnw)"))
    }
}

#[derive(Deserialize, Serialize, PartialEq, Debug, Default, Clone)]
pub struct Ca{//dt
    pub intro:Option<String>,
    pub cats:Option<Vec<CatsObject>>
}

impl std::fmt::Display for Ca{
    fn fmt(&self,f:&mut std::fmt::Formatter<'_>)->Result<(),std::fmt::Error>{
        let mut ca=Vec::new();
        if let Some(intro)=self.intro.clone(){
            ca.push(intro);
        }
        option_add_vector(self.cats.clone(),&mut ca);
        write!(f,"{}",clamp_vector(&ca,"(div-ca)"))
    }
}
#[derive(Deserialize, Serialize, PartialEq, Debug, Default, Clone)]

pub struct CatsObject{//dt
    pub cat:Option<String>,
    pub catref:Option<String>,
    pub pn:Option<String>,
    pub prs:Option<Vec<Prs>>,
    pub psl:Option<String>
}
impl std::fmt::Display for CatsObject{
    fn fmt(&self,f:&mut std::fmt::Formatter<'_>)->Result<(),std::fmt::Error>{
        let mut cats_object:Vec<String>=Vec::new();
        option_add_change_last(self.pn.clone(),&mut cats_object,|s|{format!("({})",s)});
        option_add(self.psl.clone(),&mut cats_object);
        write!(f,"{}",clamp_vector(&cats_object,"(div-CatsObject)"))
    }
}

#[derive(Deserialize, Serialize, PartialEq, Debug, Clone)]
#[serde(untagged)]
pub enum SnoteObject{//Dt
    T(tags::TagT,String),
    Vis(tags::TagVis,Vec<VisT>),
    Ri(tags::TagRi,Vec<RiObject>)
}

impl std::fmt::Display for SnoteObject{
    fn fmt(&self,f:&mut std::fmt::Formatter<'_>)->Result<(),std::fmt::Error>{
        match self{
            SnoteObject::T(_,t)=>write!(f,"{}",t),
            SnoteObject::Vis(_,vis)=>write!(f,"{}",clamp_vector(vis,"(div-SnoteObject::Vis)<br>")),
            SnoteObject::Ri(_,ri)=>write!(f,"{}",clamp_vector(&ri,"(div-SnoteObject::Ri)"))
        }
        
    }
}

#[derive(Deserialize, Serialize, PartialEq, Debug, Clone)]
#[serde(untagged)]
pub enum UnsObject{//Vec<Vec<UnsObject>>でUns
    Text(tags::TagText,String),
    Vis(tags::TagVis,Vec<VisT>),
    Ri(tags::TagRi,Vec<RiObject>)
}
impl std::fmt::Display for UnsObject{
    fn fmt(&self,f:&mut std::fmt::Formatter<'_>)->Result<(),std::fmt::Error>{
        match self{
            UnsObject::Text(_,text)=>write!(f,"{}",token_decode::change_string(&text)),
            UnsObject::Vis(_,vis)=>write!(f,"{}",&clamp_vector(&vis,"(div-UnsObject::Vis)<br>")),
            UnsObject::Ri(_,ri)=>write!(f,"{}",&clamp_vector(&ri,"(div-UnsObject::ri)"))
        }
    }
}

#[derive(Deserialize, Serialize, PartialEq, Debug, Default, Clone)]
#[serde(rename_all="lowercase")]
pub struct UrosObject{
    pub ure:String,
    pub fl:String,
    pub utxt:Option<Vec<UrosUtxtObject>>,
    pub ins:Option<Vec<InsObject>>,
    pub lbs:Option<Vec<String>>,
    pub prs:Option<Vec<Prs>>,
    pub psl:Option<String>,
    pub sls:Option<Vec<String>>,
    pub vrs:Option<Vec<VrsObject>>
}
impl std::fmt::Display for UrosObject{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut uros_object=Vec::new();
        uros_object.push(append_em_dash(&bold(&self.ure)));
        uros_object.push(self.fl.clone());
        option_add_vector(self.utxt.clone(),&mut uros_object);
        option_add_vector(self.ins.clone(),&mut uros_object);
        option_add_vector(self.lbs.clone(),&mut uros_object);
        option_add_vector_change_last(self.prs.clone(),&mut uros_object,format_prs);
        option_add(self.psl.clone(),&mut uros_object);
        option_add_vector(self.sls.clone(),&mut uros_object);
        option_add_vector(self.vrs.clone(),&mut uros_object);
        write!(f,"{}",clamp_vector(&uros_object,"(div-UrosObject)&nbsp;"))

    }
}
#[derive(Deserialize, Serialize, PartialEq, Debug, Clone)]
#[serde(untagged)]
pub enum UrosUtxtObject{
    Vis(tags::TagVis,Vec<VisT>),
    Uns(tags::TagUns,Vec<Vec<UnsObject>>)
}
impl std::fmt::Display for UrosUtxtObject{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self{
            UrosUtxtObject::Vis(_,vis)=>write!(f,"{}",clamp_vector(vis,"(div-UrosUtxtObject::Vis)<br>")),
            UrosUtxtObject::Uns(_,uns )=>write!(f,"{}",clamp_vector_2d(&uns,"(div-UrosUtxtObject::Uns::1)","(div-UrosUtxtObject::Uns::2)"))
        }
    }
}
#[derive(Deserialize, Serialize, PartialEq, Debug, Default, Clone)]
pub struct DrosObject{
    pub drp:String,
    pub def:Vec<sense_organization::DefObject>,
    pub et:Option<Vec<EtObject>>,
    pub lbs:Option<Vec<String>>,
    pub prs:Option<Vec<Prs>>,
    pub psl:Option<String>,
    pub sls:Option<Vec<String>>,
    pub vrs:Option<Vec<VrsObject>>
}
impl std::fmt::Display for DrosObject{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut dros_object:Vec<String>=Vec::new();
        dros_object.push(clamp_vector(&self.def,"(div-DrosObject::def)"));
        dros_object.push(append_em_dash(&bold(&self.drp)));
        option_add_vector(self.et.clone(),&mut dros_object);
        option_add_vector(self.lbs.clone(),&mut dros_object);
        option_add_vector_change_last(self.prs.clone(),&mut dros_object,format_prs);
        option_add(self.psl.clone(),&mut dros_object);
        option_add_vector(self.sls.clone(),&mut dros_object);
        option_add_vector(self.vrs.clone(),&mut dros_object);
        write!(f,"{}",clamp_vector(&dros_object,"(div-DrosObject)"))
    }
}




#[derive(Deserialize, Serialize, PartialEq, Debug, Default, Clone)]
pub struct UsagesObject{
    pub pl:String,
    pub pt:Vec<UsagesPtObject>
}
impl std::fmt::Display for UsagesObject{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f,"{}{}",self.pl,clamp_vector(&self.pt, "(div-UsagesObject::pt)\n"))
    }
}

#[derive(Deserialize, Serialize, PartialEq, Debug, Clone)]
#[serde(untagged)]
pub enum UsagesPtObject{
    Text(tags::TagText,String),
    Vis(tags::TagVis,Vec<VisT>),
    Uarefs(tags::TagUarefs,UsagesUarefs)
}

impl std::fmt::Display for UsagesPtObject{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self{
            Self::Text(_,text)=>write!(f,"{}",text),
            Self::Vis(_,vis)=>write!(f,"{}",clamp_vector(vis,"(div-UsagesPtObject::Vis)<br>")),
            Self::Uarefs(_,uarefs )=>write!(f,"{}",uarefs.to_string())
        }
    }
}

#[derive(Deserialize, Serialize, PartialEq, Debug, Default, Clone)]
pub struct UsagesUarefs{
    pub uaref:String,
}

impl std::fmt::Display for UsagesUarefs{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f,"{}",self.uaref)
    }
}

#[derive(Deserialize, Serialize, PartialEq, Debug, Default, Clone)]
pub struct SynsObject{
    pub pl:String,
    pub pt:Vec<SynsPtObject>,
    sarefs:Option<Vec<String>>
}

impl std::fmt::Display for SynsObject{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut syns_object=Vec::new();
        syns_object.push(bold(&self.pl.clone()));
        syns_object.push(clamp_vector(&self.pt, "(div-SynsObject::pt)<br>"));
        option_add_vector_change_last(self.sarefs.clone(),&mut syns_object,|s|{format!("see in addition {}",s)});
        write!(f,"{}",clamp_vector(&syns_object,"(div-SynsObject)<br>"))
    }
}
#[derive(Deserialize, Serialize, PartialEq, Debug, Clone)]
#[serde(untagged)]
pub enum SynsPtObject{
    Text(tags::TagText,String),
    Vis(tags::TagVis,Vec<VisT>),
}
impl std::fmt::Display for SynsPtObject{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self{
        Self::Text(_,text)=>write!(f,"{}",text),
        Self::Vis(_,vis)=>write!(f,"{}",clamp_vector(vis,"(div-SynsPtObject)<br>"))
    }
    }

}
#[derive(Deserialize, Serialize, PartialEq, Debug, Default, Clone)]
pub struct QuotesObject{
    pub t:String,
    pub aq:Aq
}
impl std::fmt::Display for QuotesObject{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut quotes_object=Vec::new();
        quotes_object.push(self.t.clone());
        quotes_object.push(self.aq.to_string());
        write!(f,"{}",clamp_vector(&quotes_object,"(div-QuotesObjext)"))
    }
}


#[derive(Deserialize, Serialize, PartialEq, Debug, Default, Clone)]
pub struct Art{
    pub artid:String,
    pub capt:String,
}

impl std::fmt::Display for Art{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut art=Vec::new();
        art.push(self.artid.clone());
        art.push(self.capt.clone());
        write!(f,"{}",clamp_vector(&art,"(div-Art)"))

    }
}
#[derive(Deserialize, Serialize, PartialEq, Debug, Default, Clone)]
pub struct Table{
    pub tableid:String,
    pub displayname:String
}
impl std::fmt::Display for Table{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut table=Vec::new();
        table.push(self.tableid.clone());
        table.push(self.displayname.clone());
        write!(f,"{}",clamp_vector(&table,"(div-Table)"))
    }
}
#[derive(Deserialize, Serialize, PartialEq, Debug, Clone)]
#[serde(untagged)]
pub enum  EtObject{
    Text(tags::TagText,String),
    EtSnote(tags::TagEtSnote,Vec<EtSnoteObject>),
}
impl std::fmt::Display for EtObject{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self{
            EtObject::Text(_,text)=>write!(f,"{}",token_decode::change_string(&text)),
            EtObject::EtSnote(_,etsnote_object)=>write!(f,"{}",clamp_vector(&etsnote_object,"(div-EtObject::EtSnoteObject)"))
        }
    }
}

#[derive(Deserialize,Serialize,PartialEq,Debug,Clone)]
#[serde(untagged)]
pub enum EtSnoteObject{
    T(tags::TagT,String)
}

impl std::fmt::Display for EtSnoteObject{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self{
            EtSnoteObject::T(_,t )=>write!(f,"{}",t)
        }
    }
}