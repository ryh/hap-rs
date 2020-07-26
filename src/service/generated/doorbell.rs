// this file is auto-generated by build.rs

use serde::ser::{Serialize, SerializeStruct, Serializer};

use crate::{
    service::HapService,
    characteristic::{
        HapCharacteristic,
		programmable_switch_event::ProgrammableSwitchEventCharacteristic,
		brightness::BrightnessCharacteristic,
		volume::VolumeCharacteristic,
		name::NameCharacteristic,
	},
    HapType,
};

/// Doorbell Service.
#[derive(Debug, Default)]
pub struct DoorbellService {
    /// ID of the Doorbell Service.
    id: u64,
    /// `HapType` of the Doorbell Service.
    hap_type: HapType,
    /// Specifies if the Service is hidden.
    hidden: bool,
    /// Specifies if the Service is the primary Service of the Accessory.
    primary: bool,

	/// Programmable Switch Event Characteristic (required).
	pub programmable_switch_event: ProgrammableSwitchEventCharacteristic,

	/// Brightness Characteristic (optional).
	pub brightness: Option<BrightnessCharacteristic>,
	/// Volume Characteristic (optional).
	pub volume: Option<VolumeCharacteristic>,
	/// Name Characteristic (optional).
	pub name: Option<NameCharacteristic>,
}

impl DoorbellService {
    /// Creates a new Doorbell Service.
    pub fn new(id: u64, accessory_id: u64) -> Self {
        Self {
            id,
            hap_type: HapType::Doorbell,
			programmable_switch_event: ProgrammableSwitchEventCharacteristic::new(id + 1 + 0, accessory_id),
			..Default::default()
        }
    }
}

impl HapService for DoorbellService {
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
			&self.programmable_switch_event,
		];
		if let Some(c) = &self.brightness {
		    characteristics.push(c);
		}
		if let Some(c) = &self.volume {
		    characteristics.push(c);
		}
		if let Some(c) = &self.name {
		    characteristics.push(c);
		}
		characteristics
    }

    fn get_mut_characteristics(&mut self) -> Vec<&mut dyn HapCharacteristic> {
        let mut characteristics: Vec<&mut dyn HapCharacteristic> = vec![
			&mut self.programmable_switch_event,
		];
		if let Some(c) = &mut self.brightness {
		    characteristics.push(c);
		}
		if let Some(c) = &mut self.volume {
		    characteristics.push(c);
		}
		if let Some(c) = &mut self.name {
		    characteristics.push(c);
		}
		characteristics
    }
}

impl Serialize for DoorbellService {
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
