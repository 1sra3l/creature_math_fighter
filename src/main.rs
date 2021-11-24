mod creature_fighter;
//mod creature;
//mod item;
mod item_images;
mod signals;
//mod condition;
//mod moves;
//mod element;

use std::fs;

// GUI
use fltk::{prelude::*, enums::*, image::*, *, input::Input, menu::MenuButton, menu::MenuItem, valuator::ValueInput, group::Group, window::Window};
extern crate ears;
use ears::{Music, AudioController};//Sound,

use rpgstat::creature::Stats as Creature;
use rpgstat::types::Normal as Element;
use rpgstat::random::*;
use rpgstat::special::ManaCost;


use fltk_form::{FltkForm, HasProps};
use toml::*;
use serde::{Deserialize, Serialize};

use crate::signals::*;

fn random_song() -> &'static str{
    let ele = Element::None;
    let val = random_0max(12);
    match val {
        0 => return "assets/music/0.ogg",
        1 => return "assets/music/1.ogg",
        2 => return "assets/music/2.ogg",
        3 => return "assets/music/3.ogg",
        4 => return "assets/music/4.ogg",
        5 => return "assets/music/5.ogg",
        6 => return "assets/music/6.ogg",
        7 => return "assets/music/7.ogg",
        8 => return "assets/music/8.ogg",
        9 => return "assets/music/9.ogg",
        10 => return "assets/music/10.ogg",
        11 => return "assets/music/11.ogg",
        _=> return "assets/music/12.ogg",
    }
}
fn random_image()->String {
    let max = 7;
    let val = random_0max(max);
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
fn get_image(creature:Creature, view:View) -> Option<SharedImage>{
    let mut res = creature.image.to_owned();
    
    res.push('/');
    match view {
        View::Left => res.push_str("left"),
        View::Right => res.push_str("right"),
        View::HurtLeft => res.push_str("left_hurt"),
        View::HurtRight => res.push_str("right_hurt"),
        View::AttackRight => res.push_str("right_attack"),
        View::AttackLeft => res.push_str("left_attack"),
        View::Icon => res.push_str("icon"),
    }
    res.push_str(".png");
    let mut image_filename:String = match fs::canonicalize(res.as_str()) {
        Ok(image_filename) => image_filename.to_str().unwrap().to_owned(),
        Err(e) => {
            println!("ERROR: {:?}\nFilename:{:?}",e, res);
            return None
        },
    };
    //println!("image:{:?}",image_filename);
    let mut image = SharedImage::load(image_filename.as_str());
    if image.is_ok() {
        let current_image = image.ok().unwrap();
        return Some(current_image)
    }
    None
}

fn main() {
    let app = app::App::default();
    let mut ui = creature_fighter::UserInterface::make_window();
    ui.win.show();
    let (send_action, receive_action) = app::channel::<Action>();
// Math Menu
    ui.check_answer.emit(send_action, Action::Check);
// Main bottom menu
    ui.fight.emit(send_action, Action::Fight);
    ui.run.emit(send_action, Action::Run);
    ui.item.emit(send_action, Action::Item(ItemScreen::Show));
    ui.switch.emit(send_action, Action::Switch(SwitchScreen::Show));
// Fight menu
    ui.move_button_0.emit(send_action, Action::Move0);
    ui.move_button_1.emit(send_action, Action::Move1);
    ui.move_button_2.emit(send_action, Action::Move2);
    ui.move_button_3.emit(send_action, Action::Move3);
// Switch screen
    // Choose
    ui.choose_0.emit(send_action, Action::Switch(SwitchScreen::Choose(0)));
    ui.choose_1.emit(send_action, Action::Switch(SwitchScreen::Choose(1)));
    ui.choose_2.emit(send_action, Action::Switch(SwitchScreen::Choose(2)));
    ui.choose_3.emit(send_action, Action::Switch(SwitchScreen::Choose(3)));
    ui.choose_4.emit(send_action, Action::Switch(SwitchScreen::Choose(4)));
    //Creature
    ui.show_creature_0.emit(send_action, Action::Switch(SwitchScreen::Stats(0)));
    ui.show_creature_1.emit(send_action, Action::Switch(SwitchScreen::Stats(1)));
    ui.show_creature_2.emit(send_action, Action::Switch(SwitchScreen::Stats(2)));
    ui.show_creature_3.emit(send_action, Action::Switch(SwitchScreen::Stats(3)));
    ui.show_creature_4.emit(send_action, Action::Switch(SwitchScreen::Stats(4)));
    ui.hide_stats.emit(send_action, Action::Switch(SwitchScreen::HideStats));
// Item Screen
    ui.use_items.emit(send_action, Action::Item(ItemScreen::Items(ItemUsage::Use)));
    ui.use_herbs.emit(send_action, Action::Item(ItemScreen::Herbs(ItemUsage::Use)));
    ui.use_relics.emit(send_action, Action::Item(ItemScreen::Relics(ItemUsage::Use)));
    ui.give_items.emit(send_action, Action::Item(ItemScreen::Items(ItemUsage::Give)));
    ui.give_herbs.emit(send_action, Action::Item(ItemScreen::Herbs(ItemUsage::Give)));
    ui.give_relics.emit(send_action, Action::Item(ItemScreen::Relics(ItemUsage::Give)));
    ui.toss_items.emit(send_action, Action::Item(ItemScreen::Items(ItemUsage::Toss)));
    ui.toss_herbs.emit(send_action, Action::Item(ItemScreen::Herbs(ItemUsage::Toss)));
    ui.toss_relics.emit(send_action, Action::Item(ItemScreen::Relics(ItemUsage::Toss)));
    ui.sound.emit(send_action, Action::ToggleMusic);
    let mut enemy_iter = 0;
    let mut player = Creature::new();
    player = player.random_type();
    player.image = random_image();
    let mut name = player.name.clone();
    let mut creatures:Vec<Creature> = vec![player.clone()];
    let mut enemy = Creature::new();
    let mut enemies:Vec<Creature> = vec![enemy.random_type(), enemy.random_type(), enemy.random_type(), enemy.random_type(), enemy.random_type(), enemy.random_type()];
    enemy = enemies[0].clone();
    ui.hp_guage.set_maximum(player.hp_max);
    ui.hp_guage.set_value(player.hp);
    ui.xp_guage.set_maximum(player.next());
    ui.xp_guage.set_value(player.xp);
    ui.enemy_hp.set_maximum(enemy.hp_max);
    ui.enemy_hp.set_value(enemy.hp);
    ui.player.set_image(get_image(player.clone(), View::Right));
    ui.player.set_label(player.name.as_str());
    enemy.image = random_image();
    ui.enemy.set_image(get_image(enemy.clone(), View::Left));
    ui.enemy.set_label(enemy.name.as_str());

    ui.move_guage_0.set_maximum(player.move0_mp);
    ui.move_button_0.set_label(player.move0.to_string().as_str());
    ui.move_guage_1.set_maximum(player.move1_mp);
    ui.move_button_1.set_label(player.move1.to_string().as_str());
    // timer
    let mut elapsed:f64 = 0.0;
    let threshold:f64 = 500.0;
    let busy:bool = false;
    let mut playing:bool = false;
    let mut current_move:u32 = 0;
    //TODO config file read opts
/*
music = true
name = 
hp = 
hp_max = 
hp_effort = 
xp = 
total_xp = 
moves = { name="element/condition" }
items = { item="element/condition" }

*/
    ui.win.redraw();
    let song = random_song();
    let mut bg_music = Music::new(song).unwrap();
    // loop music
    bg_music.set_looping(true);
    if playing {
        bg_music.play();
    }
    while app.wait() {
        if elapsed > threshold {
            elapsed = 0.0;
        }
        if busy {
            //do stuff
        }
        if let Some(button_action) = receive_action.recv() {
            match button_action {
                Action::ToggleMusic => {
                    playing = !playing;
                    if playing {
                        bg_music.play();
                    } else {
                        bg_music.stop();
                    }
                }
                Action::Check => {
                  //TODO
                    let mut passed:bool = false;
                    // eq 0
                    ui.eq0_answer.set_color(Color::by_index(135));
                    let dmg = ui.eq0_dmg.value();
                    let atk = ui.eq0_atk.value();
                    let mut ans0 = dmg * atk;
                    ans0 = (ans0 * 0.5).round() / 0.5;
                    let u_ans0 = ui.eq0_answer.value();
                    println!("answer0:{} vs {}", ans0, u_ans0);
                    if ans0 != u_ans0 {
                        ui.eq0_answer.set_color(Color::by_index(1));
                        ui.win.redraw();
                    } else {
                        ui.eq0_answer.set_color(Color::by_index(135));
                        ui.win.redraw();
                        passed = true;
                    }
                    // eq 1
                    let def = ui.eq1_def.value();
                    let hp = ui.eq1_hp.value();
                    let mut ans1 = def + hp;
                    ans1 = (ans1 * 0.5).round() / 0.5;
                    let u_ans1 = ui.eq1_answer.value();
                    println!("answer1:{} vs {}", ans1, u_ans1);
                    if ans1 != u_ans1 {
                        ui.eq1_answer.set_color(Color::by_index(1));
                        ui.win.redraw();
                        passed = false;
                    } else {
                        ui.eq1_answer.set_color(Color::by_index(135));
                        ui.win.redraw();
                    }
                    let at = ui.eq2_atk.value();
                    let lvl = ui.eq2_level.value();
                    let mut ans2 = ((at * 100.0).round() / 100.0) - ((lvl * 100.0).round() / 100.0);
                    ans2 = (ans2 * 0.5).round() / 0.5;
                    let u_ans2 = ui.eq2_answer.value();
                    println!("answer3:{} vs {}", ans2, u_ans2);
                    if ans2 != u_ans2 {
                        ui.eq2_answer.set_color(Color::by_index(1));
                        ui.win.redraw();
                        passed = false;
                    } else {
                        ui.eq2_answer.set_color(Color::by_index(135));
                        ui.win.redraw();
                    }
                    if !passed {
                        continue
                    }
                    let special_move = player.clone().get_move(current_move);
                    let dmg = special_move.damage(player.level);
                    let res = (dmg * 0.5).round() / 0.5;
                    enemy.hp -= res;
                    player.use_mp(current_move);
                    let ui_mp = player.get_mp(current_move);
                    let special_name = player.clone().get_move(current_move);
                    match current_move {
                        1 => {
                            ui.move_guage_1.set_value(ui_mp);
                            ui.move_guage_1.set_label(special_name.to_string().as_str());
                        }
                        2 => {
                            ui.move_guage_2.set_value(ui_mp);
                            ui.move_guage_2.set_label(special_name.to_string().as_str());
                        }
                        3=> {
                            ui.move_guage_3.set_value(ui_mp);
                            ui.move_guage_3.set_label(special_name.to_string().as_str());
                        }
                        4=> {
                            //ui.move_guage_4.set_value(ui_mp);
                            //ui.move_guage_4.set_label(special_name.to_string().as_str());
                        }
                        _ => {
                            ui.move_guage_0.set_value(ui_mp);
                            ui.move_guage_0.set_label(special_name.to_string().as_str());
                        }
                    }
                    ui.win.redraw();
                    ui.enemy_hp.set_value(enemy.hp);
                    if enemy.hp == 0.0 {
                        player.xp += 100.0 - enemy.rate;
                        player.level_up();
                        
                        if enemy.items().len() > 0 {
                            player.add_item(enemy.items()[0]);
                        }
                        ui.xp_guage.set_value(player.xp);
                        ui.xp_guage.set_maximum(player.next());
                        enemy_iter += 1;
                        if enemy_iter >= enemies.len() {
                            enemy_iter = 0;
                        }
                        enemy = enemies[enemy_iter].clone();
                        enemy.image = random_image();
                        enemy.name = random_creature_name().to_owned();
                        ui.enemy.set_image(get_image(enemy.clone(), View::Left));
                        ui.enemy.set_label(enemy.name.as_str());
                        ui.enemy_hp.set_maximum(enemy.hp_max);
                        ui.enemy_hp.set_value(enemy.hp);
                    }
                    if playing {
                        let song = random_song();
                        bg_music.stop();
                        bg_music = Music::new(song).unwrap();
                        bg_music.play();
                    }
                    current_move = 0;
                    ui.math.hide();
                },
                Action::Move0 => {
                    current_move = 0;
                    if player.clone().valid_move(current_move) {
                        let atk_move = player.get_move(current_move);
                        let mp:f64 = atk_move.mp_total(0.0);
                        if mp == 0.0 {
                            continue
                        }
                        ui.math.show();
                        if playing {
                            let song = random_song();
                            bg_music.stop();
                            bg_music = Music::new(song).unwrap();
                            bg_music.play();
                        }
                        ui.eq0_dmg.set_value(atk_move.damage(player.level));
                        ui.eq0_atk.set_value(player.atk);
                        ui.eq0_answer.set_value(0.0);
                        let def = (enemy.def * 0.5).round() / 0.5;
                        ui.eq1_def.set_value(def);
                        let hp = (enemy.hp * 0.5).round() / 0.5;
                        ui.eq1_hp.set_value(hp);
                        ui.eq1_answer.set_value(0.0);
                        let mut atk = enemy.random(1.0, enemy.atk + 2.0);
                        atk = (atk * 0.5).round() / 0.5;
                        let mut level = enemy.random(1.0, enemy.level + 2.0);
                        level = (level * 0.5).round() / 0.5;
                        ui.eq2_atk.set_value(atk);
                        ui.eq2_level.set_value(level);
                        ui.eq2_answer.set_value(0.0);
                    }
                },
                Action::Move1 => {
                    current_move = 1;
                    if player.clone().valid_move(current_move) {
                        let atk_move = player.get_move(current_move);
                        let mp:f64 = atk_move.mp_total(0.0);
                        if mp == 0.0 {
                            continue
                        }
                        ui.math.show();
                        if playing {
                            let song = random_song();
                            bg_music.stop();
                            bg_music = Music::new(song).unwrap();
                            bg_music.play();
                        }
                        ui.eq0_dmg.set_value(atk_move.damage(player.level));
                        ui.eq0_atk.set_value(player.atk);
                        ui.eq0_answer.set_value(0.0);
                        let def = (enemy.def * 0.5).round() / 0.5;
                        ui.eq1_def.set_value(def);
                        let hp = (enemy.hp * 0.5).round() / 0.5;
                        ui.eq1_hp.set_value(hp);
                        ui.eq1_answer.set_value(0.0);
                        let mut atk = enemy.random(1.0, enemy.atk + 2.0);
                        atk = (atk * 0.5).round() / 0.5;
                        let mut level = enemy.random(1.0, enemy.level + 2.0);
                        level = (level * 0.5).round() / 0.5;
                        ui.eq2_atk.set_value(atk);
                        ui.eq2_level.set_value(level);
                        ui.eq2_answer.set_value(0.0);
                    }
                },
                Action::Move2 => {
                    current_move = 2;
                    if player.clone().valid_move(current_move) {
                        let atk_move = player.get_move(current_move);
                        let mp:f64 = atk_move.mp_total(0.0);
                        if mp == 0.0 {
                            continue
                        }
                        ui.math.show();
                        if playing {
                            let song = random_song();
                            bg_music.stop();
                            bg_music = Music::new(song).unwrap();
                            bg_music.play();
                        }
                        ui.eq0_dmg.set_value(atk_move.damage(player.level));
                        ui.eq0_atk.set_value(player.atk);
                        ui.eq0_answer.set_value(0.0);
                        let def = (enemy.def * 0.5).round() / 0.5;
                        ui.eq1_def.set_value(def);
                        let hp = (enemy.hp * 0.5).round() / 0.5;
                        ui.eq1_hp.set_value(hp);
                        ui.eq1_answer.set_value(0.0);
                        let mut atk = enemy.random(1.0, enemy.atk + 2.0);
                        atk = (atk * 0.5).round() / 0.5;
                        let mut level = enemy.random(1.0, enemy.level + 2.0);
                        level = (level * 0.5).round() / 0.5;
                        ui.eq2_atk.set_value(atk);
                        ui.eq2_level.set_value(level);
                        ui.eq2_answer.set_value(0.0);
                    }
                },
                Action::Move3 => {
                    current_move = 3;
                    if player.clone().valid_move(current_move) {
                        let atk_move = player.get_move(current_move);
                        let mp:f64 = atk_move.mp_total(0.0);
                        if mp == 0.0 {
                            continue
                        }
                        ui.math.show();
                        if playing {
                            let song = random_song();
                            bg_music.stop();
                            bg_music = Music::new(song).unwrap();
                            bg_music.play();
                        }
                        ui.eq0_dmg.set_value(atk_move.damage(player.level));
                        ui.eq0_atk.set_value(player.atk);
                        ui.eq0_answer.set_value(0.0);
                        let def = (enemy.def * 0.5).round() / 0.5;
                        ui.eq1_def.set_value(def);
                        let hp = (enemy.hp * 0.5).round() / 0.5;
                        ui.eq1_hp.set_value(hp);
                        ui.eq1_answer.set_value(0.0);
                        let mut atk = enemy.random(1.0, enemy.atk + 2.0);
                        atk = (atk * 0.5).round() / 0.5;
                        let mut level = enemy.random(1.0, enemy.level + 2.0);
                        level = (level * 0.5).round() / 0.5;
                        ui.eq2_atk.set_value(atk);
                        ui.eq2_level.set_value(level);
                        ui.eq2_answer.set_value(0.0);
                    }
                },
                Action::Fight => {
                    if ui.special_moves.visible() {
                        ui.special_moves.hide();
                    } else {
                        ui.special_moves.show();
                        let special = player.clone().get_move(0);
                        ui.move_guage_0.set_value(player.clone().get_mp(0));
                        ui.move_guage_0.set_maximum(special.mp_total(0.0));
                        ui.move_guage_0.set_label(special.to_string().as_str());
                        let special = player.clone().get_move(1);
                        ui.move_guage_1.set_value(player.clone().get_mp(1));
                        ui.move_guage_1.set_maximum(special.mp_total(0.0));
                        ui.move_guage_1.set_label(special.to_string().as_str());
                        let special = player.clone().get_move(2);
                        ui.move_guage_2.set_value(player.clone().get_mp(2));
                        ui.move_guage_2.set_maximum(special.mp_total(0.0));
                        ui.move_guage_2.set_label(special.to_string().as_str());
                        let special = player.clone().get_move(3);
                        ui.move_guage_3.set_maximum(special.mp_total(0.0));
                        ui.move_guage_3.set_value(player.clone().get_mp(3));
                        ui.move_guage_3.set_label(special.to_string().as_str());
                    }
                },
                Action::Item(item_screen) => {
                    match item_screen {
                        ItemScreen::Show => {
                            if playing {
                                let song = random_song();
                                bg_music.stop();
                                bg_music = Music::new(song).unwrap();
                                bg_music.play();
                            }
                            ui.item_screen.show();
                            ui.items.clear();
                            for item in player.clone().items() {
                                let v = item.to_string();
                                ui.items.add(v.as_str());
                            }
                        },
                        ItemScreen::Items(usage) => {
                            let item = ui.items.value() - 1;
                            if item < 0 {
                                continue
                            }
                            match usage {
                                ItemUsage::Use => {
                                    player.use_item(item as u32);
                                    //
                                    ui.hp_guage.set_maximum(player.hp_max);
                                    ui.hp_guage.set_value(player.hp);
                                    ui.xp_guage.set_maximum(player.next());
                                    ui.xp_guage.set_value(player.xp);
                                    ui.player.set_image(get_image(player.clone(), View::Right));
                                    // special guages
                                    let special = player.clone().get_move(0);
                                    ui.move_guage_0.set_value(player.clone().get_mp(0));
                                    ui.move_guage_0.set_maximum(special.mp_total(0.0));
                                    ui.move_guage_0.set_value(player.clone().get_mp(0));
                                    ui.move_guage_0.set_maximum(special.mp_total(0.0));
                                    ui.move_guage_0.set_label(special.to_string().as_str());
                                    let special = player.clone().get_move(1);
                                    ui.move_guage_1.set_value(player.clone().get_mp(1));
                                    ui.move_guage_1.set_maximum(special.mp_total(0.0));
                                    ui.move_guage_1.set_label(special.to_string().as_str());
                                    let special = player.clone().get_move(2);
                                    ui.move_guage_2.set_value(player.clone().get_mp(2));
                                    ui.move_guage_2.set_maximum(special.mp_total(0.0));
                                    ui.move_guage_2.set_label(special.to_string().as_str());
                                    let special = player.clone().get_move(3);
                                    ui.move_guage_3.set_maximum(special.mp_total(0.0));
                                    ui.move_guage_3.set_value(player.clone().get_mp(3));
                                    ui.move_guage_3.set_label(special.to_string().as_str());
                                },
                                ItemUsage::Give => {ui.item_screen.hide();},
                                ItemUsage::Toss => {ui.item_screen.hide();},
                            }
                        },
                        ItemScreen::Herbs(usage) => {
                            ui.item_screen.hide();
                        },
                        ItemScreen::Relics(usage) => {
                            ui.item_screen.hide();
                        },
                    }
                    
                },
                Action::Switch(switch) => {
                    match switch {
                        SwitchScreen::Show => {
                            if playing {
                                let song = random_song();
                                bg_music.stop();
                                bg_music = Music::new(song).unwrap();
                                bg_music.play();
                            }
                            ui.switch_screen.show();
                            let c = creatures                          [0].clone();
                            ui.choose_0.set_image(get_image(c.clone(), View::Icon));
                            ui.hp_creature_0.set_value(c.hp);
                            ui.hp_creature_0.set_maximum(c.hp_max);
                            ui.xp_creature_0.set_value(c.level);
                            ui.xp_creature_0.set_maximum(100.0);
                            
                            
                        },
                        SwitchScreen::Choose(num) => {
                            if num < creatures.len() {
                                player = creatures[num].clone();
                                // TODO LOAD
                                ui.player.set_label(player.name.as_str());
                                ui.hp_guage.set_maximum(player.hp_max);
                                ui.hp_guage.set_value(player.hp);
                                ui.xp_guage.set_maximum(player.next());
                                ui.xp_guage.set_value(player.xp);
                                ui.player.set_image(get_image(player.clone(), View::Right));
                            }
                            if playing {
                                let song = random_song();
                                bg_music.stop();
                                bg_music = Music::new(song).unwrap();
                                bg_music.play();
                            }
                            ui.switch_screen.hide();
                        },
                        SwitchScreen::Stats(num) => {
                            println!("num={}",num);
                            if num <= creatures.len() {
                                ui.stats_screen.show();
                                ui.stat_viewer.begin();
                                let s = creatures[num].clone().generate();
                                ui.stat_viewer.end();
                                ui.stats_image.set_image(get_image(creatures[num].clone(), View::Right));
                            }
                            ui.switch_screen.hide();
                        },
                        SwitchScreen::Item(num) => {
                            ui.switch_screen.hide();
                        },
                        SwitchScreen::HideStats => {
                            if playing {
                                let song = random_song();
                                bg_music.stop();
                                bg_music = Music::new(song).unwrap();
                                bg_music.play();
                            }
                            ui.stats_screen.hide();
                            ui.win.redraw();
                        },
                    }
                    
                },
                Action::Run => {
                    if playing { 
                        let song = random_song();
                        bg_music.stop();
                        bg_music = Music::new(song).unwrap();
                        bg_music.play();
                        enemy_iter += 1;
                    }
                    if enemy_iter >= enemies.len() {
                        enemy_iter = 0;
                    }
                    enemy = enemies[enemy_iter].clone();
                    enemy.image = random_image();
                    enemy.name = random_creature_name().to_owned();
                    ui.enemy.set_image(get_image(enemy.clone(), View::Left));
                    ui.enemy_hp.set_maximum(enemy.hp_max);
                    ui.enemy_hp.set_value(enemy.hp);
                    ui.enemy.set_label(enemy.name.as_str());
                },
                _=> {},
            }
        }
        ui.win.redraw();
        elapsed += 1.0;
    }
}
