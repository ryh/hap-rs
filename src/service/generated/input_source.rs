// this file is auto-generated by build.rs

use serde::ser::{Serialize, SerializeStruct, Serializer};

use crate::{
    service::HapService,
    characteristic::{
        HapCharacteristic,
		configured_name::ConfiguredNameCharacteristic,
		input_source_type::InputSourceTypeCharacteristic,
		is_configured::IsConfiguredCharacteristic,
		current_visibility_state::CurrentVisibilityStateCharacteristic,
		identifier::IdentifierCharacteristic,
		input_device_type::InputDeviceTypeCharacteristic,
		target_visibility_state::TargetVisibilityStateCharacteristic,
		name::NameCharacteristic,
	},
    HapType,
};

/// Input Source Service.
#[derive(Debug, Default)]
pub struct InputSourceService {
    /// ID of the Input Source Service.
    id: u64,
    /// `HapType` of the Input Source Service.
    hap_type: HapType,
    /// Specifies if the Service is hidden.
    hidden: bool,
    /// Specifies if the Service is the primary Service of the Accessory.
    primary: bool,

	/// Configured Name Characteristic (required).
	pub configured_name: ConfiguredNameCharacteristic,
	/// Input Source Type Characteristic (required).
	pub input_source_type: InputSourceTypeCharacteristic,
	/// Is Configured Characteristic (required).
	pub is_configured: IsConfiguredCharacteristic,
	/// Current Visibility State Characteristic (required).
	pub current_visibility_state: CurrentVisibilityStateCharacteristic,

	/// Identifier Characteristic (optional).
	pub identifier: Option<IdentifierCharacteristic>,
	/// Input Device Type Characteristic (optional).
	pub input_device_type: Option<InputDeviceTypeCharacteristic>,
	/// Target Visibility State Characteristic (optional).
	pub target_visibility_state: Option<TargetVisibilityStateCharacteristic>,
	/// Name Characteristic (optional).
	pub name: Option<NameCharacteristic>,
}

impl InputSourceService {
    /// Creates a new Input Source Service.
    pub fn new(id: u64, accessory_id: u64) -> Self {
        Self {
            id,
            hap_type: HapType::InputSource,
			configured_name: ConfiguredNameCharacteristic::new(id + 1 + 0, accessory_id),
			input_source_type: InputSourceTypeCharacteristic::new(id + 1 + 1, accessory_id),
			is_configured: IsConfiguredCharacteristic::new(id + 1 + 2, accessory_id),
			current_visibility_state: CurrentVisibilityStateCharacteristic::new(id + 1 + 3, accessory_id),
			..Default::default()
        }
    }
}

impl HapService for InputSourceService {
    fn get_id(&self) -> u64 {
        self.id
    }

    fn get_type(&self) -> HapType {
        self.hap_type
    }

    fn get_hidden(&self) -> bool {
        self.hidden
    }

    fn set_hidden(&mut self, hidden: bool) {
        self.hidden = hidden;
    }

    fn get_primary(&self) -> bool {
        self.primary
    }

    fn set_primary(&mut self, primary: bool) {
        self.primary = primary;
    }

    fn get_characteristic(&self, hap_type: HapType) -> Option<&dyn HapCharacteristic> {
        for characteristic in self.get_characteristics() {
            if characteristic.get_type() == hap_type {
                return Some(characteristic);
            }
        }
        None
    }

    fn get_mut_characteristic(&mut self, hap_type: HapType) -> Option<&mut dyn HapCharacteristic> {
        for characteristic in self.get_mut_characteristics() {
            if characteristic.get_type() == hap_type {
                return Some(characteristic);
            }
        }
        None
    }

    fn get_characteristics(&self) -> Vec<&dyn HapCharacteristic> {
        let mut characteristics: Vec<&dyn HapCharacteristic> = vec![
			&self.configured_name,
			&self.input_source_type,
			&self.is_configured,
			&self.current_visibility_state,
		];
		if let Some(c) = &self.identifier {
		    characteristics.push(c);
		}
		if let Some(c) = &self.input_device_type {
		    characteristics.push(c);
		}
		if let Some(c) = &self.target_visibility_state {
		    characteristics.push(c);
		}
		if let Some(c) = &self.name {
		    characteristics.push(c);
		}
		characteristics
    }

    fn get_mut_characteristics(&mut self) -> Vec<&mut dyn HapCharacteristic> {
        let mut characteristics: Vec<&mut dyn HapCharacteristic> = vec![
			&mut self.configured_name,
			&mut self.input_source_type,
			&mut self.is_configured,
			&mut self.current_visibility_state,
		];
		if let Some(c) = &mut self.identifier {
		    characteristics.push(c);
		}
		if let Some(c) = &mut self.input_device_type {
		    characteristics.push(c);
		}
		if let Some(c) = &mut self.target_visibility_state {
		    characteristics.push(c);
		}
		if let Some(c) = &mut self.name {
		    characteristics.push(c);
		}
		characteristics
    }
}

impl Serialize for InputSourceService {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut state = serializer.serialize_struct("HapService", 5)?;
        state.serialize_field("iid", &self.get_id())?;
        state.serialize_field("type", &self.get_type())?;
        state.serialize_field("hidden", &self.get_hidden())?;
        state.serialize_field("primary", &self.get_primary())?;
        state.serialize_field("characteristics", &self.get_characteristics())?;
        // linked services left out for now
        state.end()
    }
}
