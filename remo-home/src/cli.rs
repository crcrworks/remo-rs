use clap::{Parser, ValueEnum};

#[derive(Clone, Debug, Copy, PartialEq, Eq)]
pub enum Cmds {
    Me,
    GetDevices,
    Operate { device: Device, power: Power },
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Cli {
    #[arg(value_enum)]
    pub cmds: Cmds,
}

#[derive(Clone, Debug, Copy, PartialEq, Eq)]
pub enum Device {
    Ac,
    Light,
}

#[derive(Clone, Debug, ValueEnum, Copy, PartialEq, Eq)]
pub enum Power {
    On,
    Off,
}

const CMDS_NAME_TO_VARIANTS: [(&str, Cmds); 6] = [
    ("get-me", Cmds::Me),
    ("get-devices", Cmds::GetDevices),
    (
        "ac-on",
        Cmds::Operate {
            device: Device::Ac,
            power: Power::On,
        },
    ),
    (
        "ac-off",
        Cmds::Operate {
            device: Device::Ac,
            power: Power::Off,
        },
    ),
    (
        "light-on",
        Cmds::Operate {
            device: Device::Light,
            power: Power::On,
        },
    ),
    (
        "light-off",
        Cmds::Operate {
            device: Device::Light,
            power: Power::Off,
        },
    ),
];

const CMDS_VARIANTS: [Cmds; CMDS_NAME_TO_VARIANTS.len()] = [
    CMDS_NAME_TO_VARIANTS[0].1,
    CMDS_NAME_TO_VARIANTS[1].1,
    CMDS_NAME_TO_VARIANTS[2].1,
    CMDS_NAME_TO_VARIANTS[3].1,
    CMDS_NAME_TO_VARIANTS[4].1,
    CMDS_NAME_TO_VARIANTS[5].1,
];

impl ValueEnum for Cmds {
    fn value_variants<'a>() -> &'a [Self] {
        &CMDS_VARIANTS
    }

    fn to_possible_value(&self) -> Option<clap::builder::PossibleValue> {
        CMDS_NAME_TO_VARIANTS
            .iter()
            .find(|(_, level)| self == level)
            .map(|(name, _)| clap::builder::PossibleValue::new(name))
    }
}
