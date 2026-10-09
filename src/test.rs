#[cfg(test)]
mod tests{
    use crate::*;
    #[test]
    fn riw_test(){
    let ri=serde_json::from_str::<other_structure::RiObject>(r#" ["riw",{"rie":"Great Abaco"}]"#).unwrap();
    assert_eq!(other_structure::RiObject::Riw(tags::TagRiw::Riw,other_structure::RiRiw{rie:"Great Abaco".to_string(),prs:None}),ri);
    let text=serde_json::from_str::<other_structure::RiObject>(r#"["text","a"]"#).unwrap();
        assert_eq!(other_structure::RiObject::Text(tags::TagText::Text, "a".to_string()),text);
   
    }
    #[test]
    fn snote_test(){
        let t=serde_json::from_str::<other_structure::SnoteObject>(r#"["t","a"]"#).unwrap();
            assert_eq!(other_structure::SnoteObject::T(tags::TagT::T,"a".to_string()),t);
        
    }
    #[test]
    fn uns_test(){
        let vis=serde_json::from_str::<other_structure::UnsObject>(r#"["vis",[{"t":"a","aq":{"auth":"b"}},{"t":"c"}]]"#);
        match vis{
            Err(e)=>{
                panic!("{:?}",e);
            }
       Ok(vis)=>{ assert_eq!(other_structure::UnsObject::Vis(tags::TagVis::Vis,
            vec![other_structure::VisT{t:"a".to_string(),
            aq:Some(other_structure::Aq{auth:Some("b".to_string()),source:None,aqdate:None,subsource:None})
        },
        other_structure::VisT{t:"c".to_string(),aq:None}
        ]),vis);
    }
    }
    }
    #[test]
    fn utxt_test(){
        let vis=serde_json::from_str::<other_structure::UrosUtxtObject>(r#"["vis",[{"t":"a"}]]"#);
        match vis{
            Err(e)=>{
                panic!("{:?}",e);
            },
                        Ok(vis)=>{
                            assert_eq!(other_structure::UrosUtxtObject::Vis(tags::TagVis::Vis,vec![other_structure::VisT{t:"a".to_string(),aq:None}]),vis);
                        }

        }
    }
    #[test]
    fn usages_pt_test(){
        let uarefs=serde_json::from_str::<other_structure::UsagesPtObject>(r#"["uarefs",{"uaref":"id"}]"#);
        let text=serde_json::from_str::<other_structure::UsagesPtObject>(r#"["text","a"]"#);
        
        match text{
            Err(e)=>{
                panic!("{:?}",e);
            }
            Ok(text)=>{
                assert_eq!(other_structure::UsagesPtObject::Text(tags::TagText::Text,"a".to_string()),text);
            }
        }
        match uarefs{
            Err(e)=>{
                panic!("{:?}",e);
            }
            ,
            Ok(uarefs)=>{
                assert_eq!(other_structure::UsagesPtObject::Uarefs(tags::TagUarefs::Uarefs,other_structure::UsagesUarefs { uaref: "id".to_string() }),uarefs);
            }
        }

    }

    #[test]
    fn syns_pt_test(){
        let text=serde_json::from_str::<other_structure::SynsPtObject>(r#"["text","a"]"#);
        match text{
            Err(e)=>panic!("{:?}",e),
            Ok(text)=>{
                assert_eq!(other_structure::SynsPtObject::Text(tags::TagText::Text,"a".to_string()),text);
            }
        }
    }
    #[test]
    fn et_test(){
        let text=serde_json::from_str::<other_structure::EtObject>(r#"["text","a"]"#);
        match text{
            Err(e)=>panic!("{:?}",e),
            Ok(text)=>{
                assert_eq!(other_structure::EtObject::Text(tags::TagText::Text,"a".to_string()),text);
            }
        }
    }
    #[test]
    fn et_snote_test(){
        let t=serde_json::from_str::<other_structure::EtSnoteObject>(r#"["t","a"]"#);
        match t{
            Err(e)=>panic!("{:?}",e),
            Ok(t)=>{
                assert_eq!(other_structure::EtSnoteObject::T(tags::TagT::T,"a".to_string()),t);
            }
        }
    }
    #[test]
    fn sseq_test(){
        let sen=serde_json::from_str::<sense_organization::SseqObject>(r#"["sen",{"sn":"4","et":[["text","a"]]}]"#);
        match sen{
            Err(e)=>panic!("{:?}",e),
            Ok(sen)=>{
                assert_eq!(sense_organization::SseqObject::Sen(tags::TagSen::Sen,sense_organization::Sen { et:Some(vec![other_structure::EtObject::Text(tags::TagText::Text,"a".to_string())]), ins: None, lbs: None, prs: None, sgram: None, sls: None, sn: Some("4".to_string()), vrs: None }),sen);
            }       
        }
    }
    #[test]
    fn dt_test(){
        let ca=serde_json::from_str::<sense_organization::DtObject>(r#"["ca",{"intro":"a","cats":[{"cat":"b"}]}]"#);
      match ca{
        Err(e)=>panic!("{:?}",e),
        Ok(ca)=>{
            assert_eq!(sense_organization::DtObject::Ca(tags::TagCa::Ca,other_structure::Ca{intro:Some("a".to_string()),cats:Some(vec![other_structure::CatsObject{cat:Some("b".to_string()),catref:None,pn:None,prs:None,psl:None}])}),ca);
        }
      }
    }
    #[test]
    fn pseq_test(){
        let bs=serde_json::from_str::<sense_organization::PseqObject>(r#"["bs",{"sense":{"sn":"2","dt":[["text","a"]]}}]"#);
        match bs{
            Err(e)=>panic!("{:?}",e),
            Ok(bs)=>{
                assert_eq!(sense_organization::PseqObject::Bs(tags::TagBs::Bs,sense_organization::Bs { sense: sense_organization::Sense { dt:vec![sense_organization::DtObject::Text(tags::TagText::Text,"a".to_string())], et: None, ins: None, lbs: None, prs: None, sdsense: None, sgram: None, sls: None, sn: Some("2".to_string()), vrs: None } }),bs);
            }
        }
    }
}