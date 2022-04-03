// Example code that deserializes and serializes the model.
// extern crate serde;
// #[macro_use]
// extern crate serde_derive;
// extern crate serde_json;
//
// use generated_module::[object Object];
//
// fn main() {
//     let json = r#"{"answer": 42}"#;
//     let model: [object Object] = serde_json::from_str(&json).unwrap();
// }

pub mod sensor_temp {

use serde_derive::{self, Deserialize, Serialize};
   


#[derive(Debug, Serialize, Deserialize)]
pub struct SensorTemp {
    #[serde(rename = "coretemp-isa-0000")]
    pub coretemp_isa_0000: CoretempIsa0000,

    #[serde(rename = "acpitz-acpi-0")]
    pub acpitz_acpi_0: AcpitzAcpi0,

    #[serde(rename = "pch_skylake-virtual-0")]
    pub pch_skylake_virtual_0: PchSkylakeVirtual0,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AcpitzAcpi0 {
    #[serde(rename = "Adapter")]
    pub adapter: String,

    #[serde(rename = "temp1")]
    pub temp1: AcpitzAcpi0_Temp1,

    #[serde(rename = "temp2")]
    pub temp2: Temp2,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AcpitzAcpi0_Temp1 {
    #[serde(rename = "temp1_input")]
    pub temp1_input: f64,

    #[serde(rename = "temp1_crit")]
    pub temp1_crit: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Temp2 {
    #[serde(rename = "temp2_input")]
    pub temp2_input: f64,

    #[serde(rename = "temp2_crit")]
    pub temp2_crit: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoretempIsa0000 {
    #[serde(rename = "Adapter")]
    pub adapter: String,

    #[serde(rename = "Package id 0")]
    pub package_id_0: PackageId0,

    #[serde(rename = "Core 0")]
    pub core_0: Core0,

    #[serde(rename = "Core 1")]
    pub core_1: Core1,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Core0 {
    #[serde(rename = "temp2_input")]
    pub temp2_input: f64,

    #[serde(rename = "temp2_max")]
    pub temp2_max: f64,

    #[serde(rename = "temp2_crit")]
    pub temp2_crit: f64,

    #[serde(rename = "temp2_crit_alarm")]
    pub temp2_crit_alarm: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Core1 {
    #[serde(rename = "temp3_input")]
    pub temp3_input: f64,

    #[serde(rename = "temp3_max")]
    pub temp3_max: f64,

    #[serde(rename = "temp3_crit")]
    pub temp3_crit: f64,

    #[serde(rename = "temp3_crit_alarm")]
    pub temp3_crit_alarm: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PackageId0 {
    #[serde(rename = "temp1_input")]
    pub temp1_input: f64,

    #[serde(rename = "temp1_max")]
    pub temp1_max: f64,

    #[serde(rename = "temp1_crit")]
    pub temp1_crit: f64,

    #[serde(rename = "temp1_crit_alarm")]
    pub temp1_crit_alarm: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PchSkylakeVirtual0 {
    #[serde(rename = "Adapter")]
    pub adapter: String,

    #[serde(rename = "temp1")]
    pub temp1: PchSkylakeVirtual0_Temp1,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PchSkylakeVirtual0_Temp1 {
    #[serde(rename = "temp1_input")]
    pub temp1_input: f64,
}
}