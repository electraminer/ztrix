use std::rc::Rc;

use controller::input_bindings::KeyBindings;
use controller::input_bindings::ButtonBindings;
use controller::action_handler::HandlingSettings;
use serde::Serialize;
use serde::Deserialize;

use yewdux::prelude::*;

#[derive(Default, Store, Serialize, Deserialize, Clone)]
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

    pub fn set(mut user_prefs: UserPrefs) {
        let dispatch = Dispatch::<Self>::new();
        user_prefs.nonce = dispatch.get().nonce + 1;
        dispatch.set(user_prefs);
    }
}