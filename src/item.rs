use crate::item_images::*;
use crate::condition::*;
use crate::element::*;
use crate::creature::*;
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Item {
    Health,
    ExtraHealth,
    FullHealth,
    Mana,
    ExtraMana,
    FullMana,
    Crystal(Element),
    Powder(Condition),
    Heal(Condition),
    None,
}
impl fmt::Display for Item {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let v:String;
        let mut other = String::from("");
        match *self {
            Item::Health => v = String::from("Health"),
            Item::ExtraHealth => v = String::from("Extra Health"),
            Item::FullHealth => v = String::from("Full Health"),
            Item::Mana => v = String::from("Mana"),
            Item::ExtraMana => v = String::from("Extra Mana"),
            Item::FullMana => v = String::from("Full Mana"),
            Item::Crystal(e) => {
                v = String::from(" Crystal");
                other = e.to_string();
            },
            Item::Powder(c) => {
                v = String::from("  Powder");
                other = c.to_string();
            },
            Item::Heal(c) => {
                v = String::from(" Heal");
                other = c.to_string();
            },
            Item::None => v = String::from(""),
        }
        write!(f, "{}{}", other.as_str(), v.as_str())
    }
}
impl Item {
    pub fn from(string:String) -> Self {
        if string == "Health".to_string() {
            return Item::Health
        } else if string == "Mana".to_string() {
            return Item::Mana
        } else {
            return Item::None
        }
    }
    pub fn apply(&self, mut character:Creature) -> Creature {
        match *self {
            Item::Health => {
                character.hp += 50.0;
            },
            Item::ExtraHealth => {
                character.hp += 200.0;
            },
            Item::FullHealth => {
                character.hp += 500.0;
            },
            Item::Mana => {
                let mut total = 20.0;
                for mut special in character.moves.clone() {
                    total = special.restore_mp(total);
                }
            },
            Item::ExtraMana =>  {
                let mut total = 50.0;
                for mut special in character.moves.clone() {
                    total = special.restore_mp(total);
                }
            },
            Item::FullMana =>  {
                let mut total = 100.0;
                for mut special in character.moves.clone() {
                    total = special.restore_mp(total);
                }
            },
            Item::Crystal(e) => {
                // TODO 10.0 what stat could we use?
                let val = character.element1.effect(e, 10.0);
                match e {
                    Element::Earth => character.def_effort += val,
                    Element::Water => character.hp_effort += val,
                    Element::Electric => character.atk_effort += val,
                    Element::Fire => character.atk_effort += val,
                    Element::Wind => character.speed_effort += val,
                    Element::Plant => character.atk_effort += val,
                    Element::Celestial => character.special_effort += val,
                    Element::None => character.xp += val,
                }
            },
            Item::Powder(c) => {
                character.condition = c;
            },
            Item::Heal(c) => {
                if character.condition == c {
                    character.condition = Condition::None;
                }
            },
            Item::None => return character,
        }
        character.level_up();
        character
    }
    /*pub fn pixmap(&self) -> &[&str]{
        
    }*/
}
