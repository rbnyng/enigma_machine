use eframe::egui;

struct Alphabet;

impl Alphabet {
    const LETTERS: &'static [char; 26] = &[
        'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M',
        'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
    ];

    fn char_to_index(c: char) -> usize {
        c as usize - 'A' as usize
    }

    fn index_to_char(index: usize) -> char {
        Alphabet::LETTERS[index % 26]
    }
}

struct Rotor {
    wiring: Vec<char>,
    reverse_lookup: std::collections::HashMap<char, usize>,
    notch: char,
    position: usize,
}

impl Rotor {
    fn new(wiring: &str, notch: char) -> Self {
        let wiring_array: Vec<char> = wiring.chars().collect();
        // Reverse lookup for rotor makes encode_backward O(1) instead of O(n)
        let reverse_lookup: std::collections::HashMap<char, usize> = wiring_array.iter().enumerate()
            .map(|(i, &c)| (c, i))
            .collect();

        Self {
            wiring: wiring_array,
            reverse_lookup,
            notch,
            position: 0,
        }
    }

    fn encode_forward(&self, input: char) -> char {
        let index = Alphabet::char_to_index(input);
        let shifted_index = (index + self.position) % 26;
        self.wiring[shifted_index]
    }
        
    fn encode_backward(&self, input: char) -> char {
        let index = *self.reverse_lookup.get(&input).expect("Invalid character in reverse lookup");
        let shifted_index = (26 + index - self.position) % 26;
        Alphabet::index_to_char(shifted_index)
    }
    
    fn rotate(&mut self) -> bool {
        self.position = (self.position + 1) % 26;
        Alphabet::index_to_char(self.position) == self.notch
    }

    fn set_position(&mut self, pos: char) {
        self.position = Alphabet::char_to_index(pos);
    }
}

struct Plugboard {
    swaps: std::collections::HashMap<char, char>,
}

impl Plugboard {
    fn new(pairs: &[(char, char)]) -> Self {
        let mut swaps = std::collections::HashMap::new();
        for &(a, b) in pairs {
            swaps.insert(a, b);
            swaps.insert(b, a);
        }
        Self { swaps }
    }

    fn swap(&self, input: char) -> char {
        *self.swaps.get(&input).unwrap_or(&input)
    }
}

struct EnigmaMachine {
    rotors: Vec<Rotor>,
    reflector: [char; 26],
    plugboard: Plugboard,
}

impl EnigmaMachine {
    fn new(rotor_configurations: Vec<(&str, char)>, reflector_wiring: &str, plugboard_pairs: &[(char, char)]) -> Self {
        let rotors = rotor_configurations
            .into_iter()
            .map(|(wiring, notch)| Rotor::new(wiring, notch))
            .collect();

        let reflector: [char; 26] = reflector_wiring.chars().collect::<Vec<_>>().try_into().unwrap();
        let plugboard = Plugboard::new(plugboard_pairs);

        Self { rotors, reflector, plugboard }
    }

    fn rotate_rotors(&mut self) {
        let mut rotate_next = true;
    
        for i in 0..self.rotors.len() {
            if i == 0 || rotate_next {
                rotate_next = self.rotors[i].rotate();
            }
    
            // Double-stepping:
            // Check if the rotor is the second rotor from the right and it has hit its notch
            // If so, ensure the next rotor to its left also rotates in the next cycle
            if i == 1 && self.rotors[i].position == Alphabet::char_to_index(self.rotors[i].notch) {
                rotate_next = true;
            }
        }
    }

    fn encode_decode(&mut self, input: String, output: &mut String) {
        output.clear();

        for input_char in input.to_uppercase().chars().filter(|c| c.is_ascii_alphabetic()) {
            let mut encoded_char = self.plugboard.swap(input_char); // Plugboard swap before encoding

            // Forward through the rotors
            for rotor in &mut self.rotors {
                encoded_char = rotor.encode_forward(encoded_char);
            }

            // Reflector
            let index = Alphabet::char_to_index(encoded_char);
            encoded_char = self.reflector[index];
            encoded_char = Alphabet::index_to_char(Alphabet::char_to_index(encoded_char)); 

            // Through the rotors in reverse order
            for rotor in self.rotors.iter_mut().rev() {
                encoded_char = rotor.encode_backward(encoded_char);
            }

            // Rotate rotors
            self.rotate_rotors();

            encoded_char = self.plugboard.swap(encoded_char); // Plugboard swap back after decoding
            output.push(encoded_char);
        }
    }
}

struct EnigmaApp {
    input: String,
    output: String,
    enigma: EnigmaMachine,
    rotor_positions_input: String,
    plugboard_input: String,
    show_help_bool: bool,
}

impl EnigmaApp {
    fn new() -> Self {
        // Initialize the Enigma Machine with a default configuration
        let enigma = EnigmaMachine::new(
            vec![
                ("EKMFLGDQVZNTOWYHXUSPAIBRCJ", 'Q'),
                ("AJDKSIRUXBLHWTMCQGZNPYFVOE", 'E'),
                ("BDFHJLCPRTXVZNYEIWGAKMUSQO", 'V'),
            ],
            "YRUHQSLDPXNGOKMIEBFZCWVJAT",
            &[
                ('A', 'B'), ('C', 'D'), // Default plugboard configuration
            ],
        );

        Self {
            input: Default::default(),
            output: Default::default(),
            enigma,
            rotor_positions_input: String::new(),
            plugboard_input: String::new(),
            show_help_bool: false,
        }
    }

    fn encode(&mut self) {
        if self.input.chars().all(|c| c.is_ascii_alphabetic() || c == ' ') {
            self.enigma.encode_decode(self.input.clone(), &mut self.output);
        } else {
            self.output = "Invalid input: Please enter only alphabetic characters.".to_string();
        }
    }

    fn set_rotor_positions_from_string(&mut self, positions: &str) {
        let positions: Vec<char> = positions.chars()
            .map(|c| c.to_uppercase().next().unwrap())
            .collect();

        if positions.len() == self.enigma.rotors.len() {
            for (i, &pos) in positions.iter().enumerate() {
                if pos.is_ascii_alphabetic() {
                    self.enigma.rotors[i].set_position(pos);
                    self.output = format!("Rotor positions set.");
                } else {
                    self.output = format!("Invalid input: {} is not an alphabetic character.", pos);
                    return;
                }
            }
        } else {
            self.output = format!("Invalid input: Expected {} positions, got {}.", self.enigma.rotors.len(), positions.len());
        }
    }

    fn update_plugboard_from_input(&mut self) {
        if !self.plugboard_input.is_empty() {
            let pair_strings = self.plugboard_input.split_whitespace().collect::<Vec<&str>>();
            let mut plugboard_pairs = Vec::new();
            let mut letter_set = std::collections::HashSet::new();
            let mut valid_configuration = true;
            let mut error_message = String::new();
    
            for pair_str in pair_strings {
                // Each pair should be exactly 2 characters long
                if pair_str.len() == 2 {
                    let chars: Vec<char> = pair_str.chars().collect();
                    let pair = (chars[0], chars[1]);
    
                    // Check for duplicate or invalid pairs
                    if pair.0 == pair.1 || letter_set.contains(&pair.0) || letter_set.contains(&pair.1) {
                        error_message = format!("Invalid plugboard configuration: duplicate letters or invalid pair '{}{}'.", pair.0, pair.1);
                        valid_configuration = false;
                        break;
                    } else {
                        plugboard_pairs.push(pair);
                        letter_set.insert(pair.0);
                        letter_set.insert(pair.1);
                    }
                } else {
                    error_message = format!("Invalid input: Plugboard pairs must be exactly 2 letters. '{}' is invalid.", pair_str);
                    valid_configuration = false;
                    break;
                }
            }
    
            if valid_configuration {
                self.enigma.plugboard = Plugboard::new(&plugboard_pairs);
                self.output.clear();
                self.output.push_str("Plugboard set.")
            } else {
                // If the configuration is not valid, push the error message to the output
                self.output.clear(); 
                self.output.push_str(&error_message);
            }
        }
    }     
}

impl Default for EnigmaApp {
    fn default() -> Self {
        Self::new()
    }
}

impl eframe::App for EnigmaApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Enigma Machine Simulator");
            ui.separator();
            const AVERAGE_CHAR_WIDTH: f32 = 12.0;
            let text_edit_width = AVERAGE_CHAR_WIDTH * self.enigma.rotors.len() as f32;

            // Plugboard input
            ui.horizontal(|ui| {
                ui.label("Plugboard Pairs (e.g., AB CD):");
                ui.add(egui::TextEdit::singleline(&mut self.plugboard_input)
                    .desired_width(text_edit_width));
                if ui.button("Set Plugboard").clicked() {
                    self.update_plugboard_from_input();
                }    
            });

            ui.add_space(2.5);

            // Set rotor positions            
            ui.horizontal(|ui| {
                ui.label("Set rotor positions (A-Z):");
                ui.add(egui::TextEdit::singleline(&mut self.rotor_positions_input)
                    .desired_width(text_edit_width));
                if ui.button("Set Positions").clicked() {
                    let input = std::mem::take(&mut self.rotor_positions_input);
                    self.set_rotor_positions_from_string(&input);
                    self.rotor_positions_input = input;
                }    
            });

            ui.add_space(2.5);

            ui.horizontal(|ui| {
                ui.label("Current Rotor Positions:");
                for rotor in &self.enigma.rotors {
                    ui.label(format!("{}", (rotor.position as u8 + 'A' as u8) as char));
                }
            });

            // Encode/decode message input
            ui.add(egui::TextEdit::multiline(&mut self.input).hint_text("Enter your message here"));
            ui.add_space(2.5);
            ui.horizontal(|ui| {
                if ui.button("Encode").clicked() {
                    self.encode();
                }
                if ui.button("Decode").clicked() {
                    self.encode(); // Encoding and decoding are the same operation in the Enigma machine
                }
                if ui.button("About").clicked() {
                    self.show_help_bool = !self.show_help_bool;
                }    
            });

            if self.show_help_bool {
                // Help window with information
                egui::Window::new("About the Enigma Machine")
                    .open(&mut self.show_help_bool) 
                    .show(ctx, |ui| {
                        ui.label("The Enigma machine was a cryptographic device used by the German military in World War II for secure communication. 
                        \nIt uses a combination of rotors, a plugboard, and a reflector to encrypt and decrypt messages. 
                        \nHere's a brief overview of its components:");
                        ui.label("\n- Rotors: These are disks with wiring that scrambles the letters. Each rotor can be set to a starting position, affecting the encryption. The historical Enigma machine had three rotors.");
                        ui.label("\n- Plugboard: A panel used to swap pairs of letters before and after they pass through the rotors.");
                        ui.label("\n- Reflector: A component that redirects the signal back through the rotors in a different path, ensuring that the machine can both encrypt and decrypt messages using the same settings.");
                        ui.label("\nHistorically, the rotor arrangement and plugboard configurations were changed daily. Operators would receive codebooks with daily settings.");
                    });
            }

            ui.separator();
            ui.add_space(10.0);

            ui.label("Output:");
            ui.monospace(&self.output);
        });
    }
}

// native app
#[cfg(not(target_arch = "wasm32"))]
fn main() {
    let options = eframe::NativeOptions::default();
    let _ = eframe::run_native(
        "Enigma Machine Simulator",
        options,
        Box::new(|_cc| Box::new(EnigmaApp::new())),
    );
}

// When compiling to web using trunk:
#[cfg(target_arch = "wasm32")]
fn main() {
    let options = eframe::WebOptions::default();
    wasm_bindgen_futures::spawn_local(async {
        eframe::WebRunner::new()
            .start(
                "the_canvas_id", // hardcode it
                options,
                Box::new(|_cc| Box::new(EnigmaApp::new())),
            )
            .await
            .expect("failed to start eframe");
    });
}
