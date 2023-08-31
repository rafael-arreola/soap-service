use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Order {
    #[serde(rename(serialize = "MD_CODIGO"))]
    #[serde(rename(deserialize = "MD_CODIGO"))]
    pub codigo: String,

    #[serde(rename(serialize = "CMARCA"))]
    #[serde(rename(deserialize = "CMARCA"))]
    pub marca: String,

    #[serde(rename(serialize = "FECHAINI"))]
    #[serde(rename(deserialize = "FECHAINI"))]
    pub fecha_ini: String,

    #[serde(rename(serialize = "FECHAFIN"))]
    #[serde(rename(deserialize = "FECHAFIN"))]
    pub fecha_fin: String,
}