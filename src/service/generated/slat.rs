// this file is auto-generated by build.rs

use serde::ser::{Serialize, SerializeStruct, Serializer};

use crate::{
    service::HapService,
    characteristic::{
        HapCharacteristic,
		slat_type::SlatTypeCharacteristic,
		current_slat_state::CurrentSlatStateCharacteristic,
		name::NameCharacteristic,
		current_tilt_angle::CurrentTiltAngleCharacteristic,
		target_tilt_angle::TargetTiltAngleCharacteristic,
		swing_mode::SwingModeCharacteristic,
	},
    HapType,
};

/// Slat Service.
#[derive(Debug, Default)]
pub struct SlatService {
    /// ID of the Slat Service.
    id: u64,
    /// `HapType` of the Slat Service.
    hap_type: HapType,
    /// Specifies if the Service is hidden.
    hidden: bool,
    /// Specifies if the Service is the primary Service of the Accessory.
    primary: bool,

	/// Slat Type Characteristic (required).
	pub slat_type: SlatTypeCharacteristic,
	/// Current Slat State Characteristic (required).
	pub current_slat_state: CurrentSlatStateCharacteristic,

	/// Name Characteristic (optional).
	pub name: Option<NameCharacteristic>,
	/// Current Tilt Angle Characteristic (optional).
	pub current_tilt_angle: Option<CurrentTiltAngleCharacteristic>,
	/// Target Tilt Angle Characteristic (optional).
	pub target_tilt_angle: Option<TargetTiltAngleCharacteristic>,
	/// Swing Mode Characteristic (optional).
	pub swing_mode: Option<SwingModeCharacteristic>,
}

impl SlatService {
    /// Creates a new Slat Service.
    pub fn new(id: u64, accessory_id: u64) -> Self {
        Self {
            id,
            hap_type: HapType::Slat,
			slat_type: SlatTypeCharacteristic::new(id + 1 + 0, accessory_id),
			current_slat_state: CurrentSlatStateCharacteristic::new(id + 1 + 1, accessory_id),
			..Default::default()
        }
    }
}

impl HapService for SlatService {
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
			&self.slat_type,
			&self.current_slat_state,
		];
		if let Some(c) = &self.name {
		    characteristics.push(c);
		}
		if let Some(c) = &self.current_tilt_angle {
		    characteristics.push(c);
		}
		if let Some(c) = &self.target_tilt_angle {
		    characteristics.push(c);
		}
		if let Some(c) = &self.swing_mode {
		    characteristics.push(c);
		}
		characteristics
    }

    fn get_mut_characteristics(&mut self) -> Vec<&mut dyn HapCharacteristic> {
        let mut characteristics: Vec<&mut dyn HapCharacteristic> = vec![
			&mut self.slat_type,
			&mut self.current_slat_state,
		];
		if let Some(c) = &mut self.name {
		    characteristics.push(c);
		}
		if let Some(c) = &mut self.current_tilt_angle {
		    characteristics.push(c);
		}
		if let Some(c) = &mut self.target_tilt_angle {
		    characteristics.push(c);
		}
		if let Some(c) = &mut self.swing_mode {
		    characteristics.push(c);
		}
		characteristics
    }
}

impl Serialize for SlatService {
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
