pub enum Regions {
    Americas,
    Asia,
    Europe,
}

impl WithHosts for Regions {
    fn host(&self) -> String {
        match *self {
            Regions::Americas => format!("americas.{}", dotenv!("RIOT_API_HOST")),
            Regions::Asia => format!("asia.{}", dotenv!("RIOT_API_HOST")),
            Regions::Europe => format!("europe.{}", dotenv!("RIOT_API_HOST")),
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
    fn host(&self) -> String {
        match *self {
            Platforms::Br1 => format!("br1.{}", dotenv!("RIOT_API_HOST")),
            Platforms::Eun1 => format!("eun1.{}", dotenv!("RIOT_API_HOST")),
            Platforms::Euw1 => format!("euw1.{}", dotenv!("RIOT_API_HOST")),
            Platforms::Jp1 => format!("jp1.{}", dotenv!("RIOT_API_HOST")),
            Platforms::Kr => format!("kr.{}", dotenv!("RIOT_API_HOST")),
            Platforms::La1 => format!("la1.{}", dotenv!("RIOT_API_HOST")),
            Platforms::La2 => format!("la2.{}", dotenv!("RIOT_API_HOST")),
            Platforms::Na1 => format!("na1.{}", dotenv!("RIOT_API_HOST")),
            Platforms::Oc1 => format!("oc1.{}", dotenv!("RIOT_API_HOST")),
            Platforms::Tr1 => format!("tr1.{}", dotenv!("RIOT_API_HOST")),
            Platforms::Ru => format!("ru.{}", dotenv!("RIOT_API_HOST")),
        }
    }
}

pub trait WithHosts {
    fn host(&self) -> String;
}
