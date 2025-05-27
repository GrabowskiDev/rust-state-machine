use eframe::egui;
use std::collections::HashMap;

struct StateMachine {
    transitions: HashMap<(String, char), String>,
    start_state: String,
    accept_states: Vec<String>,
}

impl StateMachine {
    fn run(&self, input: &str) -> bool {
        let mut current_state = self.start_state.clone();
        for symbol in input.chars() {
            match self.transitions.get(&(current_state.clone(), symbol)) {
                Some(next_state) => current_state = next_state.clone(),
                None => return false,
            }
        }
        self.accept_states.contains(&current_state)
    }
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
}

impl Default for MyApp {
    fn default() -> Self {
        let num_columns = 2; // Minimum 2 columns for alphabet and one for states
        let num_rows = 2; // Minimum 2 rows for states and one for alphabet
        Self {
            num_rows: 2,
            num_columns: 2,
            alphabet_cells: vec!["a".to_string(); num_columns - 1],
            state_names: vec!["q0".to_string(); num_rows - 1],
            transitions: vec![vec!["".to_string(); num_columns - 1]; num_rows - 1],
            accepting_states: vec![false; num_rows - 1],
            input_string: String::new(),
            result: None,
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            // Obsługa dodawania/odejmowania kolumn i wierszy
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
                self.transitions.push(vec!["".to_string(); self.num_columns - 1]);
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

            // Wyświetlanie siatki
            let grid_size = egui::vec2(60.0 * self.num_columns as f32, 60.0 * self.num_rows as f32);
            ui.allocate_ui(grid_size, |ui| {
                egui::Grid::new("my_grid")
                    .min_col_width(60.0)
                    .show(ui, |ui| {
                        for row in 0..self.num_rows {
                            for col in 0..=self.num_columns { // +1 kolumna na checkbox
                                if row == 0 && col == 0 {
                                    ui.label("Akcept.");
                                } else if row == 0 && col == 1 {
                                    ui.label("Stany");
                                } else if row == 0 {
                                    // Edytowalny znak alfabetu
                                    let cell = &mut self.alphabet_cells[col - 2];
                                    if ui.text_edit_singleline(cell).changed() {
                                        if cell.chars().count() > 1 {
                                            let c = cell.chars().next().unwrap();
                                            *cell = c.to_string();
                                        }
                                    }
                                } else if col == 0 {
                                    // Checkbox akceptujący
                                    ui.checkbox(&mut self.accepting_states[row - 1], "");
                                } else if col == 1 {
                                    // Edytowalna nazwa stanu
                                    ui.text_edit_singleline(&mut self.state_names[row - 1]);
                                } else {
                                    // Edytowalne przejście
                                    ui.text_edit_singleline(&mut self.transitions[row - 1][col - 2]);
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
                    let mut names_accept_states = vec![];
                    
                    // Zbieranie nazw stanów akceptujących
                    for( i, &accepting) in self.accepting_states.iter().enumerate() {
                        if accepting {
                            names_accept_states.push(self.state_names[i].clone());
                        }
                    };
                    
                    let alphabet: Vec<char> = self.alphabet_cells.iter().filter_map(|s| s.chars().next()).collect();
                    let mut transitions_ready = HashMap::new();
                    
                    // Przygotowanie przejść
                    for(i, name) in self.transitions.iter().enumerate(){
                        for(j, transition) in name.iter().enumerate() {
                            if !transition.is_empty() && j < alphabet.len() {
                                transitions_ready.insert((self.state_names[i].clone(), alphabet[j]), transition.clone());
                            }
                        }
                    }
                    
                    //TODO: CZEMU UZYWAMY STATEMACHINE ZDEFINIOWANEJ TUTAJ (na górze pliku) 
                    //TODO: A NIE NASZEJ? XD
                    let sm = StateMachine {
                        transitions: transitions_ready,
                        start_state: self.state_names.get(0).cloned().unwrap_or_default(),
                        accept_states: names_accept_states, // uzupełnij według potrzeb
                    };
                    self.result = Some(sm.run(&self.input_string));
                }
            });
            if let Some(result) = self.result {
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