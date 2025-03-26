use fizcalczaok::{oblicz, potegowanie_str_wrapper, stworz_dzial};

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct FizCalcApp {
    main_calc: String,
    main_calc_answer: String,
    single_calc: String,
    single_calc_answer: String,
}

impl Default for FizCalcApp {
    fn default() -> Self {
        Self {
            main_calc: String::new(),
            single_calc: String::new(),
            main_calc_answer: String::new(),
            single_calc_answer: String::new(),
        }
    }
}

impl FizCalcApp {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Default::default()
    }
}

impl eframe::App for FizCalcApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                egui::widgets::global_theme_preference_buttons(ui);
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Fizcalczaok");
            ui.label("Kalkulator, zgodny z regułami zaokrągleń w działaniach arytmetycznych laboratorium fizyki na PO");
            ui.label(
                format!("{}\n{}\n{}\n{}\n{}", "- Nie ma kolejności wykonywania działań, od lewej do prawej",
                "- Nie mieszaj działań dodawania, odejmowania z działań z mnożeniem, dzieleniem",
                "- Poprawność działań, na podstawie przykładów z poradnika pdf zostały zaimplementowane tu na podstawie testów: https://github.com/Szybet/fizcalczaok",
                "- By zrobić np pierwiastek 3 stopnia, użyj potęgi 1/3 czyli 0.3333333...",
                "- By zaznaczyć że liczba jest liczbą dokładną, należy użyć znak \"d\" przed nim, np: d5",
            ));

            if ui.button("Sprzątanie").clicked() {
                self.main_calc = "".to_string();
                self.main_calc_answer = "".to_string();
                self.single_calc = "".to_string();
                self.single_calc_answer = "".to_string();
            }

            ui.label("Podaj działanie: (dodawanie: +, odejmowanie: -, mnożenie: *, dzielenie: /)");
            ui.horizontal(|ui| {
                ui.text_edit_multiline(&mut self.main_calc);
                if ui.button("Oblicz").clicked() {
                    let d = stworz_dzial(&self.main_calc);
                    let o = oblicz(d.clone());
                    self.main_calc_answer = o.to_string();             
                }
            });
            ui.label(format!("Wynik: {}", self.main_calc_answer));

            ui.separator();

            ui.label("Podaj działanie pojedyńcze (potęgowanie: ^): ");
            ui.horizontal(|ui| {
                ui.text_edit_multiline(&mut self.single_calc);
                if ui.button("Oblicz").clicked() {
                    let d = potegowanie_str_wrapper(self.single_calc.clone());
                    self.single_calc_answer = d.to_string();             
                }
            });
            ui.label(format!("Wynik: {}", self.single_calc_answer));
            
            ui.separator();
        });
    }
}
