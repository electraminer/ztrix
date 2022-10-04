use std::rc::Rc;

use controller::input_bindings::InputBindings;
use controller::action_handler::HandlingSettings;
use serde::Serialize;
use serde::Deserialize;

use yewdux::prelude::*;

#[derive(Default, Store, Serialize, Deserialize)]
#[store(storage = "local", storage_tab_sync)]
pub struct UserPrefs {
	input_bindings: InputBindings,
    handling_settings: HandlingSettings,
    nonce: u32,
}

impl PartialEq for UserPrefs {
    fn eq(&self, prefs: &UserPrefs) -> bool {
        self.nonce == prefs.nonce
    }
}

impl UserPrefs {
	pub fn get() -> Rc<UserPrefs> {
		Dispatch::<UserPrefs>::new().get()
	}

    pub fn get_input_bindings(&self) -> &InputBindings {
        &self.input_bindings
    }

    pub fn get_handling_settings(&self) -> &HandlingSettings {
        &self.handling_settings
    }
}