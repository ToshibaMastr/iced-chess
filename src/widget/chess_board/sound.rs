use std::collections::HashMap;
use std::io::Cursor;
use std::sync::{Arc, Mutex};

use kira::{AudioManager, sound::static_sound::StaticSoundData};

use crate::assets::Assets;

#[derive(Hash, Eq, PartialEq, Debug)]
pub enum SoundType {
    Castle,
    GameEnd,
    GameStart,
    Illegal,
    Notify,
    Premove,
    TenSeconds,
    Capture,
    Promote,
    MoveCheck,
    MoveSelf,
    MoveOpponent,
}

struct Sounds(HashMap<SoundType, StaticSoundData>);

impl Sounds {
    fn new(base_path: &str) -> Self {
        let mut map = HashMap::new();
        let files = [
            (SoundType::GameEnd, "game-end.ogg"),
            (SoundType::GameStart, "game-start.ogg"),
            (SoundType::Notify, "notify.ogg"),
            (SoundType::Premove, "premove.ogg"),
            (SoundType::TenSeconds, "tenseconds.ogg"),
            (SoundType::Illegal, "illegal.ogg"),
            (SoundType::Castle, "castle.ogg"),
            (SoundType::Promote, "promote.ogg"),
            (SoundType::Capture, "capture.ogg"),
            (SoundType::MoveSelf, "move-self.ogg"),
            (SoundType::MoveCheck, "move-check.ogg"),
            (SoundType::MoveOpponent, "move-opponent.ogg"),
        ];

        for (kind, file) in files {
            let filename = format!("{base_path}/{file}");
            let data = Assets::get(&filename).unwrap();
            let sound = StaticSoundData::from_cursor(Cursor::new(data.data.into_owned())).unwrap();
            map.insert(kind, sound);
        }

        Self(map)
    }

    fn get(&self, kind: &SoundType) -> StaticSoundData {
        self.0.get(kind).unwrap().clone()
    }
}

#[derive(Clone)]
pub struct ChessBoardSound {
    manager: Arc<Mutex<AudioManager>>,
    sounds: Arc<Sounds>,
}

impl ChessBoardSound {
    pub fn new() -> Self {
        let manager = AudioManager::new(Default::default()).unwrap();
        let sounds = Sounds::new("sounds");

        Self {
            manager: Arc::new(Mutex::new(manager)),
            sounds: Arc::new(sounds),
        }
    }

    pub fn play(&self, kind: SoundType) {
        if let Ok(mut manager) = self.manager.lock() {
            manager.play(self.sounds.get(&kind)).unwrap();
        }
    }

    pub fn move_self(&self) {
        self.play(SoundType::MoveSelf);
    }
}
