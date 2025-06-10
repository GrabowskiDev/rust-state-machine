use eframe::egui;
use std::collections::HashMap;

mod elements;
mod automats;

use elements::Alphabet::Alphabet;
use elements::Node::Node;
use automats::DAS::DAS;
use automats::ENAS::{ENAS, ENASNode};

#[derive(PartialEq)]
enum AutomatType {
    DAS,
    ENAS,
}

struct MyApp {
    num_rows: usize,
    num_columns: usize,
    alphabet_cells: Vec<String>,
    state_names: Vec<String>,
    transitions: Vec<Vec<String>>,
    accepting_states: Vec<bool>,
    input_string: String,
    result: Option<bool>,
    validation_message: String,
    automat_type: AutomatType,
}

impl Default for MyApp {
    fn default() -> Self {
        let num_columns = 2;
        let num_rows = 2;
        Self {
            num_rows,
            num_columns,
            alphabet_cells: vec!["a".to_string(); num_columns - 1],
            state_names: vec!["q0".to_string(); num_rows - 1],
            transitions: vec![vec!["".to_string(); num_columns - 1]; num_rows - 1],
            accepting_states: vec![false; num_rows - 1],
            input_string: String::new(),
            result: None,
            validation_message: String::new(),
            automat_type: AutomatType::DAS,
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            // Przełącznik typu automatu
            ui.horizontal(|ui| {
                ui.label("Typ automatu:");
                ui.radio_value(&mut self.automat_type, AutomatType::DAS, "DAS");
                ui.radio_value(&mut self.automat_type, AutomatType::ENAS, "ε-NAS");
            });

            // Przyciski do edycji tabeli
            if ui.button("Dodaj znak alfabetu").clicked() {
                self.num_columns += 1;
                self.alphabet_cells.push("".to_string());
                for row in &mut self.transitions {
                    row.push("".to_string());
                }
            }
            if ui.button("Dodaj stan").clicked() {
                self.num_rows += 1;
                self.state_names.push("".to_string());
                self.transitions.push(vec!["".to_string(); self.num_columns - 1 + if matches!(self.automat_type, AutomatType::ENAS) { 1 } else { 0 }]);
                self.accepting_states.push(false);
            }
            if ui.button("Usuń znak alfabetu").clicked() && self.num_columns > 2 {
                self.num_columns -= 1;
                self.alphabet_cells.pop();
                for row in &mut self.transitions {
                    row.pop();
                }
            }
            if ui.button("Usuń stan").clicked() && self.num_rows > 2 {
                self.num_rows -= 1;
                self.state_names.pop();
                self.transitions.pop();
                self.accepting_states.pop();
            }

            // Wyznacz liczbę kolumn (dla ENAS +1 na epsilon)
            let extra_epsilon = matches!(self.automat_type, AutomatType::ENAS);
            let total_columns = self.num_columns - 1 + if extra_epsilon { 1 } else { 0 };

            // Tabela stanów i przejść
            let grid_size = egui::vec2(60.0 * (total_columns + 2) as f32, 60.0 * self.num_rows as f32);
            ui.allocate_ui(grid_size, |ui| {
                egui::Grid::new("my_grid")
                    .min_col_width(60.0)
                    .show(ui, |ui| {
                        for row in 0..self.num_rows {
                            for col in 0..=(total_columns + 1) {
                                if row == 0 && col == 0 {
                                    ui.label("Akcept.");
                                } else if row == 0 && col == 1 {
                                    ui.label("Stany");
                                } else if row == 0 {
                                    // Nagłówki alfabetu i epsilon
                                    if col >= 2 && col < 2 + self.num_columns - 1 {
                                        let idx = col - 2;
                                        let cell = &mut self.alphabet_cells[idx];
                                        if ui.text_edit_singleline(cell).changed() {
                                            if cell.chars().count() > 1 {
                                                let c = cell.chars().next().unwrap();
                                                *cell = c.to_string();
                                            }
                                        }
                                    } else if extra_epsilon && col == 2 + self.num_columns - 1 {
                                        ui.label("ε");
                                    }
                                } else if col == 0 {
                                    ui.checkbox(&mut self.accepting_states[row - 1], "");
                                } else if col == 1 {
                                    ui.text_edit_singleline(&mut self.state_names[row - 1]);
                                } else {
                                    // Przejścia
                                    let tcol = col - 2;
                                    if tcol < total_columns {
                                        if self.transitions[row - 1].len() < total_columns {
                                            self.transitions[row - 1].resize(total_columns, "".to_string());
                                        }
                                        ui.text_edit_singleline(&mut self.transitions[row - 1][tcol]);
                                    }
                                }
                            }
                            ui.end_row();
                        }
                    });
            });

            ui.separator();
            ui.horizontal(|ui| {
                ui.label("Ciąg wejściowy:");
                ui.text_edit_singleline(&mut self.input_string);
                if ui.button("Sprawdź").clicked() {
                    self.result = None;
                    self.validation_message.clear();

                    match self.automat_type {
                        AutomatType::DAS => {
                            // 1. Budowa alfabetu
                            let mut alphabet = Alphabet::new();
                            for s in &self.alphabet_cells {
                                if let Some(c) = s.chars().next() {
                                    alphabet.add(c);
                                }
                            }

                            // 2. Budowa stanów
                            let mut nodes = vec![];
                            for (i, name) in self.state_names.iter().enumerate() {
                                let mut node = Node::new(name, self.accepting_states[i]);
                                for (j, cell) in self.transitions[i].iter().enumerate().take(self.num_columns - 1) {
                                    if !cell.is_empty() {
                                        if let Some(symbol) = self.alphabet_cells[j].chars().next() {
                                            node.add_connection(symbol, cell);
                                        }
                                    }
                                }
                                nodes.push(node);
                            }

                            // 3. Budowa DAS
                            let mut das = DAS::new(alphabet);
                            for node in nodes {
                                das.add_state(node);
                            }
                            if let Some(start) = self.state_names.get(0) {
                                das.set_start_state(start);
                            }

                            // --- WALIDACJA DAS ---
                            let mut errors = das.validate();

                            // Sprawdź, czy ciąg wejściowy zawiera tylko znaki z alfabetu
                            for c in self.input_string.chars() {
                                if !das.alphabet.contains(&c) {
                                    errors.push(format!(
                                        "Ciąg wejściowy zawiera znak '{}' spoza alfabetu.",
                                        c
                                    ));
                                }
                            }

                            if !errors.is_empty() {
                                self.result = None;
                                self.validation_message = errors.join("\n");
                            } else {
                                // 4. Sprawdzenie ciągu
                                self.result = Some(das.process(&self.input_string));
                                self.validation_message.clear();
                            }
                        }
                        AutomatType::ENAS => {
                            // 1. Budowa alfabetu (bez epsilon)
                            let mut alphabet = Alphabet::new();
                            for s in &self.alphabet_cells {
                                if let Some(c) = s.chars().next() {
                                    alphabet.add(c);
                                }
                            }

                            // 2. Budowa stanów ENAS
                            let mut nodes = vec![];
                            for (i, name) in self.state_names.iter().enumerate() {
                                let mut node = ENASNode::new(name, self.accepting_states[i]);
                                // Przejścia dla alfabetu
                                for (j, cell) in self.transitions[i].iter().enumerate().take(self.num_columns - 1) {
                                    if !cell.trim().is_empty() {
                                        if let Some(symbol) = self.alphabet_cells[j].chars().next() {
                                            let targets: Vec<String> = cell
                                                .split(',')
                                                .map(|s| s.trim().to_string())
                                                .filter(|s| !s.is_empty())
                                                .collect();
                                            if !targets.is_empty() {
                                                node.add_connection(symbol, targets);
                                            }
                                        }
                                    }
                                }
                                // Przejście epsilon (ostatnia kolumna)
                                if self.transitions[i].len() > self.num_columns - 1 {
                                    let cell = &self.transitions[i][self.num_columns - 1];
                                    if !cell.trim().is_empty() {
                                        let targets: Vec<String> = cell
                                            .split(',')
                                            .map(|s| s.trim().to_string())
                                            .filter(|s| !s.is_empty())
                                            .collect();
                                        if !targets.is_empty() {
                                            node.add_connection('ε', targets);
                                        }
                                    }
                                }
                                nodes.push(node);
                            }

                            // 3. Budowa ENAS
                            let mut enas = ENAS::new(alphabet);
                            for node in nodes {
                                enas.add_state(node);
                            }
                            if let Some(start) = self.state_names.get(0) {
                                enas.set_start_state(start);
                            }

                            // --- WALIDACJA ENAS ---
                            let mut errors = enas.validate();

                            // Sprawdź, czy ciąg wejściowy zawiera tylko znaki z alfabetu
                            for c in self.input_string.chars() {
                                if !enas.alphabet.contains(&c) {
                                    errors.push(format!(
                                        "Ciąg wejściowy zawiera znak '{}' spoza alfabetu.",
                                        c
                                    ));
                                }
                            }

                            if !errors.is_empty() {
                                self.result = None;
                                self.validation_message = errors.join("\n");
                            } else {
                                // 4. Sprawdzenie ciągu
                                self.result = Some(enas.process(&self.input_string));
                                self.validation_message.clear();
                            }
                        }
                    }
                }
            });

            // Wyświetlanie komunikatów walidacji lub wyniku
            if !self.validation_message.is_empty() {
                for line in self.validation_message.lines() {
                    ui.colored_label(egui::Color32::RED, line);
                }
            } else if let Some(result) = self.result {
                if result {
                    ui.colored_label(egui::Color32::GREEN, "Ciąg zaakceptowany");
                } else {
                    ui.colored_label(egui::Color32::RED, "Ciąg odrzucony");
                }
            }
        });
    }
}

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Maszyna Stanów (egui)",
        options,
        Box::new(|_cc| Box::new(MyApp::default())),
    )
}
