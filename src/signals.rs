/*!
use crate::signals::{ItemUsage, ItemScreen, SwitchScreen, Action};
*/
#[derive(Debug, Clone, Copy)]
pub enum View {
    Left,
    Right,
    HurtLeft,
    HurtRight,
    AttackRight,
    AttackLeft,
    Icon,
}

#[derive(Debug, Clone, Copy)]
pub enum ItemUsage {
    Use,
    Give,
    Toss,
}

#[derive(Debug, Clone, Copy)]
pub enum ItemScreen {
    Show,
    Items(ItemUsage),
    Herbs(ItemUsage),
    Relics(ItemUsage),
}
#[derive(Debug, Clone, Copy)]
pub enum SwitchScreen {
    Show,
    Choose(usize),
    Stats(usize),
    Item(usize),
    HideStats,
}

#[derive(Debug, Clone, Copy)]
pub enum Action {
    Move0,
    Move1,
    Move2,
    Move3,
    Fight,
    Run,
    Item(ItemScreen),
    Switch(SwitchScreen),
    Check,
    Math,
}
