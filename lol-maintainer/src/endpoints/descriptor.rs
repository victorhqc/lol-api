use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct ConstantDescriptor {
    pub name: String,
    pub enums: Vec<ConstantEnum>,
}

#[derive(Serialize, Deserialize)]
pub struct ConstantEnum {
    pub key: String,
    pub value: String
}

pub trait ToDescriptor {
    fn descriptor(&self) -> ConstantDescriptor;
}
