use std::fmt;
use crate::creature::Random;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Element {
    Earth,
    Water,
    Electric,
    Fire,
    Wind,
    Plant,
    Celestial,
    None,
}
impl Random for Element {
    type Type = Element;
    fn random_type(&self) -> Self::Type {
        let max = 7;
        let val = self.random_rate(max);
        match val {
            0 => return Element::Water ,
            1 => return Element::Electric ,
            2 => return Element::Fire ,
            3 => return Element::Wind ,
            4 => return Element::Plant ,
            5 => return Element::Celestial ,
            7 => return Element::Earth ,
            _=> Element::None
        }
    }
}
impl fmt::Display for Element {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let v:String;
        match *self {
            Element::Earth => v = String::from("Earth"),
            Element::Water => v = String::from("Water"),
            Element::Electric => v = String::from("Electric"),
            Element::Fire => v = String::from("Fire"),
            Element::Wind => v = String::from("Wind"),
            Element::Plant => v = String::from("Plant"),
            Element::Celestial => v = String::from("Celestial"),
            Element::None => v = String::from(""),
        }
        write!(f, "{}", v.as_str())
    }
}
impl Element {
    pub fn double(value:f64) -> f64 {
        value * 2.0
    }
    pub fn half(value:f64) -> f64 {
        value / 2.0
    }
    pub fn half_extra(value:f64) -> f64 {
        value + (value / 2.0)
    }
    /// Effectiveness against Earth
    pub fn earth(&self, value:f64) -> f64 {
        match *self {
            Element::Earth => return value,
            Element::Water => return Element::double(value),
            Element::Electric => return Element::double(value),
            Element::Fire => return Element::double(value),
            Element::Wind => return value,
            Element::Plant => return  Element::half_extra(value),
            Element::Celestial => return 0.0,
            Element::None => return value,
        }
    }
    /// Effectiveness against Water
    pub fn water(&self, value:f64) -> f64 {
        match *self {
            Element::Earth => return  Element::half(value),
            Element::Water => return value,
            Element::Electric => return  Element::half_extra(value),
            Element::Fire => return Element::double(value),
            Element::Wind => return  Element::half(value),
            Element::Plant => return  Element::half(value),
            Element::Celestial => return value,
            Element::None => return value,
        }
    }
    /// Effectiveness against Electric
    pub fn electric(&self, value:f64) -> f64 {
        match *self {
            Element::Earth => return  Element::half(value),
            Element::Water => return Element::double(value),
            Element::Electric => return value,
            Element::Fire => return 0.0,
            Element::Wind => return  Element::half(value),
            Element::Plant => return value,
            Element::Celestial => return value,
            Element::None => return value,
        }
    }
    /// Effectiveness against Fire
    pub fn fire(&self, value:f64) -> f64 {
        match *self {
            Element::Earth => return 0.0,
            Element::Water => return  Element::half(value),
            Element::Electric => return value,
            Element::Fire => return value,
            Element::Wind => return Element::double(value),
            Element::Plant => return Element::double(value),
            Element::Celestial => return value,
            Element::None => return value,
        }
    }
    //TODO
    /// Effectiveness against Wind
    pub fn wind(&self, value:f64) -> f64 {
        match *self {
            Element::Earth => return value,
            Element::Water => return value,
            Element::Electric => return value,
            Element::Fire => return value,
            Element::Wind => return value,
            Element::Plant => return value,
            Element::Celestial => return value,
            Element::None => return value,
        }
    }
    /// Effectiveness against Plant
    pub fn tree(&self, value:f64) -> f64 {
        match *self {
            Element::Earth => return value,
            Element::Water => return value,
            Element::Electric => return value,
            Element::Fire => return value,
            Element::Wind => return value,
            Element::Plant => return value,
            Element::Celestial => return value,
            Element::None => return value,
        }
    }
    /// Effectiveness against Celestial
    pub fn celestial(&self, value:f64) -> f64 {
        match *self {
            Element::Earth => return value,
            Element::Water => return value,
            Element::Electric => return value,
            Element::Fire => return value,
            Element::Wind => return value,
            Element::Plant => return value,
            Element::Celestial => return value,
            Element::None => return value,
        }
    }
    /// Effectiveness against None
    pub fn none(&self, value:f64) -> f64 {
        match *self {
            Element::Earth => return  Element::half(value),
            Element::Water => return  Element::half(value),
            Element::Electric => return  Element::half(value),
            Element::Fire => return  Element::half(value),
            Element::Wind => return  Element::half(value),
            Element::Plant => return  Element::half(value),
            Element::Celestial => return 0.0,
            Element::None => return value,
        }
    }
    pub fn effect(&self, other:Element, value:f64) -> f64 {
        match other {
            Element::Earth => return self.earth(value),
            Element::Water => return self.water(value),
            Element::Electric => return self.electric(value),
            Element::Fire => return self.fire(value),
            Element::Wind => return self.wind(value),
            Element::Plant => return self.tree(value),
            Element::Celestial => return self.celestial(value),
            Element::None => return self.none(value),
        }
    }
}
