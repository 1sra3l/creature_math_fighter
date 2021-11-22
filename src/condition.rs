use std::fmt;
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Condition {
    Poison,
    Burn,
    Sleep,
    Paralisys,
    Drown,
    None,
}
impl fmt::Display for Condition {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let v:String;
        match *self {
            Condition::Poison => v = String::from("Poison"),
            Condition::Burn => v = String::from("Burn"),
            Condition::Sleep => v = String::from("Sleep"),
            Condition::Paralisys => v = String::from("Paralisys"),
            Condition::Drown => v = String::from("Drown"),
            Condition::None => v = String::from(""),
        }
        write!(f, "{}", v.as_str())
    }
}
