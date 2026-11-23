//! Janela principal do MecAuto (réplica): barra de menus, telas e barra de status.

use crate::data::MecAutoData;
use crate::screens;
use crate::storage::{Storage, StorageResult};
use egui::{Color32, RichText};
use std::collections::HashMap;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Section {
    Clientes,
    Vendas,
    Oficina,
    Caixa,
    Produtos,
    Profissionais,
    Fornecedores,
    Financeiro,
    Administracao,
    Fiscal,
    Favoritos,
}

impl Section {
    pub const ALL: [Section; 11] = [
        Section::Clientes,
        Section::Vendas,
        Section::Oficina,
        Section::Caixa,
        Section::Produtos,
        Section::Profissionais,
        Section::Fornecedores,
        Section::Financeiro,
        Section::Administracao,
        Section::Fiscal,
        Section::Favoritos,
    ];

    pub fn title(&self) -> &'static str {
        match self {
            Section::Clientes => "Clientes",
            Section::Vendas => "Vendas",
            Section::Oficina => "Oficina",
            Section::Caixa => "Caixa",
            Section::Produtos => "Produtos",
            Section::Profissionais => "Profissionais",
            Section::Fornecedores => "Fornecedores",
            Section::Financeiro => "Financeiro",
            Section::Administracao => "Administração",
            Section::Fiscal => "Fiscal",
            Section::Favoritos => "Favoritos",
        }
    }

    pub fn from_title(title: &str) -> Option<Section> {
        Section::ALL.iter().find(|s| s.title() == title).copied()
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum Form {
    NovoCliente,
    Cliente(usize),
    NovoProduto,
    Produto(usize),
    VendaNova,
    Venda(usize),
    OSNova,
    OS(usize),
    CaixaBoxNova,
    CaixaBox(usize),
    NovoMovCaixa,
    MovCaixaEdit(usize),
    SaidaProduto(usize),
    NovoProfissional,
    Profissional(usize),
    NovoFornecedor,
    Fornecedor(usize),
    ContaBanc(usize, usize),
    NovoUsuario,
    Usuario(usize),
    NovoEvento,
    EventoFin(usize),
}

pub const APP_NAME: &str = "MecAuto";
/// Versão-info real do MecAuto original (FileVersion/ProductVersion, RT_VERSION @ 0x1911832c).
pub const APP_VERSION: &str = "1.0.0.0";
/// ProductID real do MecAuto original (version-info resource).
pub const PRODUCT_ID: &str = "com.embarcadero.MecAuto";
pub const APP_WINDOW_SIZE: [f32; 2] = [1440.0, 900.0];
/// Valores do status da barra inferior observados nos prints do MecAuto original
/// (estimados; OCR de reference/picture-2.png).
pub const STATUS_IP: &str = "192.168.0.39";
pub const LICENSE_EXPIRE: &str = "20/12/2026";
/// Linha de rodapé do original (estimado; OCR de reference/picture-2.png).
pub const FOOTER: &str = "Versão: 2026.11.04-B | 21/11/2026 14:36:04 | SERVIDOR | 0% | CICOM: (11) 3042-5369 | (11) 98232-3059 www.cicom.com.br";

pub struct App {
    pub data: MecAutoData,
    pub persisted_data: MecAutoData,
    pub storage: Storage,
    pub storage_error: Option<String>,
    pub last_save_attempt: Option<MecAutoData>,
    pub backup_path: String,
    pub import_path: String,
    pub restore_pending: bool,
    pub storage_notice: Option<String>,
    pub delete_confirmation: Option<Form>,
    /// Tela ativa: `None` = tela principal (home).
    pub active: Option<Section>,
    pub form: Option<Form>,
    pub frames: u64,
    pub screenshot_path: Option<std::path::PathBuf>,
    pub search: String,
    pub themed: bool,
    pub prod_tab: usize,
    pub product_draft: Option<crate::data::Produto>,
    pub product_drafts: HashMap<Form, crate::data::Produto>,
    pub search_focus: bool,
    pub form_drafts: HashMap<Form, FormDraft>,
    pub stock_exit_error: Option<String>,
}

impl App {
    pub fn new() -> StorageResult<Self> {
        let (storage, data) = Storage::open_default()?;
        let backup_path = storage.suggested_backup_path().display().to_string();
        Ok(App {
            persisted_data: data.clone(),
            storage,
            storage_error: None,
            last_save_attempt: None,
            backup_path,
            import_path: String::new(),
            restore_pending: false,
            storage_notice: None,
            delete_confirmation: None,
            data,
            active: None,
            form: None,
            frames: 0,
            screenshot_path: None,
            search: String::new(),
            themed: false,
            prod_tab: 0,
            product_draft: None,
            product_drafts: HashMap::new(),
            search_focus: false,
            form_drafts: HashMap::new(),
            stock_exit_error: None,
        })
    }

    pub fn take_form_draft(&mut self, form: Form, initial: FormDraft) -> FormDraft {
        self.form_drafts.remove(&form).unwrap_or(initial)
    }

    pub fn keep_form_draft(&mut self, form: Form, draft: FormDraft) {
        self.form_drafts.insert(form, draft);
    }

    pub fn clear_form_draft(&mut self) {
        self.form_drafts.clear();
        self.product_draft = None;
        self.product_drafts.clear();
        self.delete_confirmation = None;
    }

    pub fn create_backup(&mut self) {
        let destination = std::path::PathBuf::from(&self.backup_path);
        match self.storage.export_zip(&self.data, &destination) {
            Ok(()) => {
                self.storage_notice = Some(format!("Backup criado em {}", destination.display()));
                self.backup_path = self.storage.suggested_backup_path().display().to_string();
            }
            Err(error) => {
                self.storage_notice = Some(format!("Falha ao criar backup: {error}"));
            }
        }
    }

    pub fn restore_backup(&mut self) {
        let source = std::path::PathBuf::from(&self.import_path);
        let imported = match Storage::read_zip(&source) {
            Ok(data) => data,
            Err(error) => {
                self.storage_notice = Some(format!("Importação cancelada: {error}"));
                self.restore_pending = false;
                return;
            }
        };

        let backup = self.storage.suggested_backup_path();
        if let Err(error) = self.storage.export_zip(&self.data, &backup) {
            self.storage_notice = Some(format!(
                "Importação cancelada; não foi possível salvar o backup atual: {error}"
            ));
            self.restore_pending = false;
            return;
        }
        if let Err(error) = self.storage.save(&imported) {
            self.storage_notice = Some(format!(
                "Importação cancelada; falha ao gravar os dados: {error}"
            ));
            self.restore_pending = false;
            return;
        }

        self.data = imported;
        self.persisted_data = self.data.clone();
        self.last_save_attempt = None;
        self.storage_error = None;
        self.form = None;
        self.clear_form_draft();
        self.storage_notice = Some(format!(
            "Dados importados. Uma cópia anterior foi salva em {}",
            backup.display()
        ));
        self.restore_pending = false;
    }

    fn sidebar(&mut self, ui: &mut egui::Ui) {
        egui::Frame::new()
            .fill(Color32::WHITE)
            .inner_margin(egui::Margin::symmetric(14, 18))
            .show(ui, |ui| {
                ui.set_min_width(ui.available_width());
                ui.vertical(|ui| {
                    ui.horizontal(|ui| {
                        ui.label(RichText::new("M").size(26.0).strong().color(accent()));
                        ui.label(RichText::new("MecAuto").size(20.0).strong());
                    });
                    ui.label(
                        RichText::new("GESTÃO AUTOMOTIVA")
                            .size(10.0)
                            .strong()
                            .color(Color32::from_rgb(125, 137, 153)),
                    );
                    ui.add_space(24.0);

                    let dashboard_selected = self.active.is_none();
                    let dashboard = egui::Button::new(
                        RichText::new("▦   Dashboard").size(13.0).strong().color(
                            if dashboard_selected {
                                Color32::WHITE
                            } else {
                                Color32::from_rgb(54, 65, 81)
                            },
                        ),
                    )
                    .fill(if dashboard_selected {
                        accent()
                    } else {
                        Color32::TRANSPARENT
                    })
                    .stroke(egui::Stroke::NONE)
                    .min_size(egui::vec2(ui.available_width(), 40.0));
                    if ui.add(dashboard).clicked() {
                        self.active = None;
                        self.form = None;
                        self.clear_form_draft();
                    }

                    ui.add_space(18.0);
                    ui.label(
                        RichText::new("MENU PRINCIPAL")
                            .size(10.0)
                            .strong()
                            .color(Color32::from_rgb(125, 137, 153)),
                    );
                    ui.add_space(6.0);
                    for section in Section::ALL {
                        let selected = self.active == Some(section);
                        let button = egui::Button::new(
                            RichText::new(format!("   {}", section.title()))
                                .size(13.0)
                                .color(if selected {
                                    Color32::WHITE
                                } else {
                                    Color32::from_rgb(54, 65, 81)
                                }),
                        )
                        .fill(if selected {
                            accent()
                        } else {
                            Color32::TRANSPARENT
                        })
                        .stroke(egui::Stroke::NONE)
                        .min_size(egui::vec2(ui.available_width(), 36.0));
                        if ui.add(button).clicked() {
                            self.active = Some(section);
                            self.form = None;
                            self.clear_form_draft();
                        }
                    }
                    ui.with_layout(egui::Layout::bottom_up(egui::Align::Min), |ui| {
                        ui.separator();
                        ui.label(
                            RichText::new(format!("Versão {}", APP_VERSION))
                                .size(10.0)
                                .color(Color32::from_rgb(125, 137, 153)),
                        );
                    });
                });
            });
    }

    fn status_bar(&self, ui: &mut egui::Ui) {
        egui::Panel::bottom("mecauto_status")
            .exact_size(40.0)
            .show(ui, |ui| {
                ui.horizontal_wrapped(|ui| {
                    ui.add_space(6.0);
                    ui.label(RichText::new(format!("IP: {STATUS_IP}")).size(11.0));
                    ui.separator();
                    ui.label(RichText::new("LOG: Conectado").size(11.0));
                    ui.separator();
                    ui.label(RichText::new("FOTOS: Conectado").size(11.0));
                    ui.separator();
                    ui.label(RichText::new("ACESSOS SIMULTÂNEOS: 1").size(11.0));
                    ui.separator();
                    ui.label(
                        RichText::new(format!(
                            "A LICENÇA DE USO EXPIRA EM: {LICENSE_EXPIRE}, 30 dias restantes"
                        ))
                        .size(11.0),
                    );
                    ui.separator();
                    ui.label(
                        RichText::new(format!("PROD: {PRODUCT_ID} | {APP_VERSION}")).size(11.0),
                    );
                });
                ui.horizontal(|ui| {
                    ui.add_space(6.0);
                    ui.label(RichText::new(FOOTER).size(10.0).weak());
                });
            });
    }

    fn module_toolbar(&mut self, ui: &mut egui::Ui) {
        let items: &[(&str, &str, Option<ToolbarAction>)] = match self.active {
            Some(Section::Clientes) => &[
                ("＋", "Novo cliente", Some(ToolbarAction::NovoCliente)),
                ("⌕", "Pesquisar", None),
                ("▤", "Relatórios", None),
            ],
            Some(Section::Vendas) => &[
                ("＋", "Nova venda", Some(ToolbarAction::NovaVenda)),
                ("▤", "Vendas", None),
                ("⌕", "Pesquisar", None),
            ],
            Some(Section::Oficina) => &[
                ("＋", "Nova ordem", Some(ToolbarAction::NovaOrdem)),
                ("▤", "Ordens de serviço", None),
                ("▦", "Agenda", None),
            ],
            Some(Section::Caixa) => &[
                ("＋", "Novo caixa", Some(ToolbarAction::NovoCaixa)),
                ("↕", "Movimentações", Some(ToolbarAction::NovaMovimentacao)),
                ("▤", "Relatórios", None),
            ],
            Some(Section::Produtos) => &[
                ("▦", "Cad. de Produtos", Some(ToolbarAction::NovoProduto)),
                ("▤", "Relatórios", None),
                ("▣", "Controle de Pedidos", None),
                ("↓", "Entrada produtos", None),
                ("↩", "Devolução", None),
                ("⌖", "Localização", None),
                ("▧", "Montagem de kit", None),
                ("◉", "Mercado Livre", None),
                (
                    "⌕",
                    "F2 - Pesquisar",
                    Some(ToolbarAction::PesquisarProdutos),
                ),
                ("≡", "Ctrl. Requisições", None),
                ("⇧", "Importação", Some(ToolbarAction::AbrirImportacao)),
            ],
            Some(Section::Profissionais) => &[
                (
                    "＋",
                    "Novo profissional",
                    Some(ToolbarAction::NovoProfissional),
                ),
                ("▤", "Profissionais", None),
                ("⌕", "Pesquisar", None),
            ],
            Some(Section::Fornecedores) => &[
                ("＋", "Novo fornecedor", Some(ToolbarAction::NovoFornecedor)),
                ("▤", "Fornecedores", None),
                ("⌕", "Pesquisar", None),
            ],
            Some(Section::Financeiro) => &[
                ("＋", "Novo lançamento", Some(ToolbarAction::NovoEvento)),
                ("▤", "Contas e eventos", None),
                ("▦", "Relatórios", None),
            ],
            Some(Section::Administracao) => &[
                ("＋", "Novo usuário", Some(ToolbarAction::NovoUsuario)),
                ("⚙", "Configurações", None),
                ("⇧", "Backup", Some(ToolbarAction::CriarBackup)),
            ],
            Some(Section::Fiscal) => &[
                ("▤", "Documentos fiscais", None),
                ("⇧", "Importação", Some(ToolbarAction::AbrirImportacao)),
                ("▦", "Relatórios", None),
            ],
            Some(Section::Favoritos) => &[("☆", "Favoritos", None)],
            None => &[
                ("＋", "Novo produto", Some(ToolbarAction::NovoProduto)),
                ("▦", "Produtos", Some(ToolbarAction::CatalogoProdutos)),
                ("＋", "Nova venda", Some(ToolbarAction::NovaVenda)),
                ("＋", "Nova ordem", Some(ToolbarAction::NovaOrdem)),
            ],
        };

        egui::Panel::top("mecauto_module_toolbar")
            .exact_size(70.0)
            .show(ui, |ui| {
                egui::Frame::new()
                    .fill(Color32::from_rgb(247, 249, 252))
                    .stroke(egui::Stroke::new(1.0, Color32::from_rgb(225, 231, 239)))
                    .inner_margin(egui::Margin::symmetric(10, 7))
                    .show(ui, |ui| {
                        egui::ScrollArea::horizontal()
                            .id_salt("module_toolbar_scroll")
                            .auto_shrink([false, true])
                            .show(ui, |ui| {
                                ui.horizontal(|ui| {
                                    for (icon, label, action) in items {
                                        let button = egui::Button::new(
                                            RichText::new(format!("{icon}\n{label}")).size(11.0),
                                        )
                                        .min_size(egui::vec2(88.0, 52.0))
                                        .fill(Color32::WHITE);
                                        let response = ui.add_enabled(
                                            action.is_some(),
                                            button.stroke(egui::Stroke::new(
                                                1.0,
                                                Color32::from_rgb(220, 227, 236),
                                            )),
                                        );
                                        if response.clicked() {
                                            if let Some(action) = action {
                                                self.run_toolbar_action(*action);
                                            }
                                        }
                                        if action.is_none() {
                                            response
                                                .on_hover_text("Disponível em uma próxima etapa");
                                        }
                                    }
                                });
                            });
                    });
            });
    }

    fn run_toolbar_action(&mut self, action: ToolbarAction) {
        self.clear_form_draft();
        match action {
            ToolbarAction::NovoCliente => {
                self.active = Some(Section::Clientes);
                self.form = Some(Form::NovoCliente);
            }
            ToolbarAction::NovaVenda => {
                self.active = Some(Section::Vendas);
                self.form = Some(Form::VendaNova);
            }
            ToolbarAction::NovaOrdem => {
                self.active = Some(Section::Oficina);
                self.form = Some(Form::OSNova);
            }
            ToolbarAction::NovoCaixa => {
                self.active = Some(Section::Caixa);
                self.form = Some(Form::CaixaBoxNova);
            }
            ToolbarAction::NovaMovimentacao => {
                self.active = Some(Section::Caixa);
                self.form = Some(Form::NovoMovCaixa);
            }
            ToolbarAction::NovoProduto => {
                self.active = Some(Section::Produtos);
                self.form = Some(Form::NovoProduto);
                self.product_draft = None;
                self.prod_tab = 0;
            }
            ToolbarAction::CatalogoProdutos => {
                self.active = Some(Section::Produtos);
                self.form = None;
                self.product_draft = None;
            }
            ToolbarAction::PesquisarProdutos => {
                self.active = Some(Section::Produtos);
                self.form = None;
                self.search_focus = true;
            }
            ToolbarAction::NovoProfissional => {
                self.active = Some(Section::Profissionais);
                self.form = Some(Form::NovoProfissional);
            }
            ToolbarAction::NovoFornecedor => {
                self.active = Some(Section::Fornecedores);
                self.form = Some(Form::NovoFornecedor);
            }
            ToolbarAction::NovoEvento => {
                self.active = Some(Section::Financeiro);
                self.form = Some(Form::NovoEvento);
            }
            ToolbarAction::NovoUsuario => {
                self.active = Some(Section::Administracao);
                self.form = Some(Form::NovoUsuario);
            }
            ToolbarAction::CriarBackup => {
                self.active = Some(Section::Administracao);
                self.form = None;
                self.create_backup();
            }
            ToolbarAction::AbrirImportacao => {
                self.active = Some(Section::Administracao);
                self.form = None;
            }
        }
    }

    fn update(&mut self, ui: &mut egui::Ui) {
        if !self.themed {
            let mut visuals = egui::Visuals::light();
            let canvas = Color32::from_rgb(244, 246, 250);
            let surface = Color32::WHITE;
            let border = Color32::from_rgb(226, 231, 239);
            let text = Color32::from_rgb(32, 36, 43);
            let muted = Color32::from_rgb(107, 114, 128);
            let hover = Color32::from_rgb(235, 241, 255);
            visuals.panel_fill = canvas;
            visuals.window_fill = Color32::WHITE;
            visuals.faint_bg_color = Color32::from_rgb(244, 246, 250);
            visuals.extreme_bg_color = surface;
            visuals.selection.bg_fill = Color32::from_rgb(225, 235, 255);
            visuals.selection.stroke = egui::Stroke::new(1.0, accent());
            visuals.hyperlink_color = accent();
            visuals.widgets.noninteractive.bg_fill = surface;
            visuals.widgets.noninteractive.fg_stroke.color = text;
            visuals.widgets.noninteractive.bg_stroke = egui::Stroke::new(1.0, border);
            visuals.widgets.inactive.bg_fill = surface;
            visuals.widgets.inactive.weak_bg_fill = Color32::from_rgb(247, 249, 251);
            visuals.widgets.inactive.fg_stroke.color = text;
            visuals.widgets.inactive.bg_stroke = egui::Stroke::new(1.0, border);
            visuals.widgets.hovered.bg_fill = hover;
            visuals.widgets.hovered.fg_stroke.color = text;
            visuals.widgets.hovered.bg_stroke = egui::Stroke::new(1.0, accent());
            visuals.widgets.active.bg_fill = accent();
            visuals.widgets.active.fg_stroke.color = text;
            visuals.widgets.active.bg_stroke = egui::Stroke::new(1.0, accent());
            visuals.widgets.open.bg_fill = Color32::from_rgb(235, 241, 255);
            visuals.widgets.open.fg_stroke.color = text;
            visuals.widgets.open.bg_stroke = egui::Stroke::new(1.0, accent());
            visuals.window_stroke = egui::Stroke::new(1.0, border);
            visuals.widgets.noninteractive.corner_radius = egui::CornerRadius::same(8);
            visuals.widgets.inactive.corner_radius = egui::CornerRadius::same(8);
            visuals.widgets.hovered.corner_radius = egui::CornerRadius::same(8);
            visuals.widgets.active.corner_radius = egui::CornerRadius::same(8);
            visuals.widgets.open.corner_radius = egui::CornerRadius::same(8);
            visuals.striped = true;
            visuals.warn_fg_color = Color32::from_rgb(154, 83, 12);
            visuals.error_fg_color = Color32::from_rgb(180, 35, 24);
            visuals.weak_text_color = Some(muted);
            ui.ctx().set_visuals(visuals);
            let mut style = (**ui.style()).clone();
            style.spacing.item_spacing = egui::vec2(10.0, 9.0);
            style.spacing.button_padding = egui::vec2(12.0, 8.0);
            style.spacing.interact_size.y = 34.0;
            ui.set_style(style);
            self.themed = true;
        }
        egui::Panel::top("mecauto_header")
            .exact_size(52.0)
            .show(ui, |ui| {
                egui::Frame::new()
                    .fill(Color32::WHITE)
                    .inner_margin(egui::Margin::symmetric(20, 10))
                    .show(ui, |ui| {
                        ui.horizontal(|ui| {
                            ui.label(
                                RichText::new(
                                    self.active.map_or("Dashboard", |section| section.title()),
                                )
                                .size(15.0)
                                .strong(),
                            );
                            ui.with_layout(
                                egui::Layout::right_to_left(egui::Align::Center),
                                |ui| {
                                    ui.label(
                                        RichText::new("●  Operação local")
                                            .size(11.0)
                                            .color(Color32::from_rgb(0x00, 0xb8, 0x96)),
                                    );
                                },
                            );
                        });
                    });
            });
        self.module_toolbar(ui);
        self.status_bar(ui);
        egui::Panel::left("mecauto_sidebar")
            .exact_size(220.0)
            .show(ui, |ui| self.sidebar(ui));
        egui::CentralPanel::default()
            .frame(egui::Frame::new().fill(Color32::from_rgb(244, 246, 250)))
            .show(ui, |ui| {
                egui::Frame::new()
                    .fill(Color32::from_rgb(244, 246, 250))
                    .inner_margin(egui::Margin::same(18))
                    .show(ui, |ui| {
                        if let Some(error) = self.storage_error.clone() {
                            ui.horizontal(|ui| {
                                ui.colored_label(
                                    Color32::from_rgb(170, 52, 48),
                                    format!("Alterações ainda não salvas no banco: {error}"),
                                );
                                if ui.button("Tentar salvar novamente").clicked() {
                                    self.last_save_attempt = None;
                                    self.storage_error = None;
                                }
                            });
                        }
                        screens::show(self, ui);
                    });
            });
        self.persist_if_needed();
    }

    fn persist_if_needed(&mut self) {
        if self.data == self.persisted_data || self.last_save_attempt.as_ref() == Some(&self.data) {
            return;
        }
        self.last_save_attempt = Some(self.data.clone());
        match self.storage.save(&self.data) {
            Ok(()) => {
                self.persisted_data = self.data.clone();
                self.last_save_attempt = None;
                self.storage_error = None;
            }
            Err(error) => {
                self.storage_error = Some(error.to_string());
            }
        }
    }
}

#[derive(Clone, Copy)]
enum ToolbarAction {
    NovoCliente,
    NovaVenda,
    NovaOrdem,
    NovoCaixa,
    NovaMovimentacao,
    NovoProduto,
    CatalogoProdutos,
    PesquisarProdutos,
    NovoProfissional,
    NovoFornecedor,
    NovoEvento,
    NovoUsuario,
    CriarBackup,
    AbrirImportacao,
}

pub enum FormDraft {
    Cliente(crate::data::Cliente),
    Venda(crate::data::Venda),
    OrdemServico(crate::data::OS),
    Caixa(crate::data::BoxCaixa),
    MovimentoCaixa(crate::data::MovCaixa),
    Profissional(crate::data::Profissional),
    Fornecedor(crate::data::Fornecedor),
    ContaBancaria(crate::data::ContaBancaria),
    EventoFinanceiro(crate::data::EventoFin),
    Usuario(crate::data::Usuario),
    SaidaEstoque(StockExitDraft),
}

#[derive(Clone, Debug)]
pub struct StockExitDraft {
    pub quantidade: f64,
    pub observacao: String,
}

pub fn accent() -> Color32 {
    Color32::from_rgb(0x47, 0x7d, 0xff)
}

pub fn f2s<T: Into<f64> + std::fmt::Display>(v: T) -> String {
    let v = v.into();
    if v.fract().abs() < 0.005 {
        format!("{v:.0}")
    } else {
        format!("{v:.2}")
    }
}

impl eframe::App for App {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        self.frames += 1;
        if self.frames >= 120 {
            if let Some(path) = self.screenshot_path.take() {
                ui.ctx()
                    .request_screenshot(move |img: std::sync::Arc<egui::ColorImage>| {
                        save_screenshot_png(&path, &img);
                        std::process::exit(0);
                    });
            }
        }
        if self.screenshot_path.is_some() {
            ui.ctx().request_repaint();
        }
        self.update(ui);
    }
}

fn save_screenshot_png(path: &std::path::Path, img: &egui::ColorImage) {
    let mut rgba = Vec::with_capacity(img.size[0] * img.size[1] * 4);
    for c in &img.pixels {
        rgba.extend_from_slice(&c.to_srgba_unmultiplied());
    }
    let image = image::RgbaImage::from_raw(img.size[0] as u32, img.size[1] as u32, rgba)
        .expect("buffer de pixels com o tamanho do arquivo");
    if let Some(parent) = path.parent() {
        let _ = std::fs::create_dir_all(parent);
    }
    if let Err(err) = image.save(path) {
        eprintln!(
            "Não foi possível salvar o screenshot em {}: {err}",
            path.display()
        );
    }
}
