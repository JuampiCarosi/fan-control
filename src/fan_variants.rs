use std::{
    fmt::{Debug, Display},
    fs::{self, ReadDir},
    path::Path,
    rc::Rc,
};

use regex::Regex;

use crate::custom_error::CustomError;

const FANS_BASE_PATH: &str = "/sys/devices/platform/applesmc.768";

/// Represents all possible fan variants.
/// The unit of the enum is the fan number.
pub enum FanVariants {
    Exhaust(u8),
    Master(u8),
    Hdd(u8),
    Cpu(u8),
    Odd(u8),
}

impl Display for FanVariants {
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

impl FanVariants {
    fn from_path(path: &Path) -> Result<Self, CustomError> {
        let filename = get_name(&path)?;
        let number = Self::extract_number_from_filename(&filename);
        let label = read_to_string(path)?;

        match label.as_str() {
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

    pub fn get_available() -> Result<Vec<Self>, CustomError> {
        let dir = match read_dir(FANS_BASE_PATH) {
            Ok(dir) => dir,
            Err(err) => {
                return Err(CustomError {
                    display_message: format!("Error reading fans file directory"),
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
                cause: Some(Rc::new(e)),
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

        return Ok(available_fans);
    }

    fn is_string_fan_label(string: &str) -> Result<bool, CustomError> {
        let pattern = r"^fan\d+_label$";
        let regex = Regex::new(pattern).map_err(|e| CustomError {
            display_message: format!("Error validating fan files"),
            internal_message: format!(
                "Error creating regex from pattern: {} on string {}",
                pattern, string
            ),
            cause: Some(Rc::new(e)),
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
}

pub fn read_dir<P>(directory: &P) -> Result<ReadDir, CustomError>
where
    P: AsRef<Path> + Debug + ?Sized,
{
    let metadada_dir = fs::metadata(directory).map_err(|err| CustomError {
        display_message: format!("Error: the directory {:?} doesn't exist", directory),
        internal_message: format!("Error reading {:?} metadata in read_dir fn", directory),
        cause: Some(Rc::new(err)),
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
        cause: Some(Rc::new(e)),
        is_fatal: false,
    })
}

pub fn get_name(directory: &Path) -> Result<String, CustomError> {
    let error = CustomError {
        display_message: format!("Error: failed to read {:?} directory ", directory),
        internal_message: format!(
            "Error: failed get the name of the directory: {:?} ",
            directory
        ),
        cause: None,
        is_fatal: false,
    };

    let directorio_split = directory
        .file_name()
        .ok_or(error.clone())?
        .to_str()
        .ok_or(error)?;

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
            cause: Some(Rc::new(err)),
            is_fatal: false,
        }),
    }
}
