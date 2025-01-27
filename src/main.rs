use clap::{Parser, Subcommand};
use serde::{Deserialize, Serialize};
use std::fs::OpenOptions;
use std::io::{Read, Write};
use chrono::Utc;

#[derive(Parser)]
#[command(name = "RemoteCare")]
#[command(about = "Manage medical emergencies in rural areas", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Log {
        #[arg(short = 'n', long)] // Short option: -n
        name: String,
        #[arg(short = 'y', long)] // Short option: -y
        symptoms: String,
        #[arg(short = 'l', long)] // Short option: -l
        location: String,
        #[arg(short = 'v', long)] // Short option: -v
        severity: String,
    },
    Prioritize,
    Guide {
        #[arg(short = 't', long)] // Short option: -t
        topic: String,
    },
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Emergency {
    name: String,
    symptoms: String,
    location: String,
    severity: String,
    timestamp: String,
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Log { name, symptoms, location, severity } => {
            log_emergency(&name, &symptoms, &location, &severity);
        }
        Commands::Prioritize => {
            prioritize_emergencies();
        }
        Commands::Guide { topic } => {
            provide_guide(&topic);
        }
    }
}

fn log_emergency(name: &str, symptoms: &str, location: &str, severity: &str) {
    let timestamp = Utc::now().to_string();
    let emergency = Emergency {
        name: name.to_string(),
        symptoms: symptoms.to_string(),
        location: location.to_string(),
        severity: severity.to_string(),
        timestamp,
    };

    let file_path = "emergencies.json";
    let mut emergencies = load_emergencies(file_path);
    emergencies.push(emergency);
    save_emergencies(file_path, &emergencies);

    println!("Emergency logged successfully!");
}

fn load_emergencies(file_path: &str) -> Vec<Emergency> {
    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(file_path)
        .expect("Failed to open emergencies file");

    let mut content = String::new();
    file.read_to_string(&mut content).unwrap_or_default();

    if content.is_empty() {
        return Vec::new();
    }

    serde_json::from_str(&content).expect("Failed to parse emergencies")
}

fn save_emergencies(file_path: &str, emergencies: &Vec<Emergency>) {
    let mut file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(file_path)
        .expect("Failed to open emergencies file");

    let content = serde_json::to_string_pretty(emergencies).expect("Failed to serialize emergencies");
    file.write_all(content.as_bytes()).expect("Failed to save emergencies");
}

fn prioritize_emergencies() {
    let file_path = "emergencies.json";
    let emergencies = load_emergencies(file_path);

    let mut sorted_emergencies = emergencies.clone();
    sorted_emergencies.sort_by(|a, b| b.severity.cmp(&a.severity));

    println!("Prioritized Emergencies:");
    for (i, emergency) in sorted_emergencies.iter().enumerate() {
        println!("{}. [{}] {} - {}", i + 1, emergency.severity, emergency.name, emergency.symptoms);
    }
}

fn provide_guide(topic: &str) {
    let guides = vec![
        ("burns", "Cool the burn under running water for 10 minutes. Do not apply creams."),
        ("fractures", "Immobilize the limb and avoid moving it."),
        ("fever", "Keep the patient hydrated and check for other symptoms."),
    ];

    match guides.iter().find(|(t, _)| *t == topic) {
        Some((_, guide)) => println!("First Aid for {}: {}", topic, guide),
        None => println!("No guide available for {}", topic),
    }
}
