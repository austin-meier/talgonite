use super::{GameAction, GamepadConfig, GamepadInputType, InputBindings, KeyBinding};
use bevy::input::ButtonInput;
use bevy::input::keyboard::KeyCode;
use bevy::prelude::*;
use std::collections::HashMap;

#[derive(Resource, Default)]
pub struct ActionRepeatState {
    timers: HashMap<GameAction, Timer>,
}

impl ActionRepeatState {
    fn timer_for(
        &mut self,
        action: GameAction,
        repeat_interval: std::time::Duration,
    ) -> &mut Timer {
        self.timers.entry(action).or_insert_with(|| {
            Timer::from_seconds(repeat_interval.as_secs_f32(), TimerMode::Repeating)
        })
    }

    fn clear(&mut self, action: GameAction) {
        self.timers.remove(&action);
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InputSource {
    Keyboard(KeyBinding),
    Gamepad(GamepadInputType),
}

impl InputSource {
    pub fn label(&self) -> String {
        match self {
            InputSource::Keyboard(kb) => kb.to_dom_code(),
            InputSource::Gamepad(gi) => gi.label().to_string(),
        }
    }

    pub fn from_string(s: &str) -> Option<Self> {
        GamepadInputType::from_string(s)
            .map(InputSource::Gamepad)
            .or_else(|| KeyBinding::from_dom_code(s).map(InputSource::Keyboard))
    }
}

#[derive(Resource)]
pub struct UnifiedInputBindings {
    bindings: std::collections::HashMap<GameAction, Vec<InputSource>>,
}

impl UnifiedInputBindings {
    pub fn new() -> Self {
        Self {
            bindings: std::collections::HashMap::new(),
        }
    }

    pub fn with_defaults() -> Self {
        let mut bindings = std::collections::HashMap::new();
        let keyboard_defaults = InputBindings::new();

        for action in GameAction::all() {
            let mut sources = Vec::new();

            if let Some(key_bindings) = keyboard_defaults.get(*action) {
                sources.extend(key_bindings.iter().cloned().map(InputSource::Keyboard));
            }

            sources.extend(
                action
                    .default_gamepad_inputs()
                    .iter()
                    .copied()
                    .map(InputSource::Gamepad),
            );

            if !sources.is_empty() {
                bindings.insert(*action, sources);
            }
        }

        Self { bindings }
    }

    pub fn from_settings(settings: &game_types::KeyBindings) -> Self {
        let mut unified = Self::with_defaults();

        macro_rules! bind {
            ($field:ident, $action:ident) => {
                for (i, key_str) in settings.$field.iter().enumerate() {
                    if !key_str.is_empty() {
                        if let Some(kb) = KeyBinding::from_dom_code(key_str) {
                            unified.set_keyboard_binding(GameAction::$action, kb, i);
                        }
                    } else {
                        unified.unbind_keyboard_at(GameAction::$action, i);
                    }
                }
            };
        }

        bind!(move_up, MoveUp);
        bind!(move_down, MoveDown);
        bind!(move_left, MoveLeft);
        bind!(move_right, MoveRight);
        bind!(inventory, Inventory);
        bind!(character, Character);
        bind!(skills, Skills);
        bind!(spells, Spells);
        bind!(settings, Settings);
        bind!(refresh, Refresh);
        bind!(toggle_overview, ToggleOverview);
        bind!(basic_attack, BasicAttack);
        bind!(auto_attack_toggle, AutoAttackToggle);
        bind!(item_pickup_below, ItemPickupBelow);
        bind!(hotbar_slot_1, HotbarSlot1);
        bind!(hotbar_slot_2, HotbarSlot2);
        bind!(hotbar_slot_3, HotbarSlot3);
        bind!(hotbar_slot_4, HotbarSlot4);
        bind!(hotbar_slot_5, HotbarSlot5);
        bind!(hotbar_slot_6, HotbarSlot6);
        bind!(hotbar_slot_7, HotbarSlot7);
        bind!(hotbar_slot_8, HotbarSlot8);
        bind!(hotbar_slot_9, HotbarSlot9);
        bind!(hotbar_slot_10, HotbarSlot10);
        bind!(hotbar_slot_11, HotbarSlot11);
        bind!(hotbar_slot_12, HotbarSlot12);
        bind!(hotbar_slot_13, HotbarSlot13);
        bind!(hotbar_slot_14, HotbarSlot14);
        bind!(hotbar_slot_15, HotbarSlot15);
        bind!(hotbar_slot_16, HotbarSlot16);
        bind!(hotbar_slot_17, HotbarSlot17);
        bind!(hotbar_slot_18, HotbarSlot18);
        bind!(hotbar_slot_19, HotbarSlot19);
        bind!(hotbar_slot_20, HotbarSlot20);
        bind!(hotbar_slot_21, HotbarSlot21);
        bind!(hotbar_slot_22, HotbarSlot22);
        bind!(hotbar_slot_23, HotbarSlot23);
        bind!(hotbar_slot_24, HotbarSlot24);
        bind!(hotbar_slot_25, HotbarSlot25);
        bind!(hotbar_slot_26, HotbarSlot26);
        bind!(hotbar_slot_27, HotbarSlot27);
        bind!(hotbar_slot_28, HotbarSlot28);
        bind!(hotbar_slot_29, HotbarSlot29);
        bind!(hotbar_slot_30, HotbarSlot30);
        bind!(hotbar_slot_31, HotbarSlot31);
        bind!(hotbar_slot_32, HotbarSlot32);
        bind!(hotbar_slot_33, HotbarSlot33);
        bind!(hotbar_slot_34, HotbarSlot34);
        bind!(hotbar_slot_35, HotbarSlot35);
        bind!(hotbar_slot_36, HotbarSlot36);
        bind!(hotbar_slot_37, HotbarSlot37);
        bind!(hotbar_slot_38, HotbarSlot38);
        bind!(hotbar_slot_39, HotbarSlot39);
        bind!(hotbar_slot_40, HotbarSlot40);
        bind!(hotbar_slot_41, HotbarSlot41);
        bind!(hotbar_slot_42, HotbarSlot42);
        bind!(hotbar_slot_43, HotbarSlot43);
        bind!(hotbar_slot_44, HotbarSlot44);
        bind!(hotbar_slot_45, HotbarSlot45);
        bind!(hotbar_slot_46, HotbarSlot46);
        bind!(hotbar_slot_47, HotbarSlot47);
        bind!(hotbar_slot_48, HotbarSlot48);
        bind!(switch_to_inventory, SwitchToInventory);
        bind!(switch_to_skills, SwitchToSkills);
        bind!(switch_to_spells, SwitchToSpells);
        bind!(switch_to_hotbar_1, SwitchToHotbar1);
        bind!(switch_to_hotbar_2, SwitchToHotbar2);
        bind!(switch_to_hotbar_3, SwitchToHotbar3);

        unified
    }

    pub fn get(&self, action: GameAction) -> Option<&[InputSource]> {
        self.bindings.get(&action).map(|v| v.as_slice())
    }

    pub fn add_binding(&mut self, action: GameAction, source: InputSource) {
        self.bindings.entry(action).or_default().push(source);
    }

    pub fn set_keyboard_binding(&mut self, action: GameAction, binding: KeyBinding, index: usize) {
        let sources = self.bindings.entry(action).or_default();

        let mut current_kb_count = 0;
        let mut target_idx = None;

        for (i, s) in sources.iter().enumerate() {
            if matches!(s, InputSource::Keyboard(_)) {
                if current_kb_count == index {
                    target_idx = Some(i);
                    break;
                }
                current_kb_count += 1;
            }
        }

        if let Some(i) = target_idx {
            sources[i] = InputSource::Keyboard(binding);
        } else {
            sources.push(InputSource::Keyboard(binding));
        }
    }

    pub fn unbind_keyboard_at(&mut self, action: GameAction, index: usize) {
        if let Some(sources) = self.bindings.get_mut(&action) {
            let mut current_kb_count = 0;
            let mut target_idx = None;

            for (i, s) in sources.iter().enumerate() {
                if matches!(s, InputSource::Keyboard(_)) {
                    if current_kb_count == index {
                        target_idx = Some(i);
                        break;
                    }
                    current_kb_count += 1;
                }
            }

            if let Some(i) = target_idx {
                sources.remove(i);
            }
        }
    }

    pub fn set_gamepad_binding(&mut self, action: GameAction, binding: GamepadInputType) {
        let sources = self.bindings.entry(action).or_default();
        sources.retain(|s| !matches!(s, InputSource::Gamepad(_)));
        sources.push(InputSource::Gamepad(binding));
    }

    pub fn set_binding(&mut self, action: GameAction, source: InputSource, index: usize) {
        match source {
            InputSource::Keyboard(kb) => self.set_keyboard_binding(action, kb, index),
            InputSource::Gamepad(gp) => self.set_gamepad_binding(action, gp),
        }
    }

    pub fn is_pressed(
        &self,
        action: GameAction,
        keyboard: &ButtonInput<KeyCode>,
        gamepad_query: Option<&Query<&Gamepad>>,
        gamepad_config: Option<&GamepadConfig>,
    ) -> bool {
        let Some(sources) = self.bindings.get(&action) else {
            return false;
        };

        for source in sources {
            match source {
                InputSource::Keyboard(kb) => {
                    if kb.is_pressed(keyboard) {
                        return true;
                    }
                }
                InputSource::Gamepad(gi) => {
                    if let (Some(config), Some(query)) = (gamepad_config, gamepad_query) {
                        if let Some(gamepad) =
                            config.primary_gamepad.and_then(|e| query.get(e).ok())
                        {
                            if gi.is_pressed(gamepad, config.stick_threshold) {
                                return true;
                            }
                        }
                    }
                }
            }
        }

        false
    }

    pub fn is_just_pressed(
        &self,
        action: GameAction,
        keyboard: &ButtonInput<KeyCode>,
        gamepad_query: Option<&Query<&Gamepad>>,
        gamepad_config: Option<&GamepadConfig>,
    ) -> bool {
        let Some(sources) = self.bindings.get(&action) else {
            return false;
        };

        for source in sources {
            match source {
                InputSource::Keyboard(kb) => {
                    if kb.is_just_pressed(keyboard) {
                        return true;
                    }
                }
                InputSource::Gamepad(gi) => {
                    if let (Some(config), Some(query)) = (gamepad_config, gamepad_query) {
                        if let Some(gamepad) =
                            config.primary_gamepad.and_then(|e| query.get(e).ok())
                        {
                            if gi.is_just_pressed(gamepad) {
                                return true;
                            }
                        }
                    }
                }
            }
        }

        false
    }

    pub fn is_just_pressed_or_repeated(
        &self,
        action: GameAction,
        keyboard: &ButtonInput<KeyCode>,
        gamepad_query: Option<&Query<&Gamepad>>,
        gamepad_config: Option<&GamepadConfig>,
        repeat_state: &mut ActionRepeatState,
        time: &Time,
    ) -> bool {
        if self.is_just_pressed(action, keyboard, gamepad_query, gamepad_config) {
            if let Some(interval) = action.repeat_interval() {
                repeat_state.timer_for(action, interval).reset();
            }
            return true;
        }

        let Some(interval) = action.repeat_interval() else {
            repeat_state.clear(action);
            return false;
        };

        if !self.is_pressed(action, keyboard, gamepad_query, gamepad_config) {
            repeat_state.clear(action);
            return false;
        }

        repeat_state
            .timer_for(action, interval)
            .tick(time.delta())
            .just_finished()
    }

    pub fn any_pressed(
        &self,
        actions: &[GameAction],
        keyboard: &ButtonInput<KeyCode>,
        gamepad_query: Option<&Query<&Gamepad>>,
        gamepad_config: Option<&GamepadConfig>,
    ) -> bool {
        actions
            .iter()
            .any(|&action| self.is_pressed(action, keyboard, gamepad_query, gamepad_config))
    }

    pub fn any_just_pressed(
        &self,
        actions: &[GameAction],
        keyboard: &ButtonInput<KeyCode>,
        gamepad_query: Option<&Query<&Gamepad>>,
        gamepad_config: Option<&GamepadConfig>,
    ) -> bool {
        actions
            .iter()
            .any(|&action| self.is_just_pressed(action, keyboard, gamepad_query, gamepad_config))
    }
}

impl Default for UnifiedInputBindings {
    fn default() -> Self {
        Self::with_defaults()
    }
}
