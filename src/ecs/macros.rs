use bevy::prelude::*;
use crossbeam_channel::{Receiver, Sender, unbounded};
use rhai::{Engine, Scope};

use packets::{client::Emote, types::BodyAnimationKind};

pub struct MacrosPlugin;

impl Plugin for MacrosPlugin {
    fn build(&self, app: &mut App) {
        let (tx, rx) = unbounded();
        app.insert_resource(MacroManager::new(tx));
        app.insert_resource(MacroEventReceiver(rx));
        app.add_systems(Update, handle_macro_events);
    }
}

pub enum MacroEvent {
    Emote(BodyAnimationKind),
}

#[derive(Resource)]
pub struct MacroEventReceiver(pub Receiver<MacroEvent>);

#[derive(Resource)]
pub struct MacroManager {
    engine: Engine,
}

impl MacroManager {
    pub fn new(tx: Sender<MacroEvent>) -> Self {
        let mut engine = Engine::new();

        // Disable loops as requested
        engine.disable_symbol("while");
        engine.disable_symbol("loop");

        // Register the `emote` function
        let tx_clone = tx.clone();
        engine.register_fn("emote", move |name: &str| {
            if let Some(anim) = get_animation_by_name(name) {
                let _ = tx_clone.send(MacroEvent::Emote(anim));
            } else {
                tracing::warn!("Unknown emote in macro: {}", name);
            }
        });

        Self { engine }
    }

    pub fn execute(&self, script: &str) {
        let mut scope = Scope::new();
        if let Err(e) = self.engine.eval_with_scope::<()>(&mut scope, script) {
            tracing::error!("Macro execution failed: {}", e);
        }
    }
}

fn handle_macro_events(rx: Res<MacroEventReceiver>, outbox: Res<crate::network::PacketOutbox>) {
    while let Ok(event) = rx.0.try_recv() {
        match event {
            MacroEvent::Emote(animation) => {
                outbox.send(&Emote { animation });
            }
        }
    }
}

pub fn get_animation_by_name(name: &str) -> Option<BodyAnimationKind> {
    match name.to_lowercase().replace(" ", "").as_str() {
        "smile" => Some(BodyAnimationKind::Smile),
        "cry" => Some(BodyAnimationKind::Cry),
        "frown" => Some(BodyAnimationKind::Frown),
        "wink" => Some(BodyAnimationKind::Wink),
        "surprise" => Some(BodyAnimationKind::Surprise),
        "tongue" => Some(BodyAnimationKind::Tongue),
        "pleasant" => Some(BodyAnimationKind::Pleasant),
        "snore" => Some(BodyAnimationKind::Snore),
        "mouth" => Some(BodyAnimationKind::Mouth),
        "blowkiss" => Some(BodyAnimationKind::BlowKiss),
        "wave" => Some(BodyAnimationKind::Wave),
        "rockon" => Some(BodyAnimationKind::RockOn),
        "peace" => Some(BodyAnimationKind::Peace),
        "stop" => Some(BodyAnimationKind::Stop),
        "ouch" => Some(BodyAnimationKind::Ouch),
        "impatient" => Some(BodyAnimationKind::Impatient),
        "shock" => Some(BodyAnimationKind::Shock),
        "pleasure" => Some(BodyAnimationKind::Pleasure),
        "love" => Some(BodyAnimationKind::Love),
        "sweatdrop" => Some(BodyAnimationKind::SweatDrop),
        "whistle" => Some(BodyAnimationKind::Whistle),
        "irritation" => Some(BodyAnimationKind::Irritation),
        "silly" => Some(BodyAnimationKind::Silly),
        "cute" => Some(BodyAnimationKind::Cute),
        "yelling" => Some(BodyAnimationKind::Yelling),
        "mischievous" => Some(BodyAnimationKind::Mischievous),
        "evil" => Some(BodyAnimationKind::Evil),
        "horror" => Some(BodyAnimationKind::Horror),
        "puppydog" => Some(BodyAnimationKind::PuppyDog),
        "stonefaced" => Some(BodyAnimationKind::StoneFaced),
        "tears" => Some(BodyAnimationKind::Tears),
        "firedup" => Some(BodyAnimationKind::FiredUp),
        "confused" => Some(BodyAnimationKind::Confused),
        _ => None,
    }
}

pub fn get_animation_name_by_id(id: BodyAnimationKind) -> &'static str {
    match id {
        BodyAnimationKind::Smile => "Smile",
        BodyAnimationKind::Cry => "Cry",
        BodyAnimationKind::Frown => "Frown",
        BodyAnimationKind::Wink => "Wink",
        BodyAnimationKind::Surprise => "Surprise",
        BodyAnimationKind::Tongue => "Tongue",
        BodyAnimationKind::Pleasant => "Pleasant",
        BodyAnimationKind::Snore => "Snore",
        BodyAnimationKind::Mouth => "Mouth",
        BodyAnimationKind::BlowKiss => "BlowKiss",
        BodyAnimationKind::Wave => "Wave",
        BodyAnimationKind::RockOn => "RockOn",
        BodyAnimationKind::Peace => "Peace",
        BodyAnimationKind::Stop => "Stop",
        BodyAnimationKind::Ouch => "Ouch",
        BodyAnimationKind::Impatient => "Impatient",
        BodyAnimationKind::Shock => "Shock",
        BodyAnimationKind::Pleasure => "Pleasure",
        BodyAnimationKind::Love => "Love",
        BodyAnimationKind::SweatDrop => "SweatDrop",
        BodyAnimationKind::Whistle => "Whistle",
        BodyAnimationKind::Irritation => "Irritation",
        BodyAnimationKind::Silly => "Silly",
        BodyAnimationKind::Cute => "Cute",
        BodyAnimationKind::Yelling => "Yelling",
        BodyAnimationKind::Mischievous => "Mischievous",
        BodyAnimationKind::Evil => "Evil",
        BodyAnimationKind::Horror => "Horror",
        BodyAnimationKind::PuppyDog => "PuppyDog",
        BodyAnimationKind::StoneFaced => "StoneFaced",
        BodyAnimationKind::Tears => "Tears",
        BodyAnimationKind::FiredUp => "FiredUp",
        BodyAnimationKind::Confused => "Confused",
        _ => "",
    }
}
