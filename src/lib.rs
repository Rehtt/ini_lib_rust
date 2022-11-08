use std::collections::HashMap;
use std::fs::File;
use std::io::Read;

#[derive(Debug, Clone)]
pub struct INI;

#[derive(Debug, Clone)]
pub struct Section {
    pub name: String,
    pub sub: HashMap<String, Option<String>>,
}



/// ```
/// use ini_lib::ini_str;
/// let a = "[Interface]
/// Address = 10.1.1.2/24
/// PrivateKey = keykeykeykey
/// ListenPort = 51820
/// DNS = 8.8.8.8
///
/// [Peer]
/// PublicKey = keykeykeykeykeykeykeykey
/// Endpoint = 1.1.1.1:51820
/// AllowedIPs = 10.1.1.5/32
///
/// [Peer]
/// PublicKey = keykeykeykeykeykeykeykeykeykeykeykey
/// PresharedKey = keykeykeykeykeykeykeykeykeykeykeykeykeykeykeykey
/// Endpoint = 2.2.2.2:51820
/// AllowedIPs = 10.13.13.0/24
/// PersistentKeepalive = 25";
///
/// println!("{:#?}",ini_str!(a));
/// ```
#[macro_export]
macro_rules! ini_str {
    {$($data: expr),+} => {{
		($($crate::from_str($data)),+)
	}};
}

///```
/// use ini_lib::ini_file;
/// println!("{:#?}",ini_file!("test.ini"));
/// ```
#[macro_export]
macro_rules! ini_file {
    {$($data: expr),+} => {{
		($($crate::from_file($data)),+)
	}};
}

impl Default for Section {
    fn default() -> Self {
        Self {
            name: String::new(),
            sub: HashMap::new(),
        }
    }
}

impl Section {
    pub fn is_empty(&self) -> bool {
        self.name.is_empty()
    }
    pub fn clear(&mut self) {
        self.name = String::new();
        self.sub.clear();
    }
}


pub fn from_str(s: &str) -> Result<Vec<Section>, String> {
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
                    tmp_section.sub = tmp_map.clone();
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
            tmp_section.sub = tmp_map.clone();
        }
        amaps.push(tmp_section.clone());
    }
    Ok(amaps)
}

pub fn from_file(path: &str) -> Result<Vec<Section>, String> {
    return match File::open(path) {
        Ok(mut s) => {
            let mut buf = String::new();
            return match s.read_to_string(&mut buf) {
                Ok(_) => { from_str(buf.as_str()) }
                Err(err) => { Err(err.to_string()) }
            };
        }
        Err(err) => { Err(err.to_string()) }
    };
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let a = "[Interface]
Address = 10.1.1.2/24
PrivateKey = keykeykeykey
ListenPort = 51820
DNS = 8.8.8.8

[Peer]
PublicKey = keykeykeykeykeykeykeykey
Endpoint = 1.1.1.1:51820
AllowedIPs = 10.1.1.5/32

[Peer]
PublicKey = keykeykeykeykeykeykeykeykeykeykeykey
PresharedKey = keykeykeykeykeykeykeykeykeykeykeykeykeykeykeykey
Endpoint = 2.2.2.2:51820
AllowedIPs = 10.13.13.0/24
PersistentKeepalive = 25";

        println!("{:#?}", ini_str!(a));
    }
}
