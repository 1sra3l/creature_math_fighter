use crate::element::*;
use crate::condition::*;
use std::fmt;

#[derive(Debug, Clone)]
pub struct Special {
    pub id:Move,
    pub mp:f64,
}
impl fmt::Display for Special {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let v:String = self.id.to_string();
        let other = self.mp.to_string();
        let total = self.mp_total().to_string();
        write!(f, "{} ({}/{})", v.as_str(), other.as_str(), total.as_str())
    }
}
impl Special {
    pub fn new(special:Move) -> Self {
        Special {
            id:special,
            mp:special.mp_cost(),
        }
    }
    pub fn upgrade_new(level:f64, element:Element) -> Self {
        let special:Move;
        if level == 5.0 {
            println!("New move {} Fang", element);
            special = Move::Fang(element);
        } else if level == 10.0{
            println!("New move {} Slash", element);
            special = Move::Slash(element);        
        } else {
            special = Move::None;
        }
        Special {
            id:special,
            mp:special.mp_cost(),
        }
    }
    pub fn name(&self) -> String {
        self.id.to_string()
    }
    pub fn restore_mp(&mut self, value:f64) -> f64 {
        let total = self.mp_total();
        let diff = total - self.mp;
        println!("total={}",total);
        self.mp = total;
        value - diff
    }
    /// Tail Move Mana cost
    pub fn tail(my_element:Element, element:Element) -> f64 {
        return my_element.effect(element, 10.0)
    }
    /// Horn Move Mana cost
    pub fn horn(my_element:Element, element:Element) -> f64 {
        return my_element.effect(element, 12.0)
    }
    /// Fang Move Mana cost
    pub fn fang(my_element:Element, element:Element) -> f64 {
        return my_element.effect(element, 15.0)
    }
    /// Claw Move Mana cost
    pub fn claw(my_element:Element, element:Element) -> f64 {
        return my_element.effect(element, 11.0) 
    }
    /// Slash Move Mana cost
    pub fn slash(my_element:Element, element:Element) -> f64 {
        return my_element.effect(element, 20.0)
    }
    /// Smash Move Mana cost
    pub fn smash(my_element:Element, element:Element) -> f64 {
        return my_element.effect(element, 22.0)
    }
    /// Pummel Move Mana cost
    pub fn pummle(my_element:Element, element:Element) -> f64 {
        return my_element.effect(element, 30.0)
    }
    /// Plasma Move Mana cost
    pub fn plasma(my_element:Element, element:Element) -> f64 {
        return my_element.effect(element, 35.0)
    }
    /// Bite Move Mana cost
    pub fn bite(my_element:Element, element:Element) -> f64 {
        return my_element.effect(element, 18.0)
    }
    /// 
    pub fn condition(condition:Condition) -> f64 {
        match condition {
            Condition::Poison => 5.0,
            Condition::Burn => 4.0,
            Condition::Drown => 4.0,
            _=> 0.0,
        }
    }
    ///  Move Mana Total
    pub fn mp_total(&self) -> f64 {
        match self.id {
            Move::Tail(_element) => 30.0,
            Move::Horn(_element) => 20.0,
            Move::Fang(_element) => 15.0,
            Move::Claw(_element) => 20.0,
            Move::Slash(_element) => 25.0,
            Move::Smash(_element) => 20.0,
            Move::Pummel(_element) => 15.0,
            Move::Plasma(_element) => 10.0,
            Move::Bite(_element) => 20.0,
            Move::Sting(_condition) => 15.0,
            Move::Talon(_condition) => 20.0,
            Move::Strike(_condition) => 10.0,
            Move::Grab(_condition) => 25.0,
            Move::Crush(_condition) => 30.0,
            Move::Slice(_condition) => 20.0,
            Move::Chomp(_condition) => 20.0,
            Move::None => return 0.0,
        }
    }
    ///  Move damage calculator based on element
    pub fn damage(&self, other_element:Element) -> f64 {
        match self.id {
            Move::Tail(element) => Special::tail(element, other_element),
            Move::Horn(element) => Special::horn(element, other_element),
            Move::Fang(element) => Special::fang(element, other_element),
            Move::Claw(element) => Special::claw(element, other_element),
            Move::Slash(element) => Special::slash(element, other_element),
            Move::Smash(element) => Special::smash(element, other_element),
            Move::Pummel(element) => Special::pummle(element, other_element),
            Move::Plasma(element) => Special::plasma(element, other_element),
            Move::Bite(element) => Special::bite(element, other_element),
            Move::Sting(condition) => Special::condition(condition) + 15.0,
            Move::Talon(condition) => Special::condition(condition) + 20.0,
            Move::Strike(condition) => Special::condition(condition) + 10.0,
            Move::Grab(condition) => Special::condition(condition) + 5.0,
            Move::Crush(condition) => Special::condition(condition) + 30.0,
            Move::Slice(condition) => Special::condition(condition) + 20.0,
            Move::Chomp(condition) => Special::condition(condition) + 20.0,
            Move::None => return 0.0,
        }
    }
}
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Move {
    Tail(Element),
    Horn(Element),
    Fang(Element),
    Claw(Element),
    Slash(Element),
    Smash(Element),
    Pummel(Element),
    Plasma(Element),
    Bite(Element),
    Sting(Condition),
    Talon(Condition),
    Strike(Condition),
    Grab(Condition),
    Crush(Condition),
    Slice(Condition),
    Chomp(Condition),
    None,
}
impl fmt::Display for Move {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let v:String;
        let other:String;
        match *self {
            Move::Tail(e) => {
                v = String::from(" Tail");
                other = e.to_string();
            },
            Move::Horn(e) => {
                v = String::from(" Horn");
                other = e.to_string();
            },
            Move::Fang(e) => {
                v = String::from(" Fang");
                other = e.to_string();
            },
            Move::Claw(e) => {
                v = String::from(" Claw");
                other = e.to_string();
            },
            Move::Slash(e) => {
                v = String::from(" Slash");
                other = e.to_string();
            },
            Move::Smash(e) => {
                v = String::from(" Smash");
                other = e.to_string();
            },
            Move::Pummel(e) => {
                v = String::from(" Pummel");
                other = e.to_string();
            },
            Move::Plasma(e) => {
                v = String::from(" Plasma");
                other = e.to_string();
            },
            Move::Bite(e) => {
                v = String::from(" Bite");
                other = e.to_string();
            },
            Move::Sting(e) => {
                v = String::from(" Sting");
                other = e.to_string();
            },
            Move::Talon(e) => {
                v = String::from(" Talon");
                other = e.to_string();
            },
            Move::Strike(e) => {
                v = String::from(" Strike");
                other = e.to_string();
            },
            Move::Grab(e) => {
                v = String::from(" Grab");
                other = e.to_string();
            },
            Move::Crush(e) => {
                v = String::from(" Crush");
                other = e.to_string();
            },
            Move::Slice(e) => {
                v = String::from(" Slice");
                other = e.to_string();
            },
            Move::Chomp(e) => {
                v = String::from(" Chomp");
                other = e.to_string();
            },
            Move::None =>  {
                v = String::from("");
                other = String::from("");
            },
        }
        write!(f, "{}{}", other.as_str(), v.as_str())
    }
}
impl Move {
    ///  Move Mana Total
    pub fn mp_cost(&self) -> f64 {
        match *self {
            Move::Tail(_element) => 30.0,
            Move::Horn(_element) => 20.0,
            Move::Fang(_element) => 15.0,
            Move::Claw(_element) => 20.0,
            Move::Slash(_element) => 25.0,
            Move::Smash(_element) => 20.0,
            Move::Pummel(_element) => 15.0,
            Move::Plasma(_element) => 10.0,
            Move::Bite(_element) => 20.0,
            Move::Sting(_condition) => 15.0,
            Move::Talon(_condition) => 20.0,
            Move::Strike(_condition) => 10.0,
            Move::Grab(_condition) => 25.0,
            Move::Crush(_condition) => 30.0,
            Move::Slice(_condition) => 20.0,
            Move::Chomp(_condition) => 20.0,
            Move::None => return 0.0,
        }
    }
}
