use serde::{Deserialize,Serialize};
#[derive(Deserialize, Serialize, PartialEq, Debug, Clone)]
#[serde(rename_all="lowercase")]
pub enum TagRiw{Riw}
#[derive(Deserialize, Serialize, PartialEq, Debug, Clone)]
#[serde(rename_all="lowercase")]
pub enum TagText{Text}
    
#[derive(Deserialize, Serialize, PartialEq, Debug, Clone)]
#[serde(rename_all="lowercase")]
pub enum TagT{T}

#[derive(Deserialize, Serialize, PartialEq, Debug, Clone)]
#[serde(rename_all="lowercase")]
pub enum TagVis{Vis}
#[derive(Deserialize, Serialize, PartialEq, Debug, Clone)]
#[serde(rename_all="lowercase")]
pub enum TagRi{Ri}

#[derive(Deserialize, Serialize, PartialEq, Debug, Clone)]
#[serde(rename_all="lowercase")]
pub enum TagUns{Uns}
#[derive(Deserialize, Serialize, PartialEq, Debug, Clone)]
#[serde(rename_all="lowercase")]
pub enum TagUarefs{Uarefs}

#[derive(Deserialize, Serialize, PartialEq, Debug, Clone)]
#[serde(rename_all="snake_case")]
pub enum TagEtSnote{EtSnote}

#[derive(Deserialize, Serialize, PartialEq, Debug, Clone)]
#[serde(rename_all="lowercase")]
pub enum TagSense{Sense}

#[derive(Deserialize, Serialize, PartialEq, Debug, Clone)]
#[serde(rename_all="lowercase")]
pub enum TagSen{Sen}

#[derive(Deserialize, Serialize, PartialEq, Debug, Clone)]
#[serde(rename_all="lowercase")]
pub enum TagBs{Bs}

#[derive(Deserialize, Serialize, PartialEq, Debug, Clone)]
#[serde(rename_all="lowercase")]
pub enum TagPseq{Pseq}

#[derive(Deserialize, Serialize, PartialEq, Debug, Clone)]
#[serde(rename_all="lowercase")]
pub enum TagBnw{Bnw}

#[derive(Deserialize, Serialize, PartialEq, Debug, Clone)]
#[serde(rename_all="lowercase")]
pub enum TagCa{Ca}

#[derive(Deserialize, Serialize, PartialEq, Debug, Clone)]
#[serde(rename_all="lowercase")]
pub enum TagSnote{Snote}
