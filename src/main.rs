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
    alphabet: String,
    states: String,
    start_state: String,
    accept_states: String,
    transitions: String,
    test_input: String,
    result: Option<bool>,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            alphabet: String::new(),
            states: String::new(),
            start_state: String::new(),
            accept_states: String::new(),
            transitions: String::new(),
            test_input: String::new(),
            result: None,
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Maszyna stanów");

            ui.label("Alfabet (np. a,b,c):");
            ui.text_edit_singleline(&mut self.alphabet);

            ui.label("Stany (np. q0,q1,q2):");
            ui.text_edit_singleline(&mut self.states);

            ui.label("Stan początkowy:");
            ui.text_edit_singleline(&mut self.start_state);

            ui.label("Stany akceptujące (np. q2):");
            ui.text_edit_singleline(&mut self.accept_states);

            ui.label("Przejścia (np. q0,a,q1; q1,b,q2):");
            ui.text_edit_multiline(&mut self.transitions);

            ui.separator();

            ui.label("Ciąg do sprawdzenia:");
            ui.text_edit_singleline(&mut self.test_input);

            if ui.button("Sprawdź").clicked() {
                let transitions = self.transitions
                    .split(';')
                    .filter_map(|t| {
                        let parts: Vec<_> = t.trim().split(',').collect();
                        if parts.len() == 3 {
                            Some(((parts[0].trim().to_string(), parts[1].trim().chars().next().unwrap()), parts[2].trim().to_string()))
                        } else {
                            None
                        }
                    })
                    .collect();

                let sm = StateMachine {
                    transitions,
                    start_state: self.start_state.trim().to_string(),
                    accept_states: self.accept_states.split(',').map(|s| s.trim().to_string()).collect(),
                };

                self.result = Some(sm.run(&self.test_input));
            }

            if let Some(result) = self.result {
                if result {
                    ui.colored_label(egui::Color32::GREEN, "Ciąg akceptowany!");
                } else {
                    ui.colored_label(egui::Color32::RED, "Ciąg odrzucony.");
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