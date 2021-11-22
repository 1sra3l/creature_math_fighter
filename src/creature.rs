extern crate ez_pixmap;
use crate::item_images::*;
use std::fmt;

use crate::item::*;
use crate::element::*;
use crate::condition::*;
use crate::moves::*;


use rand::{thread_rng, Rng};
pub trait Random {
    type Type;
    fn random_type(&self) -> Self::Type;
    fn random(&self, min:f64, max:f64) -> f64 {
        let mut rng = thread_rng();
        let n: f64 = rng.gen_range(min..max);
        n
    }
    fn random_image(&self)->String {
        let max = 7;
        let val = self.random_rate(max);
        match val {
            0 => return String::from("assets/possum"),
            1 => return String::from("assets/snester"),
            2 => return String::from("assets/porcupine"),
            3 => return String::from("assets/butterfly"),
            4 => return String::from("assets/possum"),
            5 => return String::from("assets/chipmonk"),
            _=> String::from("assets/acuteasuar"),
        }
    }
    fn random_name(&self) ->String {
        let max = 7;
        let val = self.random_rate(max);
        let mut name:String = match val {
            0 => String::from("Ax"),
            1 => String::from("Morph"),
            2 => String::from("Dri"),
            3 => String::from("Cor"),
            4 => String::from("Mig"),
            5 => String::from("Den"),
            7 => String::from("Por"),
            _=> String::from("Rust"),
        };
        let val = self.random_rate(max);
        let name_second:String = match val {
            0 => String::from("atlin"),
            1 => String::from("eleous"),
            2 => String::from("manthus"),
            3 => String::from("timbrle"),
            4 => String::from("darlis"),
            5 => String::from("gerar"),
            7 => String::from("truder"),
            _=> String::from("ferris"),
        };
        name.push_str(name_second.as_str());
        name
    
    }
    fn random_rate(&self,value:u32) -> u32 {
        let mut rng = thread_rng();
        let n: u32 = rng.gen_range(0..value);
        n
    }
    /// 
    fn half(&self) -> bool {
        if self.random_rate(1) == 1 {
            return true
        }
        false
    }
    /// 
    fn usually(&self) -> bool {
        if self.random_rate(9) > 1 {
            return true
        }
        false
    }
    /// 
    fn often(&self) -> bool {
        if self.random_rate(3) > 0 {
            return true
        }
        false
    }
    /// 
    fn hardly(&self) -> bool {
        if self.random_rate(3) == 0 {
            return true
        }
        false
    }
    /// 
    fn barely(&self) -> bool {
        if self.random_rate(9) == 0 {
            return true
        }
        false
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Form {
    Basic,
    Advanced,
    Extra,
    Mega,
    Ultra,
    Mythic,
    None,
}
impl Random for Form {
    type Type = Form;
    fn random_type(&self) -> Self::Type {
        if self.half() {
            return Form::Extra
        } else if self.barely() {
            return Form::Extra
        } else if self.hardly() {
            return Form::Mega
        }
        Form::Basic
    }
    
}
impl fmt::Display for Form {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let v:String;
        match *self {
            Form::Basic => v = String::from("Basic"),
            Form::Advanced => v = String::from("Advanced"),
            Form::Extra => v = String::from("Extra"),
            Form::Mega => v = String::from("Mega"),
            Form::Ultra => v = String::from("Ultra"),
            Form::Mythic => v = String::from("Mythic"),
            Form::None => v = String::from(""),
        }
        write!(f, "{}", v.as_str())
    }
}
impl Form {
    pub fn check_level(level:f64, cap:f64) -> Form {
        if level > cap {
            return Form::Mythic
        } else {
            if level > cap / 50.0 {
                return Form::Advanced
            }
            if level > cap / 20.0 {
                return Form::Extra
            }
            if level > cap / 15.0 {
                return Form::Mega
            }
            if level > cap / 2.0 {
                return Form::Ultra
            }
            return Form::Basic
        }
    }
}

#[derive(Debug, Clone)]
pub struct Creature {
    pub id:u32,
    pub name:String,
    pub form:Form,
    pub condition:Condition,
    pub element1:Element,
    pub element2:Element,
    pub rate:f64,
    pub items:Vec<Item>,
    pub owner:u32,
    pub xp:f64,
    pub total_xp:f64,
    pub hp_effort:f64,
    pub atk_effort:f64,
    pub def_effort:f64,
    pub speed_effort:f64,
    pub special_effort:f64,
    pub level:f64,
    pub hp_max:f64,
    pub hp:f64,
    pub atk:f64,
    pub def:f64,
    pub speed:f64,
    pub special:f64,
    pub image:String,
    pub moves:Vec<Special>,
}
impl Random for Creature {
    type Type = Creature;
    fn random_type(&self) -> Self::Type {
        let mut elem = Element::Earth;
        elem = elem.random_type();
        let hp = self.random(10.0,50.0);
        let atk = self.random(5.0,50.0);
        let def = self.random(5.0,50.0);
        let speed = self.random(5.0,50.0);
        let special = self.random(5.0,50.0);
        let form = Form::Basic;
        Creature {
            id:self.random_rate(100),
            name:self.random_name(),
            form:form.random_type(),
            condition:Condition::None,
            element1:elem,
            element2:elem,
            rate:self.random(5.0,90.0),
            items:vec![],
            moves:vec![],
            owner:0,
            xp:0.0,
            total_xp:0.0,
            hp_effort:hp,
            atk_effort:atk,
            def_effort:def,
            speed_effort:speed,
            special_effort:special,
            level:0.0,
            hp_max:hp,
            hp:hp,
            atk:atk,
            def:def,
            speed:speed,
            special:special,
            image:self.random_image().to_owned(),
            
        }
    }
    
}
impl Creature {
    pub fn damage_attack(&mut self, atk_move:Special, other:Creature) -> f64 {
        //first math
        let dmg = self.get_damage(atk_move.clone(), other.element1);
        dmg * self.atk
    }
    pub fn get_damage(&mut self, atk_move:Special, other:Element) -> f64 {
        atk_move.damage(other)
    }
    pub fn special(&mut self, id:usize, other:Creature) -> Option<f64> {
        let vec = self.moves.clone();
        if vec.len() < id {
            return None
        }
        let atk_move = vec[id].clone();
        let atk = self.atk;
        let mut result = self.damage_attack(atk_move.clone(), other.clone());//dmg * self.atk
        let def = other.def + other.hp;
        if result == 0.0 {
            result = def;
        }
        result /= def;
        if result > other.hp {
            result = other.hp;
        }
        Some(result)
    }
    pub fn use_item(&mut self, name:String) {
        let item = Item::from(name);
        let mut counter:usize = 0;
        for i in self.items.clone() {
            if item == i {
                match i {
                    Item::Health => {
                        self.hp += 50.0;
                    },
                    Item::ExtraHealth => {
                        self.hp += 200.0;
                    },
                    Item::FullHealth => {
                        self.hp += 500.0;
                    },
                    Item::Mana => {
                        let mut total = 20.0;
                        let m_clone = self.moves.clone();
                        self.moves.clear();
                        for mut special in m_clone {
                            let t = special.mp_total();
                            let diff = t - special.mp;
                            special.mp = t;
                            total -= diff;
                            self.moves.push(special);
                        }
                        
                    },
                    Item::ExtraMana =>  {
                        let mut total = 50.0;
                        let m_clone = self.moves.clone();
                        self.moves.clear();
                        for mut special in m_clone {
                            let t = special.mp_total();
                            let diff = t - special.mp;
                            special.mp = t;
                            total -= diff;
                            self.moves.push(special);
                        }
                    },
                    Item::FullMana =>  {
                        let mut total = 100.0;
                        let m_clone = self.moves.clone();
                        self.moves.clear();
                        for mut special in m_clone {
                            let t = special.mp_total();
                            let diff = t - special.mp;
                            special.mp = t;
                            total -= diff;
                            self.moves.push(special);
                        }
                    },
                    Item::Crystal(e) => {
                        // TODO 10.0 what stat could we use?
                        let val = self.element1.effect(e, 10.0);
                        match e {
                            Element::Earth => self.def_effort += val,
                            Element::Water => self.hp_effort += val,
                            Element::Electric => self.atk_effort += val,
                            Element::Fire => self.atk_effort += val,
                            Element::Wind => self.speed_effort += val,
                            Element::Plant => self.atk_effort += val,
                            Element::Celestial => self.special_effort += val,
                            Element::None => self.xp += val,
                        }
                    },
                    Item::Powder(c) => {
                        self.condition = c;
                    },
                    Item::Heal(c) => {
                        if self.condition == c {
                            self.condition = Condition::None;
                        }
                    },
                    Item::None => return,
                }
                self.level_up();
                self.items.remove(counter);
                return
            }
            counter += 1;
        }
    }
    pub fn monitor_condition(&mut self, counter:f64) {
        if counter < self.speed {
            return
        }
        match self.condition {
            Condition::Poison => {
                self.hp -= 4.0;
            },
            Condition::Burn => {
                self.hp -= 5.0;
            },
            _=> (),
        }
    }
    pub fn next(&self) -> f64 {
        self.level * 20.0
    }
    pub fn get_item(&mut self, item:Item) {
        if item != Item::None {
            self.items.push(item);
        }
    }
    pub fn level_up(&mut self) {
        println!("xp:{} next:{} total:{}", self.xp, self.next(), self.total_xp);
        self.total_xp += self.xp;
        if self.xp > self.next() {
            if self.hp_effort > self.hp_max {
               self.hp_max = self.hp_effort; 
            } else {
                self.hp_effort += self.level;
            }
            if self.atk_effort > self.atk {
                self.atk = self.atk_effort; 
            } else {
                self.atk_effort += self.level;
            }
            if self.def_effort > self.def {
                self.def = self.def_effort; 
            } else {
                self.def_effort += self.level;
            }
            if self.speed_effort > self.speed {
                self.speed = self.speed_effort; 
            } else {
                self.speed_effort += self.level;
            }
            if self.special_effort > self.special {
                self.special = self.special_effort; 
            } else {
                self.special_effort += self.level;
            }
            self.level += 1.0;
            let spec = Special::upgrade_new(self.level, self.element1);
            if spec.id != Move::None {
                self.moves.push(spec);
            }
            self.xp = 0.0;
        }
        println!("level:{}",self.level);
    }
    pub fn new() -> Self {
        Creature {
            id:0,
            name:String::from(""),
            form:Form::None,
            condition:Condition::None,
            element1:Element::None,
            element2:Element::None,
            rate:0.0,
            items:vec![],
            moves:vec![],
            owner:0,
            xp:0.0,
            total_xp:0.0,
            hp_effort:0.0,
            atk_effort:0.0,
            def_effort:0.0,
            speed_effort:0.0,
            special_effort:0.0,
            level:0.0,
            hp_max:0.0,
            hp:0.0,
            atk:0.0,
            def:0.0,
            speed:0.0,
            special:0.0,
            image:String::from(""),
        }
    }
    pub fn from_element(element:Element) -> Creature{
        let id:u32;
        let name:String;
        let form:Form;
        let element1:Element;
        let element2:Element;
        let moves:Vec<Special>;
        let rate:f64;
        let hp:f64;
        let atk:f64;
        let def:f64;
        let speed:f64;
        let special:f64;
        let mut image:String = String::from("assets/");
        let item:Item;
        
        match element {
            Element::Earth => {
                id = 0;
                name = String::from("Porcupiner");
                form = Form::Basic;
                moves = vec![Special::new(Move::Pummel(element)),  Special::new(Move::Crush(Condition::Paralisys)) ];
                rate = 50.0;
                hp = 50.0;
                atk = 10.0;
                def = 10.0;
                speed = 1.0;
                special = 1.0;
                image.push_str("porcupine");
                item = Item::Mana;
            },
            Element::Water => {
                id = 1;
                name = String::from("Bubbles");
                form = Form::Basic;
                moves = vec![Special::new(Move::Pummel(element)),  Special::new(Move::Crush(Condition::Drown))];
                rate = 50.0;
                hp = 55.0;
                atk = 5.0;
                def = 10.0;
                speed = 5.0;
                special = 10.0;
                image.push_str("possum");
                item = Item::Mana;
            },
            Element::Electric => {
                id = 2;
                name = String::from("Chipmonk");
                form = Form::Basic;
                moves = vec![Special::new(Move::Plasma(element)), Special::new(Move::Bite(element))];
                rate = 50.0;
                hp = 50.0;
                atk = 10.0;
                def = 5.0;
                speed = 10.0;
                special = 15.0;
                image.push_str("chipmonk");
                item = Item::Crystal(element);
            },
            Element::Fire => {
                id = 3;
                name = String::from("Satoshi");
                form = Form::Basic;
                moves = vec![Special::new(Move::Bite(element)), Special::new(Move::Sting(Condition::Burn)) ];
                rate = 50.0;
                hp = 50.0;
                atk = 10.0;
                def = 5.0;
                speed = 15.0;
                special = 10.0;
                image.push_str("butterfly");
                item = Item::Mana;
            },
            Element::Wind => {
                id = 4;
                name = String::from("Super Butterfly");
                form = Form::Basic;
                moves = vec![Special::new(Move::Bite(element)), Special::new(Move::Sting(Condition::Poison))];
                rate = 50.0;
                hp = 50.0;
                atk = 10.0;
                def = 5.0;
                speed = 15.0;
                special = 15.0;
                image.push_str("butterfly");
                item = Item::Mana;
            },
            Element::Plant => {
                id = 5;
                name = String::from("Snester");
                form = Form::Basic;
                moves = vec![Special::new(Move::Tail(element)), Special::new(Move::Strike(Condition::Poison))];
                rate = 50.0;
                hp = 55.0;
                atk = 10.0;
                def = 4.0;
                speed = 4.0;
                special = 10.0;
                image.push_str("snester");
                item = Item::Heal(Condition::Poison);
            },
            Element::Celestial => {
                id = 7;
                name = String::from("Thunderbird");
                form = Form::Mythic;
                moves = vec![ Special::new(Move::Talon(Condition::Sleep)), Special::new(Move::Strike(Condition::Paralisys))];
                rate = 1.0;
                hp = 50.0;
                atk = 10.0;
                def = 10.0;
                speed = 10.0;
                special = 10.0;
                image.push_str("butterfly");
                item = Item::Crystal(element);
            },
            Element::None => {
                id = 6;
                name = String::from("MISSINGNO:");
                form = Form::None;
                moves = vec![Special::new(Move::Claw(element))];
                rate = 90.0;
                hp = 45.0;
                atk = 5.0;
                def = 5.0;
                speed =5.0;
                special = 1.0;
                image.push_str("butterfly");
                item = Item::None;
            },
        }
        Creature {
            id:id,
            name:name,
            form:form,
            condition:Condition::None,
            element1:element,
            element2:element,
            rate:rate,
            items:vec![item],
            moves:moves,
            owner:0,
            xp:0.0,
            total_xp:0.0,
            hp_effort:hp,
            atk_effort:atk,
            def_effort:def,
            speed_effort:speed,
            special_effort:special,
            level:1.0,
            hp_max:hp,
            hp:hp,
            atk:atk,
            def:def,
            speed:speed,
            special:special,
            image:image,
        }
    }
}
