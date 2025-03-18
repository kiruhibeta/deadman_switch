// Importing necessary libraries
use serde::{Deserialize, Serialize};
use std::fs;
use std::time::{SystemTime, UNIX_EPOCH};
use lettre::{Message, SmtpTransport, Transport};
use lettre::transport::smtp::authentication::Credentials;

// Defining the Config struct, which holds the configuration data loaded from the config file.
#[derive(Debug, Deserialize, Serialize)]
struct Config {
    smtp_host: String,              // SMTP server host
    smtp_port: u16,                 // SMTP server port
    smtp_user: String,              // SMTP username
    smtp_password: String,          // SMTP password
    from_email: String,             // Sender's email address
    to_email: String,               // Recipient's email address for reminders
    checkin_file: String,           // File where the last check-in time is stored
    counter_file: String,           // File for storing some counter (not used in the code)
    days_reminder: u64,             // Days before sending a reminder email
    days_deadman: u64,              // Days after which the deadman switch is triggered
    seconds_in_a_day: u64,         // Number of seconds in a day (86400)
    count_sent_mail: u32,           // Count of sent emails (not used in the code)
    family_members: Vec<String>,    // List of family members to notify if deadman switch is activated
    files_to_attach: Vec<String>,   // List of files to attach in the email (not used in the code)
    reminder_subject: String,      // Subject of the reminder email
    reminder_message: String,      // Body of the reminder email
    dead_man_activation_subject: String, // Subject of the deadman activation email
    dead_man_activation_message: String, // Body of the deadman activation email
}

// Function to load the configuration from a JSON file
fn load_config(filename: &str) -> Config {
    // Read the configuration file as a string
    let config_data = fs::read_to_string(filename).expect("Config file could not be read");
    // Parse the string as JSON and deserialize it into the Config struct
    serde_json::from_str(&config_data).expect("Config JSON format is invalid")
}

// Function to send an email
fn send_email(config: &Config, to: &str, subject: &str, body: &str) {
    // Set up SMTP credentials using the provided user and password
    let creds = Credentials::new(config.smtp_user.clone(), config.smtp_password.clone());

    // Build the email message
    let email = Message::builder()
        .from(config.from_email.parse().expect("Invalid sender email address"))
        .to(to.parse().expect("Invalid recipient email address"))
        .subject(subject)
        .body(body.to_string())
        .expect("Failed to build email");

    // Set up the SMTP client to send the email
    let mailer = SmtpTransport::relay(&config.smtp_host)
        .expect("Could not connect to SMTP server")
        .credentials(creds)
        .port(config.smtp_port)
        .build();

    // Send the email and handle errors
    if let Err(e) = mailer.send(&email) {
        eprintln!("Error sending email: {}", e);
    } else {
        println!("Email successfully sent.");
    }
}

// Function to check the deadman switch condition
fn check_deadman_switch(config: &Config) {
    // Read the last check-in time from the file
    let checkin_time = fs::read_to_string(&config.checkin_file).unwrap_or_else(|_| "0".to_string());
    let last_checkin: u64 = checkin_time.trim().parse().unwrap_or(0);
    
    // Get the current system time in seconds since UNIX epoch
    let now = SystemTime::now().duration_since(UNIX_EPOCH).expect("Failed to read system time").as_secs();

    // Check if the deadman switch should be activated (based on days_deadman)
    if now >= last_checkin + config.days_deadman * config.seconds_in_a_day {
        // If so, send the deadman activation emails to family members
        for email in &config.family_members {
            send_email(config, email, &config.dead_man_activation_subject, &config.dead_man_activation_message);
        }
    }
    // If not, check if a reminder should be sent (based on days_reminder)
    else if now >= last_checkin + config.days_reminder * config.seconds_in_a_day {
        send_email(config, &config.to_email, &config.reminder_subject, &config.reminder_message);
    }
}

// Main function where the execution starts
fn main() {
    // Load the configuration from the config.json file
    let config = load_config("config.json");

    // Check the deadman switch condition
    check_deadman_switch(&config);
}
