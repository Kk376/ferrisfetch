use clap::Parser;
use ferrisfetch::cli::Cli;
use ferrisfetch::context::FetchContext;
use ferrisfetch::modules::{ModuleId, ModuleRegistry};
use ferrisfetch::output::formatter::{render_json, render_layout};
use ferrisfetch::output::logo::match_logo;

fn main() {
    let cli = Cli::parse();

    // Early exit for shell completion scripts or discovery tooling
    if cli.list_modules {
        for module in ModuleId::all() {
            println!("{}", module.as_str());
        }
        return;
    }

    // Initialize execution context once to share terminal dimensions and OS release metadata
    let ctx = FetchContext::new(&cli);
    let registry = ModuleRegistry::new();
    let outputs = registry.collect_all(&ctx);

    // JSON export mode skips ANSI styling and ASCII logo formatting entirely
    if cli.json {
        println!("{}", render_json(&outputs));
        return;
    }

    // Resolve distro ASCII art using explicit CLI override, distro ID, or ID_LIKE fallback
    let logo = if ctx.no_logo {
        None
    } else {
        match_logo(
            ctx.logo_override.as_deref(),
            &ctx.os_info.distro_id,
            &ctx.os_info.distro_like,
        )
    };

    // Format side-by-side or stacked layout depending on terminal column width
    let rendered = render_layout(logo, &outputs, ctx.term_width, ctx.enable_color);
    if !rendered.is_empty() {
        println!("{}", rendered);
    }
}
