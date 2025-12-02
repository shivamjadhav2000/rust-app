use eframe::egui;
use chrono::Local;

// 1. Define the state of your application
// This struct holds all the data that changes while the app is running.
struct SimpleApp {
    name: String,
    todo_input: String,
    todos: Vec<String>,
}

// 2. Initialize the default values
impl Default for SimpleApp {
    fn default() -> Self {
        Self {
            name: "World".to_owned(),
            todo_input: "".to_owned(),
            todos: vec![
                "Buy milk".to_owned(),
                "Learn Rust".to_owned(),
            ],
        }
    }
}

// 3. Implement the App trait (The logic loop)
impl eframe::App for SimpleApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // --- CLOCK LOGIC ---
        // We request a repaint every frame to keep the clock updating smoothly.
        // In a real power-saving app, you might use a timer, but this is simple.
        ctx.request_repaint(); 
        
        let current_time = Local::now().format("%H:%M:%S").to_string();
        let date_str = Local::now().format("%A, %B %d").to_string();

        // --- UI DRAWING ---
        egui::CentralPanel::default().show(ctx, |ui| {
            
            // SECTION 1: HEADER & CLOCK
            ui.heading("Rust Dashboard");
            ui.separator();
            
            ui.horizontal(|ui| {
                ui.label("Current Time: ");
                ui.colored_label(egui::Color32::LIGHT_BLUE, format!("{}", current_time));
                ui.label(format!(" ({})", date_str));
            });

            ui.add_space(20.0);

            // SECTION 2: HELLO WORLD INPUT
            ui.label("Who should we greet?");
            ui.horizontal(|ui| {
                ui.text_edit_singleline(&mut self.name);
                if ui.button("Greet").clicked() {
                    // Logic could go here, but strictly immediate mode 
                    // updates the UI instantly anyway.
                }
            });
            ui.label(format!("Hello, {}!", self.name));

            ui.add_space(20.0);
            ui.separator();

            // SECTION 3: TO-DO LIST
            ui.heading("To-Do List");
            
            ui.horizontal(|ui| {
                // Pressing Enter in the text box usually loses focus,
                // but we can check if the button is clicked.
                let response = ui.text_edit_singleline(&mut self.todo_input);
                
                // Add on button click OR on Enter key
                if ui.button("Add Task").clicked() || (response.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter))) {
                    if !self.todo_input.trim().is_empty() {
                        self.todos.push(self.todo_input.clone());
                        self.todo_input.clear();
                        
                        // Keep focus on input for rapid entry
                        response.request_focus();
                    }
                }
            });

            ui.add_space(10.0);

            // Display list items with a delete button
            // We use an index iterator in reverse so removing items doesn't panic
            let mut to_remove = None;
            
            egui::ScrollArea::vertical().max_height(200.0).show(ui, |ui| {
                for (index, task) in self.todos.iter().enumerate() {
                    ui.horizontal(|ui| {
                        if ui.button("❌").clicked() {
                            to_remove = Some(index);
                        }
                        ui.label(task);
                    });
                }
            });

            // Handle removal safely after the loop
            if let Some(index) = to_remove {
                self.todos.remove(index);
            }
        });
    }
}

fn main() -> eframe::Result<()> {
    // Window configuration
    let options = eframe::NativeOptions {
        viewport: eframe::egui::ViewportBuilder::default()
            .with_inner_size([400.0, 500.0]), // Set initial width/height
        ..Default::default()
    };

    // Run the application
    eframe::run_native(
        "Simple Rust App",
        options,
        Box::new(|_cc| Box::new(SimpleApp::default())),
    )
}
