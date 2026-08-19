use clap::Parser;
use ferrisfetch::cli::Cli;
use ferrisfetch::context::FetchContext;
use ferrisfetch::modules::{ModuleId, ModuleRegistry};
use ferrisfetch::output::formatter::{render_json, render_layout};
use ferrisfetch::output::logo::match_logo;

fn main() {
    let cli = Cli::parse();

    if cli.list_modules {
        for module in ModuleId::all() {
            println!("{}", module.as_str());
        }
        return;
    }

    let ctx = FetchContext::new(&cli);
    let registry = ModuleRegistry::new();
    let outputs = registry.collect_all(&ctx);

    if cli.json {
        println!("{}", render_json(&outputs));
        return;
    }

    let logo = if ctx.no_logo {
        None
    } else {
        match_logo(
            ctx.logo_override.as_deref(),
            &ctx.os_info.distro_id,
            &ctx.os_info.distro_like,
        )
    };

    let rendered = render_layout(logo, &outputs, ctx.term_width, ctx.enable_color);
    if !rendered.is_empty() {
        println!("{}", rendered);
    }
}
