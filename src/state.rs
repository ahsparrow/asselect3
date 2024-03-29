// Copyright 2023, Alan Sparrow
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or (at
// your option) any later version.
//
// This program is distributed in the hope that it will be useful, but
// WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU
// General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.
//
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::rc::Rc;
use yew::Reducible;

// Airspace types
#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum AirType {
    ClassA,
    ClassB,
    ClassC,
    ClassD,
    ClassE,
    ClassF,
    ClassG,
    Danger,
    Cta,
    Ctr,
    Gliding,
    Matz,
    Other,
    Prohibited,
    Restricted,
    Rmz,
    Tmz,
}

// Output format
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum Format {
    OpenAir,
    RatOnly,
    Competition,
}

// Altutude layer overlay
#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum Overlay {
    FL195,
    FL105,
    AtzDz,
}

// Settings
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Settings {
    pub atz: AirType,
    pub ils: Option<AirType>,
    pub unlicensed: Option<AirType>,
    pub microlight: Option<AirType>,
    pub gliding: Option<AirType>,
    pub home: Option<String>,
    pub hirta_gvs: Option<AirType>,
    pub obstacle: Option<AirType>,
    pub max_level: u16,
    pub radio: bool,
    pub format: Format,
    pub overlay: Option<Overlay>,
    #[serde(default)]
    pub loa: HashSet<String>,
    #[serde(default)]
    pub rat: HashSet<String>,
    #[serde(default)]
    pub wave: HashSet<String>,
}

impl Default for Settings {
    fn default() -> Self {
        Settings {
            atz: AirType::Ctr,
            ils: None,
            unlicensed: None,
            microlight: None,
            gliding: None,
            home: None,
            hirta_gvs: None,
            obstacle: None,
            max_level: 660,
            radio: false,
            format: Format::OpenAir,
            overlay: None,
            loa: HashSet::new(),
            rat: HashSet::new(),
            wave: HashSet::new(),
        }
    }
}

// Application state
#[derive(Debug, Default, PartialEq)]
pub struct State {
    pub settings: Settings,
}

// State actions
pub enum Action {
    Set { name: String, value: String },
    SetLoa { name: String, checked: bool },
    SetRat { name: String, checked: bool },
    SetWave { name: String, checked: bool },
    ClearLoa,
    ClearRat,
    ClearWave,
}

impl Reducible for State {
    type Action = Action;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        let mut set = self.settings.clone();
        match action {
            // Set airspace option
            Action::Set { name, value } => {
                match name.as_str() {
                    "atz" => set.atz = get_airtype(&value).unwrap_or(AirType::Ctr),
                    "ils" => set.ils = get_airtype(&value),
                    "unlicensed" => set.unlicensed = get_airtype(&value),
                    "microlight" => set.microlight = get_airtype(&value),
                    "gliding" => set.gliding = get_airtype(&value),
                    "hirta_gvs" => set.hirta_gvs = get_airtype(&value),
                    "obstacle" => set.obstacle = get_airtype(&value),
                    "max_level" => set.max_level = value.parse::<u16>().unwrap(),
                    "radio" => set.radio = value == "yes",
                    "home" => set.home = if value == "no" { None } else { Some(value) },
                    "overlay" => {
                        set.overlay = match value.as_str() {
                            "fl195" => Some(Overlay::FL195),
                            "fl105" => Some(Overlay::FL105),
                            "atzdz" => Some(Overlay::AtzDz),
                            _ => None,
                        }
                    }
                    "format" => {
                        set.format = match value.as_str() {
                            "ratonly" => Format::RatOnly,
                            "competition" => Format::Competition,
                            _ => Format::OpenAir,
                        }
                    }
                    _ => (),
                };
            }
            // Include/exclude LOA
            Action::SetLoa { name, checked } => {
                if checked {
                    set.loa.replace(name);
                } else {
                    set.loa.remove(&name);
                }
            }
            // Include/exclude RAT
            Action::SetRat { name, checked } => {
                if checked {
                    set.rat.replace(name);
                } else {
                    set.rat.remove(&name);
                }
            }
            // Include/exclude wave box
            Action::SetWave { name, checked } => {
                if checked {
                    set.wave.replace(name);
                } else {
                    set.wave.remove(&name);
                }
            }
            // Clear all LOAs
            Action::ClearLoa => set.loa.clear(),
            // Clear all RATs
            Action::ClearRat => set.rat.clear(),
            // Clear all Wave boxes
            Action::ClearWave => set.wave.clear(),
        }
        Self { settings: set }.into()
    }
}

// Default mapping value to airspace type
fn get_airtype(value: &str) -> Option<AirType> {
    match value {
        "classd" => Some(AirType::ClassD),
        "classf" => Some(AirType::ClassF),
        "classg" => Some(AirType::ClassG),
        "ctr" => Some(AirType::Ctr),
        "danger" => Some(AirType::Danger),
        "restricted" => Some(AirType::Restricted),
        "gsec" => Some(AirType::Gliding),
        _ => None,
    }
}
