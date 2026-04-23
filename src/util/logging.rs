/// Logging module for Rust OpenGL project.
/// Provides a simple logging system with different log levels (info, error, warning) and the ability to write logs to a file.
/// Logs are also printed to the console with color coding for better readability.
///
/// author: "Triley"
/// copyright: "Copyright (c) 2024 Triley. All rights reserved."
/// credits: ["Triley"]
/// maintainer: "Triley"
/// version: "0.1.0"

// Built-in imports
use std::fs::{self, File};
use std::io::Write;

// Third-party imports
use chrono::{TimeZone, Utc};
use colored::{ColoredString, Colorize};

// Local imports


pub enum MessageType {
    INFO,
    ERROR,
    WARNING,
}

#[allow(dead_code)]
#[derive(Copy, Clone)]
pub enum SystemType {
    ROOT,
    SYSTEM,
    SUBSYSTEM
}



pub fn info(message: &str, name: &str, system_type: SystemType){
    println!("{}",
        format!("[{}][{}][{}][{}] ..... {}",
        get_color_name_string(name.to_uppercase().as_str(), system_type),
        Utc::now().format("%Y-%m-%d %H:%M:%S").to_string().bright_purple(),
        "UTC".bright_blue(),
        get_prefix_string(MessageType::INFO).bright_green().bold(),
        message).as_str()
    );
}

pub fn error(message: &str, name: &str, system_type: SystemType){
    println!("{}",
        format!("[{}][{}][{}][{}] ..... {}",
        get_color_name_string(name.to_uppercase().as_str(), system_type),
        Utc::now().format("%Y-%m-%d %H:%M:%S").to_string().bright_purple(),
        "UTC".bright_blue(),
        get_prefix_string(MessageType::ERROR).bright_red().bold(),
        message).as_str()
    );
}

pub fn warning(message: &str, name: &str, system_type: SystemType){
    println!("{}",
        format!("[{}][{}][{}][{}] ..... {}",
        get_color_name_string(name.to_uppercase().as_str(), system_type),
        Utc::now().format("%Y-%m-%d %H:%M:%S").to_string().bright_purple(),
        "UTC".bright_blue(),
        get_prefix_string(MessageType::WARNING).bright_yellow().bold(),
        message).as_str()
    );
}

pub fn get_prefix_string(msg_type: MessageType) -> &'static str {
    match msg_type{
        MessageType::INFO => "INFO",
        MessageType::ERROR => "ERROR",
        MessageType::WARNING => "WARNING"
    }
}
pub fn get_color_name_string(name: &str, system_type: SystemType) -> ColoredString {
    match system_type {
        SystemType::ROOT => name.bright_yellow().bold(),
        SystemType::SYSTEM => name.bright_blue().bold(),
        SystemType::SUBSYSTEM => name.bright_cyan().bold()

    }
}



///
/// # Setup example
/// ```
/// // Create logger struct
/// let mut logger: Logger = Logger::new(Some("main"), Some(SystemType::SYSTEM));
///
/// // Then
/// logger.add_log("Message here!")
/// logger.add_error("Error message here!")
/// logger.add_warning("Warning message here!")
///
/// ```
///
pub struct Logger{
    name: String,
    system_type: SystemType,
    filepath: String,
    filename: String,
    file_created: bool,
}


pub fn sanity_check(){
    if fs::metadata("log").is_ok(){
        fs::remove_dir_all("log").expect("Couldn't remove log directory.");
        println!("Log directory already exists, removing and recreating it.");
    }

    fs::create_dir("log").expect("Couldn't create log directory.");

    println!("Logger sanity check passed.");
}


#[allow(dead_code)]
impl Logger {
    pub fn new(name: Option<&str>, system_type: Option<SystemType>)-> Self{
        Self{
            name: match name {
                Some(value) => value.to_uppercase(),
                None => "ROOT".to_string(),
            },
            system_type: match system_type {
                Some(value) => value,
                None => SystemType::SYSTEM,
            },
            filename: "log.log".to_string(),
            filepath: "log\\".to_string(),
            file_created: false,
        }
    }


    pub fn init(&mut self){
        // Check if file already exists, if not create it and write initial log message.
        if !self.file_created {
            self.file_created = fs::metadata(self.format_filepath().as_str()).is_ok();
            if self.file_created {
                let mut f = File::options()
                .append(true)
                .open(self.format_filepath().as_str())
                .expect("Couldn't open log file.");
                writeln!(f, "[{}] Logger Initialize", self.name).expect("Couldn't write to file.");
            }

        }

        if !self.file_created{
            // let temp_str: String = format!(""self.filename);
            let mut f = File::create(self.format_filepath().as_str()).expect("Couldn't create log file.");
            writeln!(f, "[{}] Logger Initialize", self.name).expect("Couldn't write to file.");
            self.file_created = true;
            self.add_info(format!("File Creation started: {}{} ", self.filepath, self.filename).as_str());
        }

    }
    pub fn set_filepath(&mut self, filepath: &str){
        self.filepath = filepath.to_string();
    }
    pub fn set_logger_name(&mut self, name: &str){
        self.name = name.to_uppercase();
    }
    pub fn set_system_type(&mut self, system_type: SystemType){
        self.system_type = system_type
    }

    pub fn add_info(&mut self, message: &str){
        let log_str = format!("[{}][{}][{}] ..... {}",
            self.name,
            Utc::now().format("%Y-%m-%d %H:%M:%S").to_string(),
            self.get_prefix_string(MessageType::INFO),
            message);

        self.write_to_file(&log_str);

        info(message, self.name.as_str(), self.system_type);
    }

    pub fn add_error(&mut self, message: &str){

        let log_str = format!("[{}][{}][{}] ..... {}",
            self.name,
            Utc::now().format("%Y-%m-%d %H:%M:%S").to_string(),
            self.get_prefix_string(MessageType::ERROR),
            message);

        self.write_to_file(&log_str);

        error(message, self.name.as_str(), self.system_type);

    }

    pub fn add_warning(&mut self, message: &str){

        let log_str = format!("[{}][{}][{}] ..... {}",
            self.name,
            Utc::now().format("%Y-%m-%d %H:%M:%S").to_string(),
            self.get_prefix_string(MessageType::WARNING),
            message);

        self.write_to_file(&log_str);

        warning(message, self.name.as_str(), self.system_type);
    }

    fn get_prefix_string(&self, msg_type: MessageType) -> &str {
        match msg_type{
            MessageType::INFO => "INFO",
            MessageType::WARNING => "WARNING",
            MessageType::ERROR => "ERROR"
        }
    }
    fn get_color_name_string(&self) -> ColoredString {
        match self.system_type {
            SystemType::ROOT => self.name.bright_yellow().bold(),
            SystemType::SYSTEM => self.name.bright_blue().bold(),
            SystemType::SUBSYSTEM => self.name.bright_cyan().bold()

        }
    }

    fn write_to_file(&mut self, message: &str){
        if self.file_created{
            // Try to open file
            let mut f = File::options()
                .append(true)
                .open(self.format_filepath().as_str())
                .expect("Couldn't open log file.");
            // Write log message to file
            writeln!(f, "{}", message)
            .expect("Couldn't write to file.");
        } else {
            self.init();
            if self.file_created{
                // Try to open file again after creation
                let mut f = File::options()
                .append(true)
                .open(self.format_filepath().as_str())
                .expect("Couldn't open log file.");
                // Write log message to file
                writeln!(f, "{}", message)
                .expect("Couldn't write to file.");
            } else {
                println!("Couldn't create log file.");
            }
        }
    }

    fn format_filepath(&self) -> String {
        format!("{}{}", self.filepath, self.filename)
    }
}