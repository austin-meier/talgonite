use std::time::Duration;

use bevy::input::gamepad::GamepadButton;
use bevy::input::keyboard::KeyCode;

use crate::gamepad::GamepadInputType;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GameActionMode {
    Edge,
    Continuous,
    Repeat { interval: Duration },
}

macro_rules! define_game_actions {
    (
        $(
            $variant:ident {
                id: $id:literal,
                label: $label:literal,
                mode: $mode:expr
                $(,
                    keyboard: $keyboard:expr,
                    gamepad: $gamepad:expr
                )?
            }
        ),+ $(,)?
    ) => {
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
        pub enum GameAction {
            $(
                $variant,
            )+
        }

        impl GameAction {
            pub fn mode(&self) -> GameActionMode {
                match self {
                    $(
                        Self::$variant => $mode,
                    )+
                }
            }

            pub fn repeat_interval(&self) -> Option<Duration> {
                match self.mode() {
                    GameActionMode::Repeat { interval } => Some(interval),
                    _ => None,
                }
            }

            pub fn all() -> &'static [GameAction] {
                &[
                    $(
                        GameAction::$variant,
                    )+
                ]
            }

            pub fn action_id(&self) -> &'static str {
                match self {
                    $(
                        Self::$variant => $id,
                    )+
                }
            }

            pub fn from_action_id(id: &str) -> Option<Self> {
                match id {
                    $(
                        $id => Some(Self::$variant),
                    )+
                    _ => None,
                }
            }

            pub fn label(&self) -> &'static str {
                match self {
                    $(
                        Self::$variant => $label,
                    )+
                }
            }

            pub fn default_keyboard_key(&self) -> Option<KeyCode> {
                match self {
                    $(
                        Self::$variant => define_game_actions!(@keyboard $($keyboard)?),
                    )+
                }
            }

            pub fn default_gamepad_inputs(&self) -> &'static [GamepadInputType] {
                match self {
                    $(
                        Self::$variant => define_game_actions!(@gamepad $($gamepad)?),
                    )+
                }
            }
        }
    };

    (@keyboard $keyboard:expr) => { $keyboard };
    (@keyboard) => { None };
    (@gamepad $gamepad:expr) => { $gamepad };
    (@gamepad) => { &[] };
}

define_game_actions! {
    MoveUp { id: "move_up", label: "Move Up", mode: GameActionMode::Continuous, keyboard: Some(KeyCode::ArrowUp), gamepad: &[GamepadInputType::Button(GamepadButton::DPadUp), GamepadInputType::LeftStickUp] },
    MoveDown { id: "move_down", label: "Move Down", mode: GameActionMode::Continuous, keyboard: Some(KeyCode::ArrowDown), gamepad: &[GamepadInputType::Button(GamepadButton::DPadDown), GamepadInputType::LeftStickDown] },
    MoveLeft { id: "move_left", label: "Move Left", mode: GameActionMode::Continuous, keyboard: Some(KeyCode::ArrowLeft), gamepad: &[GamepadInputType::Button(GamepadButton::DPadLeft), GamepadInputType::LeftStickLeft] },
    MoveRight { id: "move_right", label: "Move Right", mode: GameActionMode::Continuous, keyboard: Some(KeyCode::ArrowRight), gamepad: &[GamepadInputType::Button(GamepadButton::DPadRight), GamepadInputType::LeftStickRight] },
    Inventory { id: "inventory", label: "Inventory", mode: GameActionMode::Edge, keyboard: Some(KeyCode::KeyI), gamepad: &[GamepadInputType::Button(GamepadButton::North)] },
    Character { id: "character", label: "Character", mode: GameActionMode::Edge, keyboard: Some(KeyCode::KeyC), gamepad: &[] },
    Skills { id: "skills", label: "Skills", mode: GameActionMode::Edge, keyboard: Some(KeyCode::KeyK), gamepad: &[GamepadInputType::Button(GamepadButton::West)] },
    Spells { id: "spells", label: "Spells", mode: GameActionMode::Edge, keyboard: Some(KeyCode::KeyP), gamepad: &[GamepadInputType::Button(GamepadButton::East)] },
    Settings { id: "settings", label: "Settings", mode: GameActionMode::Edge, keyboard: Some(KeyCode::Escape), gamepad: &[GamepadInputType::Button(GamepadButton::Start)] },
    Refresh { id: "refresh", label: "Refresh", mode: GameActionMode::Edge, keyboard: Some(KeyCode::F5), gamepad: &[GamepadInputType::Button(GamepadButton::Select)] },
    ToggleOverview { id: "toggle_overview", label: "Tab overview", mode: GameActionMode::Edge, keyboard: Some(KeyCode::Tab), gamepad: &[] },
    BasicAttack { id: "basic_attack", label: "Basic Attack", mode: GameActionMode::Repeat { interval: Duration::from_millis(250) }, keyboard: Some(KeyCode::Space), gamepad: &[GamepadInputType::Button(GamepadButton::South)] },
    AutoAttackToggle { id: "auto_attack_toggle", label: "Auto Attack Toggle", mode: GameActionMode::Edge, keyboard: Some(KeyCode::KeyT), gamepad: &[] },
    ItemPickupBelow { id: "item_pickup_below", label: "Item Pickup", mode: GameActionMode::Edge, keyboard: Some(KeyCode::KeyB), gamepad: &[] },
    HotbarSlot1 { id: "hotbar_slot_1", label: "Slot 1", mode: GameActionMode::Repeat { interval: Duration::from_millis(250) }, keyboard: Some(KeyCode::Digit1), gamepad: &[] },
    HotbarSlot2 { id: "hotbar_slot_2", label: "Slot 2", mode: GameActionMode::Repeat { interval: Duration::from_millis(250) }, keyboard: Some(KeyCode::Digit2), gamepad: &[] },
    HotbarSlot3 { id: "hotbar_slot_3", label: "Slot 3", mode: GameActionMode::Repeat { interval: Duration::from_millis(250) }, keyboard: Some(KeyCode::Digit3), gamepad: &[] },
    HotbarSlot4 { id: "hotbar_slot_4", label: "Slot 4", mode: GameActionMode::Repeat { interval: Duration::from_millis(250) }, keyboard: Some(KeyCode::Digit4), gamepad: &[] },
    HotbarSlot5 { id: "hotbar_slot_5", label: "Slot 5", mode: GameActionMode::Repeat { interval: Duration::from_millis(250) }, keyboard: Some(KeyCode::Digit5), gamepad: &[] },
    HotbarSlot6 { id: "hotbar_slot_6", label: "Slot 6", mode: GameActionMode::Repeat { interval: Duration::from_millis(250) }, keyboard: Some(KeyCode::Digit6), gamepad: &[] },
    HotbarSlot7 { id: "hotbar_slot_7", label: "Slot 7", mode: GameActionMode::Repeat { interval: Duration::from_millis(250) }, keyboard: Some(KeyCode::Digit7), gamepad: &[] },
    HotbarSlot8 { id: "hotbar_slot_8", label: "Slot 8", mode: GameActionMode::Repeat { interval: Duration::from_millis(250) }, keyboard: Some(KeyCode::Digit8), gamepad: &[] },
    HotbarSlot9 { id: "hotbar_slot_9", label: "Slot 9", mode: GameActionMode::Repeat { interval: Duration::from_millis(250) }, keyboard: Some(KeyCode::Digit9), gamepad: &[] },
    HotbarSlot10 { id: "hotbar_slot_10", label: "Slot 10", mode: GameActionMode::Repeat { interval: Duration::from_millis(250) }, keyboard: Some(KeyCode::Digit0), gamepad: &[] },
    HotbarSlot11 { id: "hotbar_slot_11", label: "Slot 11", mode: GameActionMode::Repeat { interval: Duration::from_millis(250) }, keyboard: Some(KeyCode::Minus), gamepad: &[] },
    HotbarSlot12 { id: "hotbar_slot_12", label: "Slot 12", mode: GameActionMode::Repeat { interval: Duration::from_millis(250) }, keyboard: Some(KeyCode::Equal), gamepad: &[] },
    HotbarSlot13 { id: "hotbar_slot_13", label: "Slot 13", mode: GameActionMode::Repeat { interval: Duration::from_millis(250) }, keyboard: Some(KeyCode::Digit1), gamepad: &[] },
    HotbarSlot14 { id: "hotbar_slot_14", label: "Slot 14", mode: GameActionMode::Repeat { interval: Duration::from_millis(250) }, keyboard: Some(KeyCode::Digit2), gamepad: &[] },
    HotbarSlot15 { id: "hotbar_slot_15", label: "Slot 15", mode: GameActionMode::Repeat { interval: Duration::from_millis(250) }, keyboard: Some(KeyCode::Digit3), gamepad: &[] },
    HotbarSlot16 { id: "hotbar_slot_16", label: "Slot 16", mode: GameActionMode::Repeat { interval: Duration::from_millis(250) }, keyboard: Some(KeyCode::Digit4), gamepad: &[] },
    HotbarSlot17 { id: "hotbar_slot_17", label: "Slot 17", mode: GameActionMode::Repeat { interval: Duration::from_millis(250) }, keyboard: Some(KeyCode::Digit5), gamepad: &[] },
    HotbarSlot18 { id: "hotbar_slot_18", label: "Slot 18", mode: GameActionMode::Repeat { interval: Duration::from_millis(250) }, keyboard: Some(KeyCode::Digit6), gamepad: &[] },
    HotbarSlot19 { id: "hotbar_slot_19", label: "Slot 19", mode: GameActionMode::Repeat { interval: Duration::from_millis(250) }, keyboard: Some(KeyCode::Digit7), gamepad: &[] },
    HotbarSlot20 { id: "hotbar_slot_20", label: "Slot 20", mode: GameActionMode::Repeat { interval: Duration::from_millis(250) }, keyboard: Some(KeyCode::Digit8), gamepad: &[] },
    HotbarSlot21 { id: "hotbar_slot_21", label: "Slot 21", mode: GameActionMode::Repeat { interval: Duration::from_millis(250) }, keyboard: Some(KeyCode::Digit9), gamepad: &[] },
    HotbarSlot22 { id: "hotbar_slot_22", label: "Slot 22", mode: GameActionMode::Repeat { interval: Duration::from_millis(250) }, keyboard: Some(KeyCode::Digit0), gamepad: &[] },
    HotbarSlot23 { id: "hotbar_slot_23", label: "Slot 23", mode: GameActionMode::Repeat { interval: Duration::from_millis(250) }, keyboard: Some(KeyCode::Minus), gamepad: &[] },
    HotbarSlot24 { id: "hotbar_slot_24", label: "Slot 24", mode: GameActionMode::Repeat { interval: Duration::from_millis(250) }, keyboard: Some(KeyCode::Equal), gamepad: &[] },
    HotbarSlot25 { id: "hotbar_slot_25", label: "Slot 25", mode: GameActionMode::Repeat { interval: Duration::from_millis(250) }, keyboard: Some(KeyCode::Digit1), gamepad: &[] },
    HotbarSlot26 { id: "hotbar_slot_26", label: "Slot 26", mode: GameActionMode::Repeat { interval: Duration::from_millis(250) }, keyboard: Some(KeyCode::Digit2), gamepad: &[] },
    HotbarSlot27 { id: "hotbar_slot_27", label: "Slot 27", mode: GameActionMode::Repeat { interval: Duration::from_millis(250) }, keyboard: Some(KeyCode::Digit3), gamepad: &[] },
    HotbarSlot28 { id: "hotbar_slot_28", label: "Slot 28", mode: GameActionMode::Repeat { interval: Duration::from_millis(250) }, keyboard: Some(KeyCode::Digit4), gamepad: &[] },
    HotbarSlot29 { id: "hotbar_slot_29", label: "Slot 29", mode: GameActionMode::Repeat { interval: Duration::from_millis(250) }, keyboard: Some(KeyCode::Digit5), gamepad: &[] },
    HotbarSlot30 { id: "hotbar_slot_30", label: "Slot 30", mode: GameActionMode::Repeat { interval: Duration::from_millis(250) }, keyboard: Some(KeyCode::Digit6), gamepad: &[] },
    HotbarSlot31 { id: "hotbar_slot_31", label: "Slot 31", mode: GameActionMode::Repeat { interval: Duration::from_millis(250) }, keyboard: Some(KeyCode::Digit7), gamepad: &[] },
    HotbarSlot32 { id: "hotbar_slot_32", label: "Slot 32", mode: GameActionMode::Repeat { interval: Duration::from_millis(250) }, keyboard: Some(KeyCode::Digit8), gamepad: &[] },
    HotbarSlot33 { id: "hotbar_slot_33", label: "Slot 33", mode: GameActionMode::Repeat { interval: Duration::from_millis(250) }, keyboard: Some(KeyCode::Digit9), gamepad: &[] },
    HotbarSlot34 { id: "hotbar_slot_34", label: "Slot 34", mode: GameActionMode::Repeat { interval: Duration::from_millis(250) }, keyboard: Some(KeyCode::Digit0), gamepad: &[] },
    HotbarSlot35 { id: "hotbar_slot_35", label: "Slot 35", mode: GameActionMode::Repeat { interval: Duration::from_millis(250) }, keyboard: Some(KeyCode::Minus), gamepad: &[] },
    HotbarSlot36 { id: "hotbar_slot_36", label: "Slot 36", mode: GameActionMode::Repeat { interval: Duration::from_millis(250) }, keyboard: Some(KeyCode::Equal), gamepad: &[] },
    HotbarSlot37 { id: "hotbar_slot_37", label: "Slot 37", mode: GameActionMode::Repeat { interval: Duration::from_millis(250) }, keyboard: Some(KeyCode::Digit1), gamepad: &[] },
    HotbarSlot38 { id: "hotbar_slot_38", label: "Slot 38", mode: GameActionMode::Repeat { interval: Duration::from_millis(250) }, keyboard: Some(KeyCode::Digit2), gamepad: &[] },
    HotbarSlot39 { id: "hotbar_slot_39", label: "Slot 39", mode: GameActionMode::Repeat { interval: Duration::from_millis(250) }, keyboard: Some(KeyCode::Digit3), gamepad: &[] },
    HotbarSlot40 { id: "hotbar_slot_40", label: "Slot 40", mode: GameActionMode::Repeat { interval: Duration::from_millis(250) }, keyboard: Some(KeyCode::Digit4), gamepad: &[] },
    HotbarSlot41 { id: "hotbar_slot_41", label: "Slot 41", mode: GameActionMode::Repeat { interval: Duration::from_millis(250) }, keyboard: Some(KeyCode::Digit5), gamepad: &[] },
    HotbarSlot42 { id: "hotbar_slot_42", label: "Slot 42", mode: GameActionMode::Repeat { interval: Duration::from_millis(250) }, keyboard: Some(KeyCode::Digit6), gamepad: &[] },
    HotbarSlot43 { id: "hotbar_slot_43", label: "Slot 43", mode: GameActionMode::Repeat { interval: Duration::from_millis(250) }, keyboard: Some(KeyCode::Digit7), gamepad: &[] },
    HotbarSlot44 { id: "hotbar_slot_44", label: "Slot 44", mode: GameActionMode::Repeat { interval: Duration::from_millis(250) }, keyboard: Some(KeyCode::Digit8), gamepad: &[] },
    HotbarSlot45 { id: "hotbar_slot_45", label: "Slot 45", mode: GameActionMode::Repeat { interval: Duration::from_millis(250) }, keyboard: Some(KeyCode::Digit9), gamepad: &[] },
    HotbarSlot46 { id: "hotbar_slot_46", label: "Slot 46", mode: GameActionMode::Repeat { interval: Duration::from_millis(250) }, keyboard: Some(KeyCode::Digit0), gamepad: &[] },
    HotbarSlot47 { id: "hotbar_slot_47", label: "Slot 47", mode: GameActionMode::Repeat { interval: Duration::from_millis(250) }, keyboard: Some(KeyCode::Minus), gamepad: &[] },
    HotbarSlot48 { id: "hotbar_slot_48", label: "Slot 48", mode: GameActionMode::Repeat { interval: Duration::from_millis(250) }, keyboard: Some(KeyCode::Equal), gamepad: &[] },
    SwitchToInventory { id: "switch_to_inventory", label: "Inventory Panel", mode: GameActionMode::Edge, keyboard: Some(KeyCode::KeyA), gamepad: &[] },
    SwitchToSkills { id: "switch_to_skills", label: "Skills Panel", mode: GameActionMode::Edge, keyboard: Some(KeyCode::KeyS), gamepad: &[] },
    SwitchToSpells { id: "switch_to_spells", label: "Spells Panel", mode: GameActionMode::Edge, keyboard: Some(KeyCode::KeyD), gamepad: &[] },
    SwitchToHotbar1 { id: "switch_to_hotbar_1", label: "Hotbar 1", mode: GameActionMode::Edge, keyboard: Some(KeyCode::KeyF), gamepad: &[] },
    SwitchToHotbar2 { id: "switch_to_hotbar_2", label: "Hotbar 2", mode: GameActionMode::Edge, keyboard: Some(KeyCode::KeyG), gamepad: &[] },
    SwitchToHotbar3 { id: "switch_to_hotbar_3", label: "Hotbar 3", mode: GameActionMode::Edge, keyboard: Some(KeyCode::KeyH), gamepad: &[] },
}
