pub trait WithHosts {
    fn host(&self) -> &'static str;
}

pub enum Regions {
    Americas,
    Asia,
    Europe,
}

impl WithHosts for Regions {
    fn host(&self) -> &'static str {
        match *self {
            Regions::Americas => concat!("americas.{}", env!("RIOT_API_HOST")),
            Regions::Asia => concat!("asia.{}", env!("RIOT_API_HOST")),
            Regions::Europe => concat!("europe.{}", env!("RIOT_API_HOST")),
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
    fn host(&self) -> &'static str {
        match *self {
            Platforms::Br1 => concat!("br1.{}", env!("RIOT_API_HOST")),
            Platforms::Eun1 => concat!("eun1.{}", env!("RIOT_API_HOST")),
            Platforms::Euw1 => concat!("euw1.{}", env!("RIOT_API_HOST")),
            Platforms::Jp1 => concat!("jp1.{}", env!("RIOT_API_HOST")),
            Platforms::Kr => concat!("kr.{}", env!("RIOT_API_HOST")),
            Platforms::La1 => concat!("la1.{}", env!("RIOT_API_HOST")),
            Platforms::La2 => concat!("la2.{}", env!("RIOT_API_HOST")),
            Platforms::Na1 => concat!("na1.{}", env!("RIOT_API_HOST")),
            Platforms::Oc1 => concat!("oc1.{}", env!("RIOT_API_HOST")),
            Platforms::Tr1 => concat!("tr1.{}", env!("RIOT_API_HOST")),
            Platforms::Ru => concat!("ru.{}", env!("RIOT_API_HOST")),
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
