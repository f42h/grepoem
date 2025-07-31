struct Properties {
    delim: char,
    size_element: usize
}

impl Properties {
    fn new() -> Self {
        Self { 
            delim: '-', 
            size_element: 5 
        }
    }
}

#[derive(Debug, Clone)]
pub struct WinOemKey {
    pub value: String,
}

impl WinOemKey {
    pub fn validate(key: WinOemKey) -> bool {
        let prop = Properties::new();
        let tok: Vec<&str> = key.value.split(prop.delim).collect();
    
        for elem in tok {
            if elem.len() != prop.size_element || !elem.chars().all(|ch| ch.is_alphanumeric()) {
                return false;
            }
        }
    
        true
    }

    pub fn from(key: &str) -> Self {
        Self {
            value: String::from(key)
        }
    }
}