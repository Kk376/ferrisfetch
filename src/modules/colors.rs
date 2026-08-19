use crate::context::FetchContext;
use crate::modules::{Collector, ModuleId, ModuleOutput};

pub fn render_color_palette() -> String {
    let mut standard = String::new();
    for code in 30..=37 {
        standard.push_str(&format!("\x1b[{}m███\x1b[0m ", code));
    }

    let mut bright = String::new();
    for code in 90..=97 {
        bright.push_str(&format!("\x1b[{}m███\x1b[0m ", code));
    }

    format!("{}\n{}", standard.trim_end(), bright.trim_end())
}

pub struct ColorsCollector;

impl Collector for ColorsCollector {
    fn id(&self) -> ModuleId {
        ModuleId::Colors
    }

    fn collect(&self, ctx: &FetchContext) -> Option<ModuleOutput> {
        if !ctx.enable_color {
            return None;
        }

        Some(ModuleOutput {
            id: ModuleId::Colors,
            label: String::new(),
            value: String::new(),
            custom_rendered: Some(render_color_palette()),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_render_color_palette() {
        let palette = render_color_palette();
        assert!(palette.contains("\x1b[30m"));
        assert!(palette.contains("\x1b[97m"));
        assert!(palette.contains('\n'));
    }
}
