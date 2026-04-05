use chrono::{NaiveDate, Utc};
use egui::{Response, Ui, Widget};

#[derive(serde::Deserialize, serde::Serialize)]
pub struct DatePicker {
    selected_date: NaiveDate,
    day: String,
    month: String,
    year: String,
}

impl DatePicker {
    pub fn new() -> DatePicker {
        DatePicker {selected_date: Utc::now().date_naive(), day: String::new(), month: String::new(), year: String::new()}
    }

    pub fn get_date(&self) -> NaiveDate {
        let year = self.year.parse::<i32>().unwrap();
        let month = self.month.parse::<u32>().unwrap();
        let day = self.day.parse::<u32>().unwrap();
        NaiveDate::from_ymd_opt(year, month, day).unwrap()
    }
}

impl Widget for &mut DatePicker {
    fn ui(mut self, ui: &mut Ui) -> Response {
        ui.horizontal(|ui| {
            ui.label("Day");
            ui.text_edit_singleline(&mut self.day);
            ui.label("Month");
            ui.text_edit_singleline(&mut self.month);
            ui.label("Year");
            ui.text_edit_singleline(&mut self.year);
        }).response
    }
}