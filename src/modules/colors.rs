use crate::context::FetchContext;
use crate::modules::{Collector, ModuleId, ModuleOutput};

pub fn render_color_palette() -> String {
    let mut standard = String::new();
    for code in 40..=47 {
        standard.push_str(&format!("\x1b[{}m   ", code));
    }
    standard.push_str("\x1b[0m");

    let mut bright = String::new();
    for code in 100..=107 {
        bright.push_str(&format!("\x1b[{}m   ", code));
    }
    bright.push_str("\x1b[0m");

    format!("{}\n{}", standard, bright)
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
        assert!(palette.contains("\x1b[40m"));
        assert!(palette.contains("\x1b[107m"));
        assert!(palette.contains('\n'));
    }
}
