mod creature_fighter;
mod creature;
mod item;
mod item_images;
mod signals;
mod condition;
mod moves;
mod element;

use std::fs;

// GUI
use fltk::{prelude::*, enums::*, image::*, *, input::Input, menu::MenuButton, menu::MenuItem, valuator::ValueInput, group::Group, window::Window};

//use fltk_form::{FltkForm, HasProps};

use crate::creature::*;
use crate::signals::*;
use crate::element::*;
use crate::item::*;

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
    println!("image:{:?}",image_filename);
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
    //Stats
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
    let mut enemy_iter = 0;
    let mut player = Creature::from_element(Element::Electric);
    let mut creatures:Vec<Creature> = vec![player.clone()];
    let mut enemy = Creature::new();
    let enemies:Vec<Creature> = vec![enemy.random_type(), enemy.random_type(), enemy.random_type(), enemy.random_type(), enemy.random_type(), enemy.random_type(), Creature::from_element(Element::Plant), Creature::from_element(Element::Water)];
    enemy = enemies[0].clone();
    ui.hp_guage.set_maximum(player.hp_max);
    ui.hp_guage.set_value(player.hp);
    ui.xp_guage.set_maximum(player.next());
    ui.xp_guage.set_value(player.xp);
    ui.enemy_hp.set_maximum(enemy.hp_max);
    ui.enemy_hp.set_value(enemy.hp);
    ui.player.set_image(get_image(player.clone(), View::Right));
    ui.player.set_label(player.name.as_str());
    ui.enemy.set_image(get_image(enemy.clone(), View::Left));
    ui.enemy.set_label(enemy.name.as_str());

    let special = player.moves[0].clone();
    ui.move_guage_0.set_maximum(special.mp);
    ui.move_button_0.set_label(special.name().as_str());
    let special = player.moves[1].clone();
    ui.move_guage_1.set_maximum(special.mp);
    ui.move_button_1.set_label(special.name().as_str());
    // timer
    let mut elapsed:f64 = 0.0;
    let threshold:f64 = 500.0;
    let busy:bool = false;
    let mut check_this:usize = 0;
    ui.win.redraw();
    
    while app.wait() {
        if elapsed > threshold {
            elapsed = 0.0;
        }
        if busy {
            //do stuff
        }
        if let Some(button_action) = receive_action.recv() {
            match button_action {
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
                    //
                    let mut special = player.moves[check_this].clone();
                    let mp = special.mp;
                    if mp == 0.0 {
                        ui.move_guage_1.set_value(mp);
                        ui.move_guage_1.set_label(mp.to_string().as_str());
                        continue
                    }
                    let dmg = match player.special(check_this, enemy.clone()) {
                        Some(dmg) => dmg,
                        None => continue,
                    };
                    let res = (dmg * 0.5).round() / 0.5;
                    enemy.hp -= res;
                    player.moves[check_this].mp -= 1.0;
                    let ui_mp = player.moves[check_this].mp;
                    match check_this {
                        0 => {
                            ui.move_guage_0.set_value(ui_mp);
                            ui.move_guage_0.set_label(ui_mp.to_string().as_str());
                        }
                        1 => {
                            ui.move_guage_1.set_value(ui_mp);
                            ui.move_guage_1.set_label(ui_mp.to_string().as_str());
                        }
                        2 => {
                            ui.move_guage_2.set_value(ui_mp);
                            ui.move_guage_2.set_label(ui_mp.to_string().as_str());
                        }
                        _=> {
                            ui.move_guage_3.set_value(ui_mp);
                            ui.move_guage_3.set_label(ui_mp.to_string().as_str());
                        }
                    }
                    ui.win.redraw();
                    ui.enemy_hp.set_value(enemy.hp);
                    if enemy.hp == 0.0 {
                        player.xp += 100.0 - enemy.rate;
                        player.level_up();
                        
                        if enemy.items.len() > 0 {
                            player.get_item(enemy.items[0]);
                        }
                        ui.xp_guage.set_value(player.xp);
                        ui.xp_guage.set_maximum(player.next());
                        enemy_iter += 1;
                        if enemy_iter >= enemies.len() {
                            enemy_iter = 0;
                        }
                        enemy = enemies[enemy_iter].clone();
                        ui.enemy.set_image(get_image(enemy.clone(), View::Left));
                        ui.enemy.set_label(enemy.name.as_str());
                        ui.enemy_hp.set_maximum(enemy.hp_max);
                        ui.enemy_hp.set_value(enemy.hp);
                    }
                  check_this = 0;
                  ui.math.hide();
                },
                Action::Move0 => {
                    check_this = 0;
                    if player.moves.clone().len()  > check_this {
                        let atk_move = player.moves[check_this].clone();
                        let mp = atk_move.mp;
                        if mp == 0.0 {
                            continue
                        }
                        ui.math.show();
                        ui.eq0_dmg.set_value(player.get_damage(atk_move.clone(), enemy.element1));
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
                    check_this = 1;
                    if player.moves.clone().len()  > check_this {
                        let atk_move = player.moves[check_this].clone();
                        let mp = atk_move.mp;
                        if mp == 0.0 {
                            continue
                        }
                        ui.math.show();
                        ui.eq0_dmg.set_value(player.get_damage(atk_move.clone(), enemy.element1));
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
                    check_this = 2;
                    if player.moves.clone().len()  > check_this {
                        let atk_move = player.moves[check_this].clone();
                        let mp = atk_move.mp;
                        if mp == 0.0 {
                            continue
                        }
                        ui.math.show();
                        ui.eq0_dmg.set_value(player.get_damage(atk_move.clone(), enemy.element1));
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
                    check_this = 3;
                    if player.moves.clone().len()  > check_this {
                        let atk_move = player.moves[check_this].clone();
                        let mp = atk_move.mp;
                        if mp == 0.0 {
                            continue
                        }
                        ui.math.show();
                        ui.eq0_dmg.set_value(player.get_damage(atk_move.clone(), enemy.element1));
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
                        let special = player.moves[0].clone();
                        ui.move_guage_0.set_value(special.mp);
                        ui.move_guage_0.set_label(special.mp.to_string().as_str());
                        let special = player.moves[1].clone();
                        ui.move_guage_1.set_value(special.mp);
                        ui.move_guage_1.set_label(special.mp.to_string().as_str());
                    }
                },
                Action::Item(item_screen) => {
                    match item_screen {
                        ItemScreen::Show => {
                            ui.item_screen.show();
                            ui.items.clear();
                            for item in player.items.clone() {
                                let v = item.to_string();
                                ui.items.add(v.as_str());
                            }
                        },
                        ItemScreen::Items(usage) => {
                            let item = match ui.items.text(ui.items.value()) {
                                Some(item) => item,
                                None => {
                                    ui.item_screen.hide();
                                    continue
                                },
                            };
                            match usage {
                                ItemUsage::Use => {
                                    player.use_item(item);
                                    creatures[0] = player.clone();
                                    let special = player.moves[0].clone();
                                    ui.move_guage_0.set_value(special.mp);
                                    ui.move_guage_0.set_label(special.mp.to_string().as_str());
                                    let special = player.moves[1].clone();
                                    ui.move_guage_1.set_value(special.mp);
                                    ui.move_guage_1.set_label(special.mp.to_string().as_str());
                                    ui.item_screen.hide();
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
                            ui.switch_screen.hide();
                        },
                        SwitchScreen::Stats(num) => {
                            //HARDCODE TODO
                            creatures[0] = player.clone();
                            if num < creatures.len() {
                                let c = creatures[num].clone();
                                ui.stats_screen.show();
                                ui.hp.set_value(c.hp);
                                ui.xp.set_value(c.total_xp);
                                ui.level.set_value(c.level);
                                ui.id.set_value(c.id as f64);
                                ui.name.set_value(c.name.as_str());
                                ui.form.set_value(c.form.to_string().as_str());
                                ui.element1.set_value(c.element1.to_string().as_str());
                                ui.element2.set_value(c.element2.to_string().as_str());
                                ui.rate.set_value(c.rate);
                                ui.hp_effort.set_value(c.hp_effort);
                                ui.atk_effort.set_value(c.atk_effort);
                                ui.def_effort.set_value(c.def_effort);
                                ui.def.set_value(c.def);
                                ui.atk.set_value(c.atk);
                                ui.speed.set_value(c.speed);
                                ui.special.set_value(c.special);
                                ui.speed_effort.set_value(c.speed_effort);
                                ui.special_effort.set_value(c.special_effort);
                                ui.hp_max.set_value(c.hp_max);
                                ui.stats_image.set_image(get_image(c.clone(), View::Right));
                                ui.moves.clear();
                                for m in c.moves {
                                    ui.moves.add(m.to_string().as_str());
                                }
                                for m in c.items {
                                    ui.stat_items.add(m.to_string().as_str());
                                }
                                //ui..set_value(c.);
                                
                            }
                            ui.switch_screen.hide();
                        },
                        SwitchScreen::Item(num) => {
                            ui.switch_screen.hide();
                        },
                        SwitchScreen::HideStats => {
                            let name = ui.name.value().to_string();
                            player.name = name.to_owned();
                            ui.player.set_label(name.as_str());
                            ui.stats_screen.hide();
                            ui.win.redraw();
                        },
                    }
                    
                },
                Action::Run => {
                        enemy_iter += 1;
                        if enemy_iter >= enemies.len() {
                            enemy_iter = 0;
                        }
                        enemy = enemies[enemy_iter].clone();
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
