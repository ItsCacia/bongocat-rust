use std::collections::HashMap;
use rdev::Key;

pub fn get_hm() -> HashMap<Key, &'static str>{
    let mut hm = HashMap::new();
    hm.insert(Key::Num1, "forward");
    hm.insert(Key::Num2, "forward");
    hm.insert(Key::Num3, "forward");
    hm.insert(Key::Num4, "forward");
    hm.insert(Key::KeyW, "forward");

    hm.insert(Key::KeyS, "back");
    hm.insert(Key::KeyZ, "back");
    hm.insert(Key::KeyX, "back");
    hm.insert(Key::MetaLeft, "back");
    hm.insert(Key::Alt, "back");
    hm.insert(Key::Space, "back");

    hm.insert(Key::Escape, "left");
    hm.insert(Key::BackQuote, "left");
    hm.insert(Key::Tab, "left");
    hm.insert(Key::ShiftLeft, "left");
    hm.insert(Key::ControlLeft, "left");
    hm.insert(Key::KeyQ, "left");
    hm.insert(Key::KeyA, "left");

    hm.insert(Key::Num5, "right");
    hm.insert(Key::Num6, "right");
    hm.insert(Key::Num7, "right");
    hm.insert(Key::Num8, "right");
    hm.insert(Key::Num9, "right");
    hm.insert(Key::Num0, "right");
    hm.insert(Key::KeyE, "right");
    hm.insert(Key::KeyR, "right");
    hm.insert(Key::KeyT, "right");
    hm.insert(Key::KeyY, "right");
    hm.insert(Key::KeyU, "right");
    hm.insert(Key::KeyI, "right");
    hm.insert(Key::KeyO, "right");
    hm.insert(Key::KeyP, "right");
    hm.insert(Key::KeyD, "right");
    hm.insert(Key::KeyF, "right");
    hm.insert(Key::KeyG, "right");
    hm.insert(Key::KeyH, "right");
    hm.insert(Key::KeyJ, "right");
    hm.insert(Key::KeyK, "right");
    hm.insert(Key::KeyL, "right");
    hm.insert(Key::KeyV, "right");
    hm.insert(Key::KeyB, "right");
    hm.insert(Key::KeyN, "right");
    hm.insert(Key::KeyM, "right");

    hm.insert(Key::KeyC, "wave");

    return hm;
}
