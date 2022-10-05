use std::rc::Rc;

use controller::input_bindings::KeyBindings;
use controller::input_bindings::ButtonBindings;
use controller::action_handler::HandlingSettings;
use serde::Serialize;
use serde::Deserialize;

use yewdux::prelude::*;

#[derive(Default, Store, Serialize, Deserialize)]
#[store(storage = "local", storage_tab_sync)]
pub struct UserPrefs {
    pub key_bindings: KeyBindings,
    pub button_bindings: ButtonBindings,
    pub handling_settings: HandlingSettings,
    nonce: u32,
}

impl PartialEq for UserPrefs {
    fn eq(&self, prefs: &Self) -> bool {
        self.nonce == prefs.nonce
    }
}

impl UserPrefs {
	pub fn get() -> Rc<Self> {
		Dispatch::<Self>::new().get()
	}

    pub fn set(key_bindings: KeyBindings,
            button_bindings: ButtonBindings,
            handling_settings: HandlingSettings) {
        let dispatch = Dispatch::<Self>::new();
        let prefs = Self {
            key_bindings: key_bindings,
            button_bindings: button_bindings,
            handling_settings: handling_settings,
            nonce: dispatch.get().nonce + 1,
        };
        dispatch.set(prefs);
    }
}