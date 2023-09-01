use std::collections::BTreeMap;

use zellij_tile::prelude::TabInfo;

use crate::{config::FormattedPart, render};

use super::widget::Widget;

pub struct TabsWidget {
    active_tab_format: FormattedPart,
    normal_tab_format: FormattedPart,
}

impl TabsWidget {
    pub fn new(config: BTreeMap<String, String>) -> Self {
        let mut normal_tab_format_string = "";
        if let Some(form) = config.get("tab_normal") {
            normal_tab_format_string = form;
        }

        let active_tab_format = FormattedPart::from_format_string(match config.get("tab_active") {
            Some(form) => form.to_string(),
            None => normal_tab_format_string.to_string(),
        });

        Self {
            normal_tab_format: FormattedPart::from_format_string(
                normal_tab_format_string.to_string(),
            ),
            active_tab_format,
        }
    }
}

impl Widget for TabsWidget {
    fn process(&self, state: crate::ZellijState) -> String {
        let mut output = "".to_string();

        for tab in state.tabs {
            let formatter = self.select_format(tab.clone());

            let mut content = formatter.content.clone();
            if content.contains("{name}") {
                content = content.replace("{name}", tab.name.as_str());
            }

            output = format!("{}{}", output, render::formatting(formatter, content),);
        }

        output
    }
}

impl TabsWidget {
    fn select_format(&self, info: TabInfo) -> FormattedPart {
        match info.active {
            false => self.normal_tab_format.clone(),
            true => self.active_tab_format.clone(),
        }
    }
}