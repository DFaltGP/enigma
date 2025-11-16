#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

// Declara o módulo enigma, que será definido em src/enigma.rs
pub mod enigma;

use enigma::{EnigmaConfig, EncryptionStep};

/// Processa (criptografa/descriptografa) um texto completo e retorna apenas o resultado final.
/// Esta função é stateless; a configuração da máquina é fornecida a cada chamada.
#[tauri::command]
fn enigma_process_string(config: EnigmaConfig, text: String) -> String {
    // Cria uma nova instância da máquina com base na configuração da UI
    let mut machine = enigma::EnigmaMachine::new(config);
    machine.process_string(&text)
}

/// Processa um texto e retorna uma lista detalhada de cada passo da criptografia
#[tauri::command]
fn enigma_process_detailed(config: EnigmaConfig, text: String) -> Vec<EncryptionStep> {
    // Cria uma nova instância da máquina
    let mut machine = enigma::EnigmaMachine::new(config);
    machine.process_string_detailed(&text)
}

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            enigma_process_string,
            enigma_process_detailed
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
