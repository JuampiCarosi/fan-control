use std::{
    fmt::{Debug, Display},
    fs::{self, ReadDir},
    path::Path,
};

use regex::Regex;

use crate::custom_error::CustomError;

const FANS_BASE_PATH: &str = "/sys/devices/platform/applesmc.768";

/// Represents all possible fan variants.
/// The unit of the enum is the fan number.
pub enum FanVariant {
    Exhaust(u8),
    Master(u8),
    Hdd(u8),
    Cpu(u8),
    Odd(u8),
}

impl Display for FanVariant {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Exhaust(number) => write!(f, "Exhaust fan {}", number),
            Self::Master(number) => write!(f, "Master fan {}", number),
            Self::Hdd(number) => write!(f, "HDD fan {}", number),
            Self::Cpu(number) => write!(f, "CPU fan {}", number),
            Self::Odd(number) => write!(f, "ODD fan {}", number),
        }
    }
}

impl FanVariant {
    fn from_path(path: &Path) -> Result<Self, CustomError> {
        let filename = get_name(path)?;
        let number = Self::extract_number_from_filename(&filename);
        let label = read_to_string(path)?;

        match label.trim().to_lowercase().as_str() {
            "exhaust" => Ok(Self::Exhaust(number)),
            "master" => Ok(Self::Master(number)),
            "hdd" => Ok(Self::Hdd(number)),
            "cpu" => Ok(Self::Cpu(number)),
            "odd" => Ok(Self::Odd(number)),
            _ => Err(CustomError {
                display_message: format!("Error: {:?} is not a valid fan label", label),
                internal_message: format!(
                    "Error: {:?} is not a valid fan label from path {:?}",
                    label, path
                ),
                cause: None,
                is_fatal: false,
            }),
        }
    }

    pub fn is_valid_label(string: &str) -> Result<bool, CustomError> {
        let available_fans = Self::get_available_fans()?;
       Ok( available_fans
            .iter()
            .any(|fan| fan.get_label().to_lowercase() == string.to_lowercase()))
    }

   pub fn from_str(string: &str) -> Result<Self, CustomError> {

    let available_fans = Self::get_available_fans()?;

    let fan = available_fans.into_iter().find(|fan| fan.get_label() == string);

    match fan {
        Some(fan) => Ok(fan),
        None => Err(CustomError {
            display_message: format!("Error: {:?} is not a valid fan label", string),
            internal_message: format!(
                "Error: {:?} is not a valid fan label from string {:?}",
                string, string
            ),
            cause: None,
            is_fatal: false,
        }),
    }}

    pub fn get_available_fans() -> Result<Vec<Self>, CustomError> {
        let dir = match read_dir(FANS_BASE_PATH) {
            Ok(dir) => dir,
            Err(err) => {
                return Err(CustomError {
                    display_message: "Error reading fans file directory".to_string(),
                    internal_message: format!(
                        "Error: failed to read {:?} directory in get_available fn\n Original message: {:?}",
                        FANS_BASE_PATH, 
                        err.internal_message
                    ),
                    cause: err.cause,
                    is_fatal: true,
                })
            }
        };

        let mut available_fans = vec![];

        for entry in dir {
            let entry = entry.map_err(|e| CustomError {
                display_message: format!("Error reading {:?} directory", FANS_BASE_PATH),
                internal_message: format!("Error unwrapping entry from {:?} ", FANS_BASE_PATH),
                cause: Some(Box::new(e)),
                is_fatal: true,
            })?;
            let path = entry.path();
            let filename = get_name(&path)?;

            if !Self::is_string_fan_label(&filename)? {
                continue;
            }

            let variant = Self::from_path(&path)?;
            available_fans.push(variant);
        }

        Ok(available_fans)
    }

    fn is_string_fan_label(string: &str) -> Result<bool, CustomError> {
        let pattern = r"^fan\d+_label$";
        let regex = Regex::new(pattern).map_err(|e| CustomError {
            display_message: "Error validating fan files".to_string(),
            internal_message: format!(
                "Error creating regex from pattern: {} on string {}",
                pattern, string
            ),
            cause: Some(Box::new(e)),
            is_fatal: false,
        })?;
        Ok(regex.is_match(string))
    }

    fn extract_number_from_filename(string: &str) -> u8 {
        let pattern = r"\d+";
        let regex = Regex::new(pattern).unwrap();
        let number = regex.find(string).unwrap().as_str();
        number.parse::<u8>().unwrap()
    }

    pub fn get_label(&self) -> String {
        match self {
            Self::Exhaust(_) => "exhaust".to_string(),
            Self::Master(_) => "master".to_string(),
            Self::Hdd(_) => "hdd".to_string(),
            Self::Cpu(_) => "cpu".to_string(),
            Self::Odd(_) => "odd".to_string(),
        }
    }

    pub fn get_fan_number(&self) -> u8 {
        match self {
            Self::Exhaust(number) => *number,
            Self::Master(number) => *number,
            Self::Hdd(number) => *number,
            Self::Cpu(number) => *number,
            Self::Odd(number) => *number,
        }
    }

    pub fn set_manual_mode(&self) -> Result<(), CustomError> {
        let maual_file = format!("{FANS_BASE_PATH}/fan{}_manual", self.get_fan_number());
        fs::write(&maual_file, "1").map_err(|e| CustomError {
            display_message: format!("Error setting manual mode for {}", self),
            internal_message: format!(
                "Error writing 1 to {:?} file in set_manual_mode fn to fan {}",
                maual_file, self
            ),
            cause: Some(Box::new(e)),
            is_fatal: true,
        })
    }
    pub fn set_auto_mode(&self) -> Result<(), CustomError> {
        let maual_file = format!("{FANS_BASE_PATH}/fan{}_manual", self.get_fan_number());
        fs::write(&maual_file, "0").map_err(|e| CustomError {
            display_message: format!("Error setting auto mode for {}", self),
            internal_message: format!(
                "Error writing 1 to {:?} file in set_auto_mode fn to fan {}",
                maual_file, self
            ),
            cause: Some(Box::new(e)),
            is_fatal: true,
        })
    }

    pub fn get_max_speed(&self) -> Result<u16, CustomError> {
        let max_speed_file = format!("{FANS_BASE_PATH}/fan{}_max", self.get_fan_number());
        let max_speed_string = read_to_string(&max_speed_file)?;
        let max_speed = max_speed_string.trim().parse::<u16>().map_err(|e| CustomError {
            display_message: format!("Error reading max speed for {}", self),
            internal_message: format!(
                "Error parsing max speed from {:?} file in get_max_speed fn to fan {}, got {:?}",
                max_speed_file, self, max_speed_string
            ),
            cause: Some(Box::new(e)),
            is_fatal: true,
        })?;
        Ok(max_speed)
    }

    pub fn get_min_speed(&self) -> Result<u16, CustomError> {
        let min_speed_file = format!("{FANS_BASE_PATH}/fan{}_min", self.get_fan_number());
        let min_speed_string = read_to_string(&min_speed_file)?;
        let min_speed = min_speed_string.trim().parse::<u16>().map_err(|e| CustomError {
            display_message: format!("Error reading min speed for {}", self),
            internal_message: format!(
                "Error parsing min speed from {:?} file in get_min_speed fn to fan {}, got {:?}",
                min_speed_file, self, min_speed_string
            ),
            cause: Some(Box::new(e)),
            is_fatal: true,
        })?;
        Ok(min_speed)
    }

    pub fn set_speed(&self, speed: u16) -> Result<(), CustomError> {
        let speed_file = format!("{FANS_BASE_PATH}/fan{}_output", self.get_fan_number());
        fs::write(&speed_file, speed.to_string()).map_err(|e| CustomError {
            display_message: format!("Error setting speed for {}", self),
            internal_message: format!(
                "Error writing {} to {:?} file in set_speed fn to fan {}",
                speed, speed_file, self
            ),
            cause: Some(Box::new(e)),
            is_fatal: true,
        })
    }
}

pub fn read_dir<P>(directory: &P) -> Result<ReadDir, CustomError>
where
    P: AsRef<Path> + Debug + ?Sized,
{
    let metadada_dir = fs::metadata(directory).map_err(|err| CustomError {
        display_message: format!("Error: the directory {:?} doesn't exist", directory),
        internal_message: format!("Error reading {:?} metadata in read_dir fn", directory),
        cause: Some(Box::new(err)),
        is_fatal: false,
    })?;

    if !metadada_dir.is_dir() {
        return Err(CustomError {
            display_message: format!("Error: {:?} is not a diretory ", directory),
            internal_message: format!("Error: {:?} is not a diretory ", directory),
            cause: None,
            is_fatal: false,
        });
    }

    fs::read_dir(directory).map_err(|e| CustomError {
        display_message: format!("Error reading {:?} directory", directory),
        internal_message: format!("Error fs::read_dir {:?} in read_dir fn", directory),
        cause: Some(Box::new(e)),
        is_fatal: false,
    })
}

pub fn get_name(directory: &Path) -> Result<String, CustomError> {
    let directorio_split = directory
        .file_name()
        .ok_or(CustomError::new_simple(&format!("Error: failed to read {:?} directory ", directory)))?
        .to_str()
        .ok_or(CustomError::new_simple(&format!("Error: failed to read {:?} directory ", directory)))?;

    Ok(directorio_split.to_string())
}

pub fn read_to_string<P>(path: P) -> Result<String, CustomError>
where
    P: AsRef<Path>,
{
    match fs::read_to_string(&path) {
        Ok(contenido) => Ok(contenido),
        Err(err) => Err(CustomError {
            display_message: format!("Error reading {:?} file", path.as_ref()),
            internal_message: format!(
                "Error reading {:?} file on fn read_to_string",
                path.as_ref()
            ),
            cause: Some(Box::new(err)),
            is_fatal: false,
        }),
    }
}
