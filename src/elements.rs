#[derive(Hash, Eq, PartialEq, Clone, Debug)]
pub enum SetDataTypes {
    String(String),
    Char(char),
    Interger(i64),
}


#[derive(Hash, Eq, PartialEq, Clone, Debug)]
pub struct Elements {
    data: SetDataTypes,
}

impl Elements {
    pub fn new(data: SetDataTypes) -> Elements {
        Elements { data }
    }
}

impl From<i64> for Elements {
    fn from(value: i64) -> Self {
        let data = SetDataTypes::Interger(value);
        Elements::new(data)
    }
}

impl From<String> for Elements {
    fn from(value: String) -> Self {
        let data = SetDataTypes::String(value);
        Elements::new(data)
    }
}

impl From<char> for Elements {
    fn from(value: char) -> Self {
        let data = SetDataTypes::Char(value);
        Elements::new(data)
    }
}