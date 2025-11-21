use std::{collections::BTreeMap, str::FromStr};

use chrono::prelude::*;
use chrono::Local;
use chrono_tz::Tz;

use crate::render::FormattedPart;

use crate::{config::ZellijState, widgets::widget::Widget};

pub struct DateTimeWidget {
    format: String,
    time_format: String,
    date_format: String,
    color_format: Vec<FormattedPart>,
    time_zone: Option<Tz>,
    locale: Option<Locale>,
}

impl DateTimeWidget {
    pub fn new(config: &BTreeMap<String, String>) -> Self {
        let mut format = "%H:%M";
        if let Some(form) = config.get("datetime_format") {
            format = form;
        }

        let mut time_format = "%H:%M";
        if let Some(form) = config.get("datetime_time_format") {
            time_format = form;
        }

        let mut date_format = "%Y-%m-%d";
        if let Some(form) = config.get("datetime_date_format") {
            date_format = form;
        }

        let mut time_zone_string = "Etc/UTC";
        if let Some(tz_string) = config.get("datetime_timezone") {
            time_zone_string = tz_string;
        }

        let time_zone = Tz::from_str(time_zone_string).ok();

        let mut color_format = "";
        if let Some(form) = config.get("datetime") {
            color_format = form;
        }

        let mut locale_string = "en_US";
        if let Some(loc_string) = config.get("datetime_locale") {
            locale_string = loc_string;
        }

        let locale = match locale_string.parse::<Locale>() {
            Ok(lc) => Some(lc),
            Err(_) => None,
        };

        Self {
            format: format.to_owned(),
            date_format: date_format.to_owned(),
            time_format: time_format.to_owned(),
            color_format: FormattedPart::multiple_from_format_string(color_format, config),
            time_zone,
            locale,
        }
    }
}

impl Widget for DateTimeWidget {
    fn process(&self, _name: &str, _state: &ZellijState) -> String {
        let date = Local::now();
        let mut tz = Tz::UTC;
        if let Some(t) = self.time_zone {
            tz = t;
        }

        let date_with_tz = date.with_timezone(&tz);

        self.color_format
            .iter()
            .map(|f| {
                let mut content = f.content.clone();

                if content.contains("{format}") {
                    content = content.replace(
                        "{format}",
                        match &self.locale {
                            Some(locale) => format!("{}", date_with_tz.format_localized(self.format.as_str(), *locale)),
                            None => format!("{}", date_with_tz.format(self.format.as_str())),
                        }
                        .as_str(),
                    );
                }

                if content.contains("{date}") {
                    content = content.replace(
                        "{date}",
                        match &self.locale {
                            Some(locale) => format!("{}", date_with_tz.format_localized(self.date_format.as_str(), *locale)),
                            None => format!("{}", date_with_tz.format(self.date_format.as_str())),
                        }
                        .as_str(),
                    );
                }

                if content.contains("{time}") {
                    content = content.replace(
                        "{time}",
                        match &self.locale {
                            Some(locale) => format!("{}", date_with_tz.format_localized(self.time_format.as_str(), *locale)),
                            None => format!("{}", date_with_tz.format(self.time_format.as_str())),
                        }
                        .as_str(),
                    );
                }

                (f, content)
            })
            .fold("".to_owned(), |acc, (f, content)| {
                format!("{acc}{}", f.format_string(&content))
            })
    }

    fn process_click(&self, _name: &str, _state: &ZellijState, _pos: usize) {}
}
