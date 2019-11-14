pub enum Regions {
    Americas,
    Asia,
    Europe,
}

impl WithHosts for Regions {
    fn host(&self, hostname: &str) -> String {
        match *self {
            Regions::Americas => format!("americas.{}", hostname),
            Regions::Asia => format!("asia.{}", hostname),
            Regions::Europe => format!("europe.{}", hostname),
        }
    }
}

pub enum Platforms {
    Br1,
    Eun1,
    Euw1,
    Jp1,
    Kr,
    La1,
    La2,
    Na1,
    Oc1,
    Tr1,
    Ru,
}

impl WithHosts for Platforms {
    fn host(&self, hostname: &str) -> String {
        match *self {
            Platforms::Br1 => format!("br1.{}", hostname),
            Platforms::Eun1 => format!("eun1.{}", hostname),
            Platforms::Euw1 => format!("euw1.{}", hostname),
            Platforms::Jp1 => format!("jp1.{}", hostname),
            Platforms::Kr => format!("kr.{}", hostname),
            Platforms::La1 => format!("la1.{}", hostname),
            Platforms::La2 => format!("la2.{}", hostname),
            Platforms::Na1 => format!("na1.{}", hostname),
            Platforms::Oc1 => format!("oc1.{}", hostname),
            Platforms::Tr1 => format!("tr1.{}", hostname),
            Platforms::Ru => format!("ru.{}", hostname),
        }
    }
}

pub trait WithHosts {
    fn host(&self, hostname: &str) -> String;
}
