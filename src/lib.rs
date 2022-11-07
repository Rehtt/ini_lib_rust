use std::collections::HashMap;
use std::str::FromStr;

#[derive(Debug, Clone)]
pub struct INI(Vec<Section>);

#[derive(Debug, Clone)]
pub struct Section {
    name: String,
    map: HashMap<String, Option<String>>,
}

impl Default for Section {
    fn default() -> Self {
        Self {
            name: String::new(),
            map: HashMap::new(),
        }
    }
}

impl Section {
    fn is_empty(&self) -> bool {
        self.name.is_empty()
    }
    fn clear(&mut self) {
        self.name = String::new();
        self.map.clear();
    }
}


impl FromStr for INI {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let data = s
            .trim()
            .split("\n")
            .filter(|x| {
                let xx = x.trim();
                !(xx.starts_with(";") || xx.starts_with("#"))
            })
            .map(|x| {
                x.trim()
                    .split("=")
                    .map(|x| x.trim())
                    .filter(|x| {
                        !x.to_string().eq("")
                    })
                    .collect::<Vec<_>>()
            })
            .filter(|x| !x.is_empty())
            .collect::<Vec<_>>();

        let mut tmp_section = Section::default();
        let mut tmp_map: HashMap<String, Option<String>> = HashMap::new();
        let mut amaps = Vec::new();
        for x in data {
            if x[0].starts_with("[") && x[0].ends_with("]") {
                if !tmp_section.is_empty() {
                    if !tmp_map.is_empty() {
                        tmp_section.map = tmp_map.clone();
                        tmp_map.clear();
                    }
                    amaps.push(tmp_section.clone());
                    tmp_section.clear();
                }
                tmp_section.name = x[0].trim_start_matches('[').trim_end_matches(']').to_string();
                continue;
            }
            if x.len() < 2 {
                tmp_map.insert(x[0].to_string(), None);
                continue;
            }
            tmp_map.insert(x[0].to_string(), Some(x[1].to_string()));
        }
        if !tmp_section.is_empty() {
            if !tmp_map.is_empty() {
                tmp_section.map = tmp_map.clone();
            }
            amaps.push(tmp_section.clone());
        }
        Ok(Self(amaps))
    }
}


pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let a = "[Interface]
Address = 10.13.13.2/24
PrivateKey = OA2x4YFBii8pgEPvm9Nb7IsBamyfNlTg1lA5m5wyrUo=
ListenPort = 51820
DNS = 8.8.8.8

[Peer]
PublicKey = /A/8ru1OOVcrDMljZcHgxWYH5groyynHxcAdpRca21s=
Endpoint = 116.31.232.209:51820
AllowedIPs = 10.13.13.5/32

;[Peer]
 ;PublicKey = SoznFdDKSTgvAIeCMpYHH2y4xvaqJObS3l4AY3XVRzY=
   #PresharedKey = kguCX9oPV/ACCuaeVOX5OJ9YeLEywsn2oGkCTYN7Fco=
Endpoint = 81.71.149.31:51820
AllowedIPs = 10.13.13.0/24,192.168.31.1/32
PersistentKeepalive = 25";
        println!("{:#?}", a.parse::<INI>());
    }
}
