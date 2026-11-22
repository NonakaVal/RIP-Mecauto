//! MecAuto (réplica) — entrada do programa.
//!
//! Uso:
//!   mecauto                     — abre o programa normalmente
//!   mecauto --screenshot arq.png — desenha ~120 quadros, salva o PNG da janela e sai
//!   mecauto --screenshot-products arq.png — captura o cadastro de produto e sai
//!   mecauto --screenshot-admin arq.png — captura Administração e sai

mod app;
mod data;
mod screens;
mod storage;

fn main() {
    let mut screenshot: Option<std::path::PathBuf> = None;
    let mut screenshot_products = false;
    let mut screenshot_admin = false;
    let mut args = std::env::args().skip(1);
    while let Some(a) = args.next() {
        if a == "--screenshot" {
            screenshot = args.next().map(std::path::PathBuf::from);
        } else if a == "--screenshot-products" {
            screenshot = args.next().map(std::path::PathBuf::from);
            screenshot_products = true;
        } else if a == "--screenshot-admin" {
            screenshot = args.next().map(std::path::PathBuf::from);
            screenshot_admin = true;
        }
    }

    let mut app = match app::App::new() {
        Ok(app) => app,
        Err(err) => {
            eprintln!("MecAuto não conseguiu abrir o banco de dados: {err}");
            std::process::exit(1);
        }
    };
    app.screenshot_path = screenshot;
    if screenshot_products {
        app.active = Some(app::Section::Produtos);
        app.form = Some(app::Form::NovoProduto);
    } else if screenshot_admin {
        app.active = Some(app::Section::Administracao);
    }

    let mut native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_title(app::APP_NAME)
            .with_inner_size(app::APP_WINDOW_SIZE),
        ..Default::default()
    };
    if app.screenshot_path.is_some() {
        // Abre sem levar o foco; ajuda o check a não atrapalhar o que a pessoa faz.
        native_options.viewport = native_options.viewport.with_active(false);
        #[cfg(target_os = "windows")]
        {
            native_options.viewport = native_options
                .viewport
                .with_window_level(egui::WindowLevel::AlwaysOnBottom);
        }
    }

    let app_creator: Box<
        dyn FnOnce(
            &eframe::CreationContext<'_>,
        )
            -> Result<Box<dyn eframe::App>, Box<dyn std::error::Error + Send + Sync>>,
    > = Box::new(|_cc| Ok(Box::new(app)));
    if let Err(err) = eframe::run_native(app::APP_NAME, native_options, app_creator) {
        eprintln!("MecAuto não foi possível abrir: {err}");
        std::process::exit(1);
    }
}
