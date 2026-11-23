//! Telas dos módulos do MecAuto (réplica).
//!
//! Estrutura base de cada tela: barra de ferramentas ("Novo ..." e buscas),
//! tabela com os registros do módulo (Grid striped) e formulário para gravar
//! um novo registro ou alterar o selecionado.

use crate::app::{accent, f2s, App, Form, FormDraft, Section, StockExitDraft};
use crate::data::*;
use egui::{Color32, RichText, ScrollArea, Ui};

pub fn show(app: &mut App, ui: &mut Ui) {
    match app.active {
        None => home(app, ui),
        Some(Section::Clientes) => clientes(app, ui),
        Some(Section::Vendas) => vendas(app, ui),
        Some(Section::Oficina) => oficina(app, ui),
        Some(Section::Caixa) => caixa(app, ui),
        Some(Section::Produtos) => produtos(app, ui),
        Some(Section::Profissionais) => profissionais(app, ui),
        Some(Section::Fornecedores) => fornecedores(app, ui),
        Some(Section::Financeiro) => financeiro(app, ui),
        Some(Section::Administracao) => administra(app, ui),
        Some(Section::Fiscal) => fiscal(app, ui),
        Some(Section::Favoritos) => favoritos(app, ui),
    }
}

// ---------------------------------------------------------------- principal

/// Resumo operacional com a hierarquia de cartões do dashboard de referência.
fn home(app: &mut App, ui: &mut Ui) {
    ui.heading("Visão geral");
    ui.label(
        RichText::new("Acompanhe o estoque e a operação da sua oficina.")
            .color(Color32::from_rgb(107, 114, 128)),
    );
    ui.add_space(16.0);

    let products = &app.data.produtos;
    let total_stock: f64 = products.iter().map(|product| product.estoque).sum();
    let low_stock = products
        .iter()
        .filter(|product| !product.inativo && product.estoque <= product.estoque_minimo)
        .count();
    let stock_value: f64 = products
        .iter()
        .map(|product| product.estoque * f64::from(product.preco))
        .sum();

    ui.columns(4, |columns| {
        metric_card(
            &mut columns[0],
            "Produtos cadastrados",
            products.len().to_string(),
            "Itens ativos e inativos",
            Color32::from_rgb(0x47, 0x7d, 0xff),
        );
        metric_card(
            &mut columns[1],
            "Unidades em estoque",
            f2s(total_stock),
            "Saldo total disponível",
            Color32::from_rgb(0x00, 0xb8, 0x96),
        );
        metric_card(
            &mut columns[2],
            "Estoque baixo",
            low_stock.to_string(),
            "Itens no mínimo ou abaixo",
            Color32::from_rgb(0xff, 0x9f, 0x43),
        );
        metric_card(
            &mut columns[3],
            "Valor em estoque",
            format!("R$ {}", f2s(stock_value)),
            "Pelo preço de venda",
            Color32::from_rgb(0x8b, 0x5c, 0xf6),
        );
    });

    ui.add_space(14.0);
    card(ui, |ui| {
        ui.label(RichText::new("Reposição necessária").size(16.0).strong());
        ui.label(
            RichText::new("Produtos no estoque mínimo ou abaixo.")
                .color(Color32::from_rgb(107, 114, 128)),
        );
        ui.add_space(8.0);
        egui::Grid::new("dashboard_low_stock")
            .striped(true)
            .show(ui, |ui| {
                headers(ui, &["Produto", "Disponível", "Mínimo"]);
                let mut found = false;
                for product in products
                    .iter()
                    .filter(|product| !product.inativo && product.estoque <= product.estoque_minimo)
                    .take(6)
                {
                    found = true;
                    ui.label(&product.descricao);
                    ui.colored_label(Color32::from_rgb(0xff, 0x9f, 0x43), f2s(product.estoque));
                    ui.label(f2s(product.estoque_minimo));
                    ui.end_row();
                }
                if !found {
                    ui.label("Nenhum item precisa de reposição.");
                    ui.end_row();
                }
            });
    });
    ui.add_space(12.0);
    card(ui, |ui| {
        ui.label(RichText::new("Movimentações recentes").size(16.0).strong());
        ui.label(
            RichText::new("Últimas saídas registradas no estoque.")
                .color(Color32::from_rgb(107, 114, 128)),
        );
        ui.add_space(8.0);
        egui::Grid::new("dashboard_stock_movements")
            .striped(true)
            .show(ui, |ui| {
                headers(ui, &["Data", "Produto", "Tipo", "Quantidade"]);
                let mut found = false;
                for movement in app.data.movimentos_estoque.iter().rev().take(6) {
                    found = true;
                    ui.label(&movement.data);
                    ui.label(&movement.produto_descricao);
                    ui.colored_label(Color32::from_rgb(0x47, 0x7d, 0xff), &movement.tipo);
                    ui.label(f2s(movement.quantidade));
                    ui.end_row();
                }
                if !found {
                    ui.label("Ainda não há movimentações de estoque.");
                    ui.end_row();
                }
            });
    });
}

fn metric_card(ui: &mut Ui, title: &str, value: String, detail: &str, color: Color32) {
    card(ui, |ui| {
        ui.set_min_height(102.0);
        ui.horizontal(|ui| {
            ui.vertical(|ui| {
                ui.label(
                    RichText::new(title)
                        .size(12.0)
                        .color(Color32::from_rgb(107, 114, 128)),
                );
                ui.label(RichText::new(value).size(23.0).strong());
            });
        });
        ui.label(RichText::new(detail).size(11.0).color(color));
    });
}

// ---------------------------------------------------------------- helpers

fn heading(ui: &mut Ui, texto: &str) {
    ui.heading(texto);
    ui.add_space(6.0);
}

fn card_frame() -> egui::Frame {
    egui::Frame::new()
        .fill(Color32::WHITE)
        .stroke(egui::Stroke::new(1.0, Color32::from_rgb(203, 213, 225)))
        .corner_radius(egui::CornerRadius::same(12))
        .inner_margin(egui::Margin::same(16))
}

fn card(ui: &mut Ui, contents: impl FnOnce(&mut Ui)) {
    card_frame().show(ui, |ui| {
        let width = ui.available_width();
        ui.set_min_width(width);
        contents(ui);
    });
}

fn section_caption(ui: &mut Ui, title: &str, description: &str) {
    ui.label(RichText::new(title).size(15.0).strong());
    ui.label(
        RichText::new(description)
            .size(11.0)
            .color(Color32::from_rgb(103, 116, 132)),
    );
}

fn text(ui: &mut Ui, s: &mut String, label: &str) {
    ui.horizontal(|ui| {
        ui.label(RichText::new(label).strong());
        ui.add(egui::TextEdit::singleline(s).desired_width(240.0));
    });
}

fn text_w(ui: &mut Ui, s: &mut String, label: &str, width: f32) {
    ui.horizontal(|ui| {
        ui.label(RichText::new(label).strong());
        ui.add(egui::TextEdit::singleline(s).desired_width(width));
    });
}

fn num(ui: &mut Ui, v: &mut f32, label: &str) {
    ui.horizontal(|ui| {
        ui.label(RichText::new(label).strong());
        ui.add(egui::DragValue::new(v).max_decimals(2).speed(0.25));
    });
}

fn num_dec(ui: &mut Ui, v: &mut f64, label: &str) {
    ui.horizontal(|ui| {
        ui.label(RichText::new(label).strong());
        ui.add(egui::DragValue::new(v).max_decimals(3).speed(0.5));
    });
}

fn check(ui: &mut Ui, b: &mut bool, label: &str) {
    ui.horizontal(|ui| {
        ui.checkbox(b, RichText::new(label).strong());
    });
}

fn combo<S: ToString>(ui: &mut Ui, id: &str, label: &str, current: &mut String, options: &[S]) {
    ui.horizontal(|ui| {
        ui.label(RichText::new(label).strong());
        egui::ComboBox::from_id_salt(id)
            .selected_text(RichText::new(current.clone()).color(if current.is_empty() {
                Color32::from_rgb(0x99, 0x99, 0x99)
            } else {
                Color32::WHITE
            }))
            .show_ui(ui, |ui| {
                if options.is_empty() {
                    ui.label("(nenhuma opção)");
                }
                for o in options {
                    let s = o.to_string();
                    ui.selectable_value(current, s.clone(), s);
                }
            });
    });
}

fn abas(ui: &mut Ui, abas: &[&str], atual: &mut usize) {
    ui.horizontal_wrapped(|ui| {
        for (i, a) in abas.iter().enumerate() {
            if ui
                .selectable_label(*atual == i, RichText::new(*a).strong().size(11.0))
                .clicked()
            {
                *atual = i;
            }
        }
    });
    ui.add_space(2.0);
}

fn headers(ui: &mut Ui, cols: &[&str]) {
    for c in cols {
        ui.label(RichText::new(*c).strong().color(accent()));
    }
    ui.end_row();
}

fn campos_item(ui: &mut Ui, it: &mut Item) {
    ui.horizontal(|ui| {
        text(ui, &mut it.descricao, "Descrição");
        num_dec(ui, &mut it.qtd, "Qtde");
    });
    ui.horizontal(|ui| {
        num(ui, &mut it.valor, "Valor unit");
    });
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum FormAction {
    Save,
    Cancel,
    None,
}

fn gravar_cancela(ui: &mut Ui) -> FormAction {
    ui.horizontal(|ui| {
        if ui.button("Gravar").clicked() {
            FormAction::Save
        } else if ui.button("Cancelar").clicked() {
            FormAction::Cancel
        } else {
            FormAction::None
        }
    })
    .inner
}

fn next_id(prefix: &str, ids: impl Iterator<Item = String>) -> String {
    let next = ids
        .filter_map(|id| {
            id.strip_prefix(prefix)
                .and_then(|suffix| suffix.strip_prefix('-'))
                .and_then(|suffix| suffix.parse::<u64>().ok())
        })
        .max()
        .unwrap_or(0)
        + 1;
    format!("{prefix}-{next:03}")
}

fn next_product_code(codes: impl Iterator<Item = String>) -> String {
    let next = codes
        .filter_map(|code| code.parse::<u64>().ok())
        .max()
        .unwrap_or(0)
        + 1;
    format!("{next:03}")
}

// ---------------------------------------------------------------- clientes

fn clientes(app: &mut App, ui: &mut Ui) {
    ui.horizontal(|ui| {
        if ui.button("Novo Cliente").clicked() {
            app.form = Some(Form::NovoCliente);
        }
        ui.add_space(10.0);
        ui.label("Pesquisa Rápida");
        ui.add(
            egui::TextEdit::singleline(&mut app.search)
                .hint_text("Pesquisar por nome")
                .desired_width(300.0),
        );
    });
    ui.add_space(6.0);
    let clientes = app.data.clientes.clone();
    let busca = app.search.to_lowercase();
    ScrollArea::vertical().show(ui, |ui| {
        egui::Grid::new("grid_clientes")
            .striped(true)
            .show(ui, |ui| {
                headers(ui, &["Código", "Nome", "Telefone", "E-mail", "Saldo", ""]);
                for (i, c) in clientes.iter().enumerate() {
                    if !busca.is_empty() && !c.nome.to_lowercase().contains(&busca) {
                        continue;
                    }
                    ui.label(&c.id);
                    ui.label(&c.nome);
                    ui.label(&c.telefone);
                    ui.label(&c.email);
                    ui.label(f2s(c.saldo));
                    ui.horizontal(|ui| {
                        if ui.button("Detalhes").clicked() {
                            app.form = Some(Form::Cliente(i));
                        }
                        if ui.button("Excluir").clicked() {
                            app.data.clientes.remove(i);
                        }
                    });
                    ui.end_row();
                }
            });
    });
    if let Some(Form::NovoCliente) = app.form {
        clientes_form(app, ui, None);
    }
    if let Some(Form::Cliente(i)) = app.form {
        clientes_form(app, ui, Some(i));
    }
}

fn clientes_form(app: &mut App, ui: &mut Ui, i: Option<usize>) {
    heading(
        ui,
        if i.is_some() {
            "Cliente — Gravar / Alterar"
        } else {
            "Novo Cliente"
        },
    );
    let form = i.map(Form::Cliente).unwrap_or(Form::NovoCliente);
    let initial = match i {
        Some(i) => app.data.clientes[i].clone(),
        None => Cliente {
            id: next_id(
                "C",
                app.data.clientes.iter().map(|record| record.id.clone()),
            ),
            ..Default::default()
        },
    };
    let mut c = match app.take_form_draft(form, FormDraft::Cliente(initial)) {
        FormDraft::Cliente(c) => c,
        _ => unreachable!(),
    };
    campos_cliente(ui, &mut c);
    match gravar_cancela(ui) {
        FormAction::Save => {
            match i {
                Some(i) => app.data.clientes[i] = c,
                None => app.data.clientes.push(c),
            }
            app.clear_form_draft();
            app.form = None;
        }
        FormAction::Cancel => {
            app.clear_form_draft();
            app.form = None;
        }
        FormAction::None => app.keep_form_draft(form, FormDraft::Cliente(c)),
    }
}

fn campos_cliente(ui: &mut Ui, c: &mut Cliente) {
    text(ui, &mut c.nome, "Nome");
    ui.horizontal(|ui| {
        text(ui, &mut c.telefone, "Telefone");
        text(ui, &mut c.email, "E-mail");
    });
    text_w(ui, &mut c.endereco, "Endereço", 330.0);
    ui.horizontal(|ui| {
        text(ui, &mut c.bairro, "Bairro");
        text(ui, &mut c.cidade, "Cidade");
        text_w(ui, &mut c.uf, "UF", 40.0);
        text_w(ui, &mut c.cep, "CEP", 80.0);
    });
    if ui.small_button("Adicionar Grupo").clicked() {
        c.grupos.push("Grupo novo".into());
    }
    ui.label("Grupos:");
    for g in c.grupos.iter_mut() {
        ui.add(egui::Button::new(
            egui::RichText::new(g.clone()).color(accent()),
        ));
    }
    num(ui, &mut c.saldo, "Saldo");
    check(ui, &mut c.fotos, "Fotos do Veículo");
}

// ---------------------------------------------------------------- vendas

fn vendas(app: &mut App, ui: &mut Ui) {
    heading(ui, "Vendas");
    ui.horizontal(|ui| {
        if ui.button("Nova Venda").clicked() {
            app.form = Some(Form::VendaNova);
        }
    });
    let vendas = app.data.vendas.clone();
    ScrollArea::vertical().show(ui, |ui| {
        egui::Grid::new("grid_vendas").striped(true).show(ui, |ui| {
            headers(ui, &["Nº", "Cliente", "Veículo", "Data", "Total", ""]);
            for (i, v) in vendas.iter().enumerate() {
                ui.label(&v.id);
                ui.label(&v.cliente);
                ui.label(&v.veiculo);
                ui.label(&v.data);
                ui.label(f2s(v.total));
                if ui.button("Detalhes").clicked() {
                    app.form = Some(Form::Venda(i));
                }
                ui.end_row();
            }
        });
    });
    if let Some(Form::VendaNova) = app.form {
        vendas_form(app, ui, None);
    }
    if let Some(Form::Venda(i)) = app.form {
        vendas_form(app, ui, Some(i));
    }
}

fn vendas_form(app: &mut App, ui: &mut Ui, i: Option<usize>) {
    heading(
        ui,
        if i.is_some() {
            "Venda — Gravar / Alterar"
        } else {
            "Nova Venda"
        },
    );
    let clientes: Vec<String> = app.data.clientes.iter().map(|c| c.nome.clone()).collect();
    let veiculos: Vec<String> = app.data.veiculos.iter().map(|v| v.placa.clone()).collect();
    let form = i.map(Form::Venda).unwrap_or(Form::VendaNova);
    let initial = match i {
        Some(i) => app.data.vendas[i].clone(),
        None => Venda {
            status: "Aberta".into(),
            ..Default::default()
        },
    };
    let mut v = match app.take_form_draft(form, FormDraft::Venda(initial)) {
        FormDraft::Venda(v) => v,
        _ => unreachable!(),
    };
    combo(ui, "v_cliente", "Cliente", &mut v.cliente, &clientes);
    combo(ui, "v_veiculo", "Veículo", &mut v.veiculo, &veiculos);
    let status = [
        "Aberta".to_string(),
        "Concluída".to_string(),
        "Cancelada".to_string(),
    ];
    combo(ui, "v_status", "Status", &mut v.status, &status);
    ui.add_space(4.0);
    ui.label(RichText::new("Itens do Produto").strong());
    ScrollArea::vertical().max_height(170.0).show(ui, |ui| {
        egui::Grid::new("venda_itens").striped(true).show(ui, |ui| {
            headers(ui, &["Descrição", "Unidade", "Qtde", "Valor unit", "Total"]);
            for it in v.itens.iter_mut() {
                campos_item(ui, it);
                ui.label(f2s(it.total()));
                ui.end_row();
            }
        });
    });
    if ui.small_button("+").clicked() {
        v.itens.push(Item::default());
    }
    ui.add_space(4.0);
    ui.label(RichText::new("Serviço / Labor").strong());
    ScrollArea::vertical().max_height(120.0).show(ui, |ui| {
        egui::Grid::new("venda_labor").striped(true).show(ui, |ui| {
            headers(ui, &["Descrição", "Unidade", "Qtde", "Valor unit", "Total"]);
            for it in v.labor.iter_mut() {
                campos_item(ui, it);
                ui.label(f2s(it.total()));
                ui.end_row();
            }
        });
    });
    if ui.small_button("+").clicked() {
        v.labor.push(Item::default());
    }
    v.subtotal = v.itens.iter().map(|i| i.total()).sum::<f32>()
        + v.labor.iter().map(|i| i.total()).sum::<f32>();
    v.total = v.subtotal + v.icms_total + v.ipi_total;
    ui.horizontal(|ui| {
        ui.label("Subtotal");
        ui.label(f2s(v.subtotal));
        ui.separator();
        ui.label("Total");
        ui.label(RichText::new(f2s(v.total)).strong().color(accent()));
        ui.separator();
        ui.checkbox(&mut v.pagou_pix, "Pagar com Pix");
    });
    match gravar_cancela(ui) {
        FormAction::Save => {
            match i {
                Some(i) => app.data.vendas[i] = v,
                None => {
                    v.id = next_id("V", app.data.vendas.iter().map(|record| record.id.clone()));
                    v.data = today();
                    app.data.vendas.push(v);
                }
            }
            app.clear_form_draft();
            app.form = None;
        }
        FormAction::Cancel => {
            app.clear_form_draft();
            app.form = None;
        }
        FormAction::None => app.keep_form_draft(form, FormDraft::Venda(v)),
    }
}

// ---------------------------------------------------------------- oficina

fn oficina(app: &mut App, ui: &mut Ui) {
    heading(ui, "Oficina");
    ui.horizontal(|ui| {
        if ui.button("Nova Ordem").clicked() {
            app.form = Some(Form::OSNova);
        }
    });
    let ordens = app.data.ordens.clone();
    ScrollArea::vertical().show(ui, |ui| {
        egui::Grid::new("grid_ordens").striped(true).show(ui, |ui| {
            headers(
                ui,
                &["Nº", "Cliente", "Veículo", "Status", "Aberta", "Total", ""],
            );
            for (i, o) in ordens.iter().enumerate() {
                ui.label(&o.id);
                ui.label(&o.cliente);
                ui.label(&o.veiculo);
                ui.label(&o.status);
                ui.label(&o.data_abertura);
                ui.label(f2s(o.total));
                if ui.button("Detalhes").clicked() {
                    app.form = Some(Form::OS(i));
                }
                ui.end_row();
            }
        });
    });
    if let Some(Form::OSNova) = app.form {
        ordem_servico_form(app, ui, None);
    }
    if let Some(Form::OS(i)) = app.form {
        ordem_servico_form(app, ui, Some(i));
    }
}

fn ordem_servico_form(app: &mut App, ui: &mut Ui, i: Option<usize>) {
    heading(
        ui,
        if i.is_some() {
            "Ordem de Serviço — Gravar / Alterar"
        } else {
            "Nova Ordem de Serviço"
        },
    );
    let clientes: Vec<String> = app.data.clientes.iter().map(|c| c.nome.clone()).collect();
    let veiculos: Vec<String> = app.data.veiculos.iter().map(|v| v.placa.clone()).collect();
    let tecnicos: Vec<String> = app
        .data
        .profissionais
        .iter()
        .map(|p| p.nome.clone())
        .collect();
    let form = i.map(Form::OS).unwrap_or(Form::OSNova);
    let initial = match i {
        Some(i) => app.data.ordens[i].clone(),
        None => OS {
            status: "Aberta".into(),
            ..Default::default()
        },
    };
    let mut o = match app.take_form_draft(form, FormDraft::OrdemServico(initial)) {
        FormDraft::OrdemServico(o) => o,
        _ => unreachable!(),
    };
    combo(ui, "o_cliente", "Cliente", &mut o.cliente, &clientes);
    combo(ui, "o_veiculo", "Veículo", &mut o.veiculo, &veiculos);
    combo(ui, "o_tecnico", "Técnico", &mut o.tecnico, &tecnicos);
    let status = [
        "Aberta".to_string(),
        "Em Serviço".to_string(),
        "Concluída".to_string(),
    ];
    combo(ui, "o_status", "Status", &mut o.status, &status);
    ui.add_space(4.0);
    ui.label(RichText::new("Itens (Manutenção e Materiais)").strong());
    ScrollArea::vertical().max_height(150.0).show(ui, |ui| {
        egui::Grid::new("os_itens").striped(true).show(ui, |ui| {
            headers(ui, &["Descrição", "Unidade", "Qtde", "Valor unit", "Total"]);
            for it in o.itens.iter_mut() {
                campos_item(ui, it);
                ui.label(f2s(it.total()));
                ui.end_row();
            }
        });
    });
    if ui.small_button("+").clicked() {
        o.itens.push(Item::default());
    }
    text_w(ui, &mut o.obs, "Observações", 330.0);
    o.total = o.itens.iter().map(|i| i.total()).sum();
    ui.horizontal(|ui| {
        ui.label("Total");
        ui.label(RichText::new(f2s(o.total)).strong().color(accent()));
        ui.separator();
        ui.checkbox(&mut o.pagou_pix, "Pagar com Pix");
    });
    match gravar_cancela(ui) {
        FormAction::Save => {
            match i {
                Some(i) => app.data.ordens[i] = o,
                None => {
                    o.id = next_id("OS", app.data.ordens.iter().map(|record| record.id.clone()));
                    o.data_abertura = today();
                    app.data.ordens.push(o);
                }
            }
            app.clear_form_draft();
            app.form = None;
        }
        FormAction::Cancel => {
            app.clear_form_draft();
            app.form = None;
        }
        FormAction::None => app.keep_form_draft(form, FormDraft::OrdemServico(o)),
    }
}

// ---------------------------------------------------------------- caixa

fn caixa(app: &mut App, ui: &mut Ui) {
    heading(ui, "Caixa");
    ui.horizontal(|ui| {
        if ui.button("Abrir Caixa").clicked() {
            app.form = Some(Form::CaixaBoxNova);
        }
        if ui.button("Fechar Caixa").clicked() {
            for b in app.data.caixas.iter_mut() {
                b.aberto = false;
            }
        }
        if ui.button("Nova Entrada").clicked() {
            app.form = Some(Form::NovoMovCaixa);
        }
    });
    ui.add_space(6.0);
    let caixas = app.data.caixas.clone();
    ScrollArea::vertical().show(ui, |ui| {
        egui::Grid::new("grid_caixas").striped(true).show(ui, |ui| {
            headers(ui, &["Caixa", "Descrição", "Aberto", "Valor inicial", ""]);
            for (i, b) in caixas.iter().enumerate() {
                ui.label(&b.id);
                ui.label(&b.descricao);
                ui.label(if b.aberto { "Sim" } else { "Não" });
                ui.label(f2s(b.valor_inicial));
                if ui.button("Editar").clicked() {
                    app.form = Some(Form::CaixaBox(i));
                }
                ui.end_row();
            }
        });
    });
    ui.add_space(10.0);
    ui.label(RichText::new("Movimentações").strong());
    let movs = app.data.movimentos.clone();
    ScrollArea::vertical().max_height(260.0).show(ui, |ui| {
        egui::Grid::new("grid_movcaixa")
            .striped(true)
            .show(ui, |ui| {
                headers(ui, &["Data", "Tipo", "Descrição", "Valor", "Caixa", ""]);
                for (i, m) in movs.iter().enumerate() {
                    ui.label(&m.data);
                    ui.label(&m.tipo);
                    ui.label(&m.descricao);
                    ui.label(f2s(m.valor));
                    ui.label(&m.caixa);
                    if ui.button("Editar").clicked() {
                        app.form = Some(Form::MovCaixaEdit(i));
                    }
                    ui.end_row();
                }
            });
    });
    if let Some(Form::CaixaBoxNova) = app.form {
        caixa_box_form(app, ui, None);
    }
    if let Some(Form::CaixaBox(i)) = app.form {
        caixa_box_form(app, ui, Some(i));
    }
    if let Some(Form::NovoMovCaixa) = app.form {
        mov_caixa_form(app, ui, None);
    }
    if let Some(Form::MovCaixaEdit(i)) = app.form {
        mov_caixa_form(app, ui, Some(i));
    }
}

fn caixa_box_form(app: &mut App, ui: &mut Ui, i: Option<usize>) {
    heading(
        ui,
        if i.is_some() {
            "Caixa — Alterar"
        } else {
            "Abrir Caixa"
        },
    );
    let form = i.map(Form::CaixaBox).unwrap_or(Form::CaixaBoxNova);
    let mut initial = match i {
        Some(i) => app.data.caixas[i].clone(),
        None => BoxCaixa::default(),
    };
    if i.is_none() {
        initial.id = next_id("CX", app.data.caixas.iter().map(|record| record.id.clone()));
    }
    let mut b = match app.take_form_draft(form, FormDraft::Caixa(initial)) {
        FormDraft::Caixa(b) => b,
        _ => unreachable!(),
    };
    text(ui, &mut b.descricao, "Descrição");
    num(ui, &mut b.valor_inicial, "Valor inicial");
    check(ui, &mut b.aberto, "Aberto");
    match gravar_cancela(ui) {
        FormAction::Save => {
            match i {
                Some(i) => app.data.caixas[i] = b,
                None => app.data.caixas.push(b),
            }
            app.clear_form_draft();
            app.form = None;
        }
        FormAction::Cancel => {
            app.clear_form_draft();
            app.form = None;
        }
        FormAction::None => app.keep_form_draft(form, FormDraft::Caixa(b)),
    }
}

fn mov_caixa_form(app: &mut App, ui: &mut Ui, i: Option<usize>) {
    heading(
        ui,
        if i.is_some() {
            "Movimentação — Gravar / Alterar"
        } else {
            "Nova Entrada"
        },
    );
    let boxes: Vec<String> = app
        .data
        .caixas
        .iter()
        .map(|b| b.descricao.clone())
        .collect();
    let form = i.map(Form::MovCaixaEdit).unwrap_or(Form::NovoMovCaixa);
    let initial = match i {
        Some(i) => app.data.movimentos[i].clone(),
        None => MovCaixa {
            id: next_id(
                "MV",
                app.data.movimentos.iter().map(|record| record.id.clone()),
            ),
            data: today(),
            tipo: "Entrada".into(),
            ..Default::default()
        },
    };
    let mut m = match app.take_form_draft(form, FormDraft::MovimentoCaixa(initial)) {
        FormDraft::MovimentoCaixa(m) => m,
        _ => unreachable!(),
    };
    let tipos = [
        "Entrada".to_string(),
        "Saída".to_string(),
        "Sangria".to_string(),
        "Suprimento".to_string(),
    ];
    combo(ui, "m_tipo", "Tipo", &mut m.tipo, &tipos);
    combo(ui, "m_caixa", "Caixa", &mut m.caixa, &boxes);
    text(ui, &mut m.descricao, "Descrição");
    num(ui, &mut m.valor, "Valor");
    match gravar_cancela(ui) {
        FormAction::Save => {
            match i {
                Some(i) => app.data.movimentos[i] = m,
                None => app.data.movimentos.push(m),
            }
            app.clear_form_draft();
            app.form = None;
        }
        FormAction::Cancel => {
            app.clear_form_draft();
            app.form = None;
        }
        FormAction::None => app.keep_form_draft(form, FormDraft::MovimentoCaixa(m)),
    }
}

// ---------------------------------------------------------------- produtos

fn produtos(app: &mut App, ui: &mut Ui) {
    if let Some(Form::SaidaProduto(index)) = app.form {
        if index < app.data.produtos.len() {
            saida_produto_form(app, ui, index);
            return;
        }
        app.form = None;
    }
    if let Some(Form::EntradaProduto(index)) = app.form {
        if index < app.data.produtos.len() {
            entrada_produto_form(app, ui, index);
            return;
        }
        app.form = None;
    }
    if let Some(Form::NovoProduto) = app.form {
        produto_editor_com_busca(app, ui, None);
        return;
    }
    if let Some(Form::Produto(i)) = app.form {
        if i < app.data.produtos.len() {
            produto_editor_com_busca(app, ui, Some(i));
            return;
        }
        app.form = None;
    }

    ui.horizontal(|ui| {
        ui.vertical(|ui| {
            ui.heading("Catálogo de produtos");
            ui.label(
                RichText::new(
                    "Cadastre itens, acompanhe o estoque e mantenha os preços atualizados.",
                )
                .color(Color32::from_rgb(103, 116, 132)),
            );
        });
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            if ui.button("＋  Cadastrar produto").clicked() {
                app.form = Some(Form::NovoProduto);
                app.product_draft = None;
                app.prod_tab = 0;
            }
        });
    });
    let produtos = app.data.produtos.clone();
    let busca = app.search.to_lowercase();
    let quantidade_encontrada = produtos
        .iter()
        .filter(|produto| {
            format!(
                "{} {} {} {}",
                produto.codigo, produto.descricao, produto.fabricante, produto.codigo_barras
            )
            .to_lowercase()
            .contains(&busca)
        })
        .count();
    let estoque_total: f64 = produtos.iter().map(|produto| produto.estoque).sum();
    let estoque_baixo = produtos
        .iter()
        .filter(|produto| !produto.inativo && produto.estoque <= produto.estoque_minimo)
        .count();

    ui.add_space(14.0);
    ui.columns(3, |columns| {
        metric_card(
            &mut columns[0],
            "Produtos cadastrados",
            produtos.len().to_string(),
            "No catálogo",
            accent(),
        );
        metric_card(
            &mut columns[1],
            "Unidades em estoque",
            f2s(estoque_total),
            "Saldo total",
            Color32::from_rgb(0x00, 0xb8, 0x96),
        );
        metric_card(
            &mut columns[2],
            "Estoque baixo",
            estoque_baixo.to_string(),
            "Requer atenção",
            Color32::from_rgb(0xff, 0x9f, 0x43),
        );
    });

    ui.add_space(14.0);
    card(ui, |ui| {
        ui.horizontal(|ui| {
            ui.vertical(|ui| {
                section_caption(
                    ui,
                    "Buscar produtos",
                    "Clique em qualquer parte da linha para abrir os detalhes do produto.",
                );
            });
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                ui.label(
                    RichText::new(format!(
                        "{quantidade_encontrada} de {} produtos",
                        produtos.len()
                    ))
                    .color(Color32::from_rgb(107, 114, 128)),
                );
            });
        });
        ui.add_space(10.0);
        ui.horizontal(|ui| {
            ui.label(RichText::new("⌕").size(18.0).color(accent()));
            let response = ui.add(
                egui::TextEdit::singleline(&mut app.search)
                    .hint_text("Buscar por descrição, código, fabricante ou código de barras")
                    .desired_width(ui.available_width().max(180.0)),
            );
            if app.search_focus {
                response.request_focus();
                app.search_focus = false;
            }
            if !app.search.is_empty() && ui.small_button("Limpar").clicked() {
                app.search.clear();
            }
        });
        ui.add_space(12.0);
        ScrollArea::vertical().show(ui, |ui| {
            ui.horizontal(|ui| {
                ui.add_sized(
                    egui::vec2(76.0, 28.0),
                    egui::Label::new(RichText::new("Código").strong().color(accent())),
                );
                ui.add_sized(
                    egui::vec2(240.0, 28.0),
                    egui::Label::new(RichText::new("Descrição").strong().color(accent())),
                );
                ui.add_sized(
                    egui::vec2(82.0, 28.0),
                    egui::Label::new(RichText::new("Unidade").strong().color(accent())),
                );
                ui.add_sized(
                    egui::vec2(120.0, 28.0),
                    egui::Label::new(RichText::new("Grupo").strong().color(accent())),
                );
                ui.add_sized(
                    egui::vec2(92.0, 28.0),
                    egui::Label::new(RichText::new("Estoque").strong().color(accent())),
                );
                ui.add_sized(
                    egui::vec2(110.0, 28.0),
                    egui::Label::new(RichText::new("Preço").strong().color(accent())),
                );
            });
            for (i, p) in produtos.iter().enumerate() {
                let haystack = format!(
                    "{} {} {} {}",
                    p.codigo, p.descricao, p.fabricante, p.codigo_barras
                );
                if !busca.is_empty() && !haystack.to_lowercase().contains(&busca) {
                    continue;
                }
                let row_height = 42.0;
                let (row_rect, hover_response) = ui.allocate_exact_size(
                    egui::vec2(ui.available_width(), row_height),
                    egui::Sense::hover(),
                );
                let fill = if hover_response.hovered() {
                    Color32::from_rgb(247, 250, 255)
                } else if i % 2 == 1 {
                    Color32::from_rgb(249, 250, 252)
                } else {
                    Color32::WHITE
                };
                ui.painter().rect_filled(row_rect, 6.0, fill);
                ui.painter().rect_stroke(
                    row_rect,
                    6.0,
                    egui::Stroke::new(1.0, Color32::from_rgb(225, 231, 239)),
                    egui::StrokeKind::Inside,
                );
                let mut row = ui.new_child(
                    egui::UiBuilder::new()
                        .max_rect(row_rect)
                        .layout(egui::Layout::left_to_right(egui::Align::Min)),
                );
                row.spacing_mut().item_spacing.x = 8.0;
                row.add_sized(
                    egui::vec2(76.0, row_height),
                    egui::Label::new(RichText::new(&p.codigo).monospace()),
                );
                row.add_sized(
                    egui::vec2(240.0, row_height),
                    egui::Label::new(RichText::new(&p.descricao).strong()),
                );
                row.add_sized(egui::vec2(82.0, row_height), egui::Label::new(&p.unidade));
                row.add_sized(egui::vec2(120.0, row_height), egui::Label::new(&p.grupo));
                row.add_sized(
                    egui::vec2(92.0, row_height),
                    egui::Label::new(RichText::new(f2s(p.estoque)).color(
                        if p.estoque <= p.estoque_minimo {
                            Color32::from_rgb(0xff, 0x9f, 0x43)
                        } else {
                            Color32::from_rgb(32, 36, 43)
                        },
                    )),
                );
                row.add_sized(
                    egui::vec2(110.0, row_height),
                    egui::Label::new(format!("R$ {}", f2s(p.preco))),
                );
                let row_response = ui.interact(
                    row_rect,
                    ui.make_persistent_id(("produto_linha_principal", i)),
                    egui::Sense::click(),
                );
                if row_response.clicked() {
                    abrir_produto(app, i);
                }
            }
            if quantidade_encontrada == 0 {
                ui.add_space(10.0);
                ui.vertical_centered(|ui| {
                    ui.label(RichText::new("Nenhum produto encontrado").strong());
                    ui.label(
                        RichText::new("Tente outro termo ou cadastre um novo item.")
                            .color(Color32::from_rgb(103, 116, 132)),
                    );
                });
            }
        });
    });
}

fn produto_editor_com_busca(app: &mut App, ui: &mut Ui, index: Option<usize>) {
    let width = ui.available_width();
    let height = ui.available_height();
    let search_height = height * 0.30;
    let detail_height = height - search_height;

    ui.allocate_ui_with_layout(
        egui::vec2(width, detail_height),
        egui::Layout::top_down(egui::Align::Min),
        |ui| produto_form(app, ui, index),
    );
    ui.allocate_ui_with_layout(
        egui::vec2(width, search_height),
        egui::Layout::top_down(egui::Align::Min),
        |ui| produto_catalogo_compacto(app, ui),
    );
}

fn saida_produto_form(app: &mut App, ui: &mut Ui, index: usize) {
    let form = Form::SaidaProduto(index);
    let produto = &app.data.produtos[index];
    let nome = produto.descricao.clone();
    let codigo = produto.codigo.clone();
    let unidade = produto.unidade.clone();
    let estoque = produto.estoque;
    let initial = StockExitDraft {
        quantidade: 1.0_f64.min(estoque.max(0.0)),
        observacao: String::new(),
    };
    let mut draft = match app.take_form_draft(form, FormDraft::SaidaEstoque(initial)) {
        FormDraft::SaidaEstoque(draft) => draft,
        _ => unreachable!(),
    };

    ui.horizontal(|ui| {
        ui.vertical(|ui| {
            ui.heading("Registrar saída de estoque");
            ui.label(
                RichText::new("Informe a quantidade que está saindo do estoque.")
                    .color(Color32::from_rgb(107, 114, 128)),
            );
        });
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            if ui.button("← Voltar ao catálogo").clicked() {
                app.form = None;
                app.stock_exit_error = None;
                app.form_drafts.remove(&form);
            }
        });
    });
    ui.add_space(12.0);
    card(ui, |ui| {
        ui.label(RichText::new("Produto selecionado").size(15.0).strong());
        ui.add_space(6.0);
        ui.horizontal(|ui| {
            ui.vertical(|ui| {
                ui.label(RichText::new(&nome).size(18.0).strong());
                ui.label(
                    RichText::new(format!("Código {codigo} · Unidade {unidade}"))
                        .color(Color32::from_rgb(107, 114, 128)),
                );
            });
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                ui.vertical(|ui| {
                    ui.label(
                        RichText::new("SALDO DISPONÍVEL")
                            .size(10.0)
                            .strong()
                            .color(Color32::from_rgb(107, 114, 128)),
                    );
                    ui.label(RichText::new(f2s(estoque)).size(22.0).strong());
                });
            });
        });
        ui.add_space(14.0);
        ui.columns(2, |columns| {
            columns[0].vertical(|ui| {
                ui.label(RichText::new("Quantidade de saída").strong());
                ui.add(
                    egui::DragValue::new(&mut draft.quantidade)
                        .range(0.0..=estoque.max(0.0))
                        .speed(0.25)
                        .max_decimals(3),
                );
            });
            columns[1].vertical(|ui| {
                ui.label(RichText::new("Observação (opcional)").strong());
                ui.add(
                    egui::TextEdit::singleline(&mut draft.observacao)
                        .desired_width(ui.available_width())
                        .hint_text("Ex.: aplicação em ordem de serviço"),
                );
            });
        });
        let quantidade_valida =
            draft.quantidade.is_finite() && draft.quantidade > 0.0 && draft.quantidade <= estoque;
        if !quantidade_valida {
            ui.colored_label(
                Color32::from_rgb(180, 35, 24),
                "Informe uma quantidade maior que zero e dentro do saldo disponível.",
            );
        }
        if let Some(error) = &app.stock_exit_error {
            ui.colored_label(Color32::from_rgb(180, 35, 24), error);
        }
        ui.add_space(12.0);
        ui.horizontal(|ui| {
            if ui
                .add_enabled(
                    quantidade_valida,
                    egui::Button::new(RichText::new("Registrar saída").strong())
                        .fill(accent())
                        .min_size(egui::vec2(150.0, 38.0)),
                )
                .clicked()
            {
                match app.data.registrar_saida_estoque(
                    index,
                    draft.quantidade,
                    draft.observacao.clone(),
                    today(),
                ) {
                    Ok(()) => {
                        app.form = None;
                        app.form_drafts.remove(&form);
                        app.stock_exit_error = None;
                    }
                    Err(error) => app.stock_exit_error = Some(error.into()),
                }
            }
            if ui.button("Cancelar").clicked() {
                app.form = None;
                app.form_drafts.remove(&form);
                app.stock_exit_error = None;
            }
        });
    });
    if app.form == Some(form) {
        app.keep_form_draft(form, FormDraft::SaidaEstoque(draft));
    }
}

fn entrada_produto_form(app: &mut App, ui: &mut Ui, index: usize) {
    let form = Form::EntradaProduto(index);
    let produto = &app.data.produtos[index];
    let nome = produto.descricao.clone();
    let codigo = produto.codigo.clone();
    let unidade = produto.unidade.clone();
    let estoque = produto.estoque;
    let initial = StockExitDraft {
        quantidade: 1.0,
        observacao: String::new(),
    };
    let mut draft = match app.take_form_draft(form, FormDraft::EntradaEstoque(initial)) {
        FormDraft::EntradaEstoque(draft) => draft,
        _ => unreachable!(),
    };
    ui.horizontal(|ui| {
        ui.vertical(|ui| {
            ui.heading("Registrar entrada de estoque");
            ui.label(
                RichText::new("Registre a quantidade recebida para atualizar o saldo do produto.")
                    .color(Color32::from_rgb(107, 114, 128)),
            );
        });
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            if ui.button("← Voltar ao catálogo").clicked() {
                app.form = None;
                app.form_drafts.remove(&form);
            }
        });
    });
    ui.add_space(12.0);
    card(ui, |ui| {
        ui.label(RichText::new("Produto selecionado").size(15.0).strong());
        ui.horizontal(|ui| {
            ui.vertical(|ui| {
                ui.label(RichText::new(&nome).size(18.0).strong());
                ui.label(
                    RichText::new(format!("Código {codigo} · Unidade {unidade}"))
                        .color(Color32::from_rgb(107, 114, 128)),
                );
            });
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                ui.vertical(|ui| {
                    ui.label(
                        RichText::new("SALDO ATUAL")
                            .size(10.0)
                            .strong()
                            .color(Color32::from_rgb(107, 114, 128)),
                    );
                    ui.label(RichText::new(f2s(estoque)).size(22.0).strong());
                });
            });
        });
        ui.add_space(14.0);
        ui.columns(2, |columns| {
            columns[0].vertical(|ui| {
                ui.label(RichText::new("Quantidade de entrada").strong());
                ui.add(
                    egui::DragValue::new(&mut draft.quantidade)
                        .range(0.0..=1_000_000.0)
                        .speed(0.25)
                        .max_decimals(3),
                );
            });
            columns[1].vertical(|ui| {
                ui.label(RichText::new("Observação (opcional)").strong());
                ui.add(
                    egui::TextEdit::singleline(&mut draft.observacao)
                        .desired_width(ui.available_width())
                        .hint_text("Ex.: NF de compra ou fornecedor"),
                );
            });
        });
        let valida = draft.quantidade.is_finite() && draft.quantidade > 0.0;
        if !valida {
            ui.colored_label(
                Color32::from_rgb(180, 35, 24),
                "Informe uma quantidade maior que zero.",
            );
        }
        if let Some(error) = &app.stock_exit_error {
            ui.colored_label(Color32::from_rgb(180, 35, 24), error);
        }
        ui.add_space(12.0);
        ui.horizontal(|ui| {
            if ui
                .add_enabled(
                    valida,
                    egui::Button::new(RichText::new("Registrar entrada").strong())
                        .fill(accent())
                        .min_size(egui::vec2(150.0, 38.0)),
                )
                .clicked()
            {
                match app.data.registrar_entrada_estoque(
                    index,
                    draft.quantidade,
                    draft.observacao.clone(),
                    today(),
                ) {
                    Ok(()) => {
                        app.form = None;
                        app.form_drafts.remove(&form);
                        app.stock_exit_error = None;
                    }
                    Err(error) => app.stock_exit_error = Some(error.into()),
                }
            }
            if ui.button("Cancelar").clicked() {
                app.form = None;
                app.form_drafts.remove(&form);
                app.stock_exit_error = None;
            }
        });
    });
    if app.form == Some(form) {
        app.keep_form_draft(form, FormDraft::EntradaEstoque(draft));
    }
}

fn abrir_produto(app: &mut App, index: usize) {
    if let Some(current_form) = app.form {
        if matches!(current_form, Form::NovoProduto | Form::Produto(_)) {
            if let Some(draft) = app.product_draft.take() {
                app.product_drafts.insert(current_form, draft);
            }
        }
    }

    let target_form = Form::Produto(index);
    app.product_draft = app.product_drafts.remove(&target_form);
    app.form = Some(target_form);
    app.prod_tab = 0;
    app.delete_confirmation = None;
}

fn produto_catalogo_compacto(app: &mut App, ui: &mut Ui) {
    let produtos = app.data.produtos.clone();
    let busca = app.search.to_lowercase();
    let resultados = produtos
        .iter()
        .enumerate()
        .filter(|(_, produto)| {
            format!("{} {}", produto.codigo, produto.descricao)
                .to_lowercase()
                .contains(&busca)
        })
        .collect::<Vec<_>>();
    let mut selected_product = None;
    egui::Frame::new()
        .fill(Color32::WHITE)
        .stroke(egui::Stroke::new(1.0, Color32::from_rgb(184, 198, 216)))
        .corner_radius(egui::CornerRadius::same(8))
        .inner_margin(egui::Margin::same(8))
        .show(ui, |ui| {
            let mut style = (**ui.style()).clone();
            style.spacing.item_spacing = egui::vec2(6.0, 4.0);
            style.spacing.interact_size.y = 28.0;
            ui.set_style(style);

            ui.horizontal(|ui| {
                ui.label(RichText::new("Pesquisar outro produto").strong().size(14.0));
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    ui.label(
                        RichText::new(format!("{} resultados", resultados.len()))
                            .size(11.0)
                            .color(Color32::from_rgb(83, 98, 115)),
                    );
                });
            });
            ui.horizontal(|ui| {
                ui.label(RichText::new("⌕").size(16.0).color(accent()));
                ui.add(
                    egui::TextEdit::singleline(&mut app.search)
                        .hint_text("Código ou descrição")
                        .desired_width(ui.available_width()),
                );
            });

            ui.add_space(2.0);
            let available_width = ui.available_width();
            let column_widths = [
                available_width * 0.15,
                available_width * 0.37,
                available_width * 0.12,
                available_width * 0.17,
                available_width * 0.19,
            ];
            let header_height = 24.0;
            let (header_rect, _) = ui.allocate_exact_size(
                egui::vec2(available_width, header_height),
                egui::Sense::hover(),
            );
            ui.painter()
                .rect_filled(header_rect, 3.0, Color32::from_rgb(232, 238, 246));
            ui.painter().rect_stroke(
                header_rect,
                3.0,
                egui::Stroke::new(1.0, Color32::from_rgb(184, 198, 216)),
                egui::StrokeKind::Inside,
            );
            let mut header = ui.new_child(
                egui::UiBuilder::new()
                    .max_rect(header_rect)
                    .layout(egui::Layout::left_to_right(egui::Align::Center)),
            );
            for (title, width) in ["Código", "Descrição", "Unidade", "Estoque", "Preço"]
                .into_iter()
                .zip(column_widths)
            {
                header.add_sized(
                    egui::vec2(width, header_height),
                    egui::Label::new(RichText::new(title).strong().color(accent())).truncate(),
                );
            }

            ScrollArea::vertical()
                .id_salt("produto_form_catalogo")
                .auto_shrink([false, false])
                .show(ui, |ui| {
                    for (row_index, (index, produto)) in resultados.iter().enumerate() {
                        let row_height = 30.0;
                        let (rect, hover_response) = ui.allocate_exact_size(
                            egui::vec2(ui.available_width(), row_height),
                            egui::Sense::hover(),
                        );
                        let fill = if hover_response.hovered() {
                            Color32::from_rgb(225, 235, 250)
                        } else if row_index % 2 == 0 {
                            Color32::WHITE
                        } else {
                            Color32::from_rgb(241, 245, 249)
                        };
                        ui.painter().rect_filled(rect, 2.0, fill);
                        ui.painter().rect_stroke(
                            rect,
                            2.0,
                            egui::Stroke::new(1.0, Color32::from_rgb(215, 224, 235)),
                            egui::StrokeKind::Inside,
                        );
                        let mut row = ui.new_child(
                            egui::UiBuilder::new()
                                .max_rect(rect)
                                .layout(egui::Layout::left_to_right(egui::Align::Center)),
                        );
                        for (value, width) in [
                            RichText::new(&produto.codigo).monospace(),
                            RichText::new(&produto.descricao).strong(),
                            RichText::new(&produto.unidade),
                            RichText::new(f2s(produto.estoque)),
                            RichText::new(format!("R$ {}", f2s(produto.preco))),
                        ]
                        .into_iter()
                        .zip(column_widths)
                        {
                            row.add_sized(
                                egui::vec2(width, row_height),
                                egui::Label::new(value).truncate(),
                            );
                        }
                        let response = ui.interact(
                            rect,
                            ui.make_persistent_id(("produto_linha_preview", *index)),
                            egui::Sense::click(),
                        );
                        if response.clicked() {
                            selected_product = Some(*index);
                        }
                    }
                    if resultados.is_empty() {
                        ui.label(
                            RichText::new("Nenhum produto encontrado")
                                .color(Color32::from_rgb(83, 98, 115)),
                        );
                    }
                });
        });
    if let Some(index) = selected_product {
        abrir_produto(app, index);
    }
}

fn produto_form(app: &mut App, ui: &mut Ui, i: Option<usize>) {
    let focus_name = app.product_draft.is_none();
    if app.product_draft.is_none() {
        app.product_draft = Some(match i {
            Some(index) => app.data.produtos[index].clone(),
            None => Produto {
                id: next_id(
                    "P",
                    app.data.produtos.iter().map(|record| record.id.clone()),
                ),
                codigo: next_product_code(
                    app.data.produtos.iter().map(|record| record.codigo.clone()),
                ),
                ..Default::default()
            },
        });
    }

    let mut save = false;
    let mut cancel = false;
    let mut delete = false;
    egui::Frame::new()
        .fill(Color32::WHITE)
        .stroke(egui::Stroke::new(1.0, Color32::from_rgb(184, 198, 216)))
        .corner_radius(egui::CornerRadius::same(8))
        .inner_margin(egui::Margin::same(8))
        .show(ui, |ui| {
            let mut style = (**ui.style()).clone();
            style.spacing.item_spacing = egui::vec2(6.0, 4.0);
            style.spacing.button_padding = egui::vec2(8.0, 4.0);
            style.spacing.interact_size.y = 28.0;
            ui.set_style(style);

            ui.horizontal(|ui| {
                ui.vertical(|ui| {
                    ui.heading(if i.is_some() {
                        "Editar produto"
                    } else {
                        "Cadastrar produto"
                    });
                    ui.label(
                        RichText::new("Identificação, estoque e dados comerciais do item.")
                            .size(11.0)
                            .color(Color32::from_rgb(83, 98, 115)),
                    );
                });
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    if ui.button("← Voltar ao catálogo").clicked() {
                        cancel = true;
                    }
                    if let Some(index) = i {
                        if ui.button("↑ Entrada").clicked() {
                            app.form = Some(Form::EntradaProduto(index));
                            app.stock_exit_error = None;
                        }
                        let tem_saldo = app
                            .data
                            .produtos
                            .get(index)
                            .is_some_and(|produto| produto.estoque > 0.0);
                        if ui
                            .add_enabled(tem_saldo, egui::Button::new("↓ Saída"))
                            .clicked()
                        {
                            app.form = Some(Form::SaidaProduto(index));
                            app.stock_exit_error = None;
                        }
                    }
                    if let Some(index) = i {
                        if ui.button("Excluir produto").clicked() {
                            app.delete_confirmation = Some(Form::Produto(index));
                        }
                    }
                });
            });

            ui.separator();
            abas(
                ui,
                &[
                    "Geral",
                    "Variações",
                    "Fiscal",
                    "Unidades",
                    "Fornecedores",
                    "Estatísticas",
                    "Aplicações",
                    "Complementos",
                    "Preços",
                    "Mercado Livre",
                ],
                &mut app.prod_tab,
            );

            let p = app
                .product_draft
                .as_mut()
                .expect("rascunho do produto inicializado");
            ui.allocate_ui_with_layout(
                egui::vec2(
                    ui.available_width(),
                    (ui.available_height() - 48.0).max(0.0),
                ),
                egui::Layout::top_down(egui::Align::Min),
                |ui| match app.prod_tab {
                    0 => produto_aba_geral(ui, p, focus_name),
                    1 => produto_aba_variacoes(ui, p),
                    2 => produto_aba_fiscal(ui, p),
                    3 => produto_aba_unidades(ui, p),
                    4 => produto_aba_fornecedores(ui, p),
                    5 => produto_aba_estatisticas(ui, p),
                    6 => produto_aba_aplicacoes(ui, p),
                    7 => produto_aba_complementos(ui, p),
                    8 => produto_aba_precos(ui, p),
                    9 => produto_aba_mercado_livre(ui),
                    _ => {}
                },
            );

            ui.separator();
            let confirm_delete = matches!(
                (i, app.delete_confirmation),
                (Some(index), Some(Form::Produto(confirm_index))) if index == confirm_index
            );
            if confirm_delete {
                ui.horizontal(|ui| {
                    ui.label("Excluir este produto permanentemente?");
                    if ui.button("Confirmar exclusão").clicked() {
                        delete = true;
                    }
                    if ui.button("Manter produto").clicked() {
                        app.delete_confirmation = None;
                    }
                });
            }
            ui.horizontal(|ui| {
                let nome_valido = !p.descricao.trim().is_empty();
                let save_text = if nome_valido {
                    Color32::WHITE
                } else {
                    Color32::from_rgb(83, 98, 115)
                };
                let save_fill = if nome_valido {
                    accent()
                } else {
                    Color32::from_rgb(226, 232, 240)
                };
                save = ui
                    .add_enabled(
                        nome_valido,
                        egui::Button::new(
                            RichText::new("✓  Salvar alterações")
                                .strong()
                                .color(save_text),
                        )
                        .fill(save_fill)
                        .min_size(egui::vec2(150.0, 30.0)),
                    )
                    .clicked();
                if ui.button("Cancelar").clicked() {
                    cancel = true;
                }
                if !nome_valido {
                    ui.label(
                        RichText::new("O nome do item é obrigatório.")
                            .color(Color32::from_rgb(154, 83, 12)),
                    );
                }
            });
        });

    if save {
        if let Some(p) = app.product_draft.take() {
            let mut p = p;
            p.descricao = p.descricao.trim().to_owned();
            match i {
                Some(index) => app.data.produtos[index] = p,
                None => app.data.produtos.push(p),
            }
        }
        app.product_drafts
            .remove(&i.map_or(Form::NovoProduto, Form::Produto));
        app.delete_confirmation = None;
        app.form = None;
    } else if delete {
        if let Some(index) = i.filter(|index| *index < app.data.produtos.len()) {
            app.data.produtos.remove(index);
        }
        app.product_drafts.clear();
        app.delete_confirmation = None;
        app.product_draft = None;
        app.form = None;
    } else if cancel {
        app.product_drafts
            .remove(&i.map_or(Form::NovoProduto, Form::Produto));
        app.delete_confirmation = None;
        app.product_draft = None;
        app.form = None;
    }
}

fn produto_aba_geral(ui: &mut Ui, p: &mut Produto, focus_name: bool) {
    ui.columns(4, |columns| {
        columns[0].vertical(|ui| {
            ui.label(RichText::new("Nome do item *").strong());
            let response = ui.add(
                egui::TextEdit::singleline(&mut p.descricao)
                    .desired_width(ui.available_width())
                    .hint_text("Nome ou descrição"),
            );
            if focus_name {
                response.request_focus();
            }
            produto_campo_compacto(ui, "Código", &mut p.codigo, "Automático");
            produto_campo_compacto(ui, "Unidade", &mut p.unidade, "Un");
            produto_campo_compacto(ui, "Grupo", &mut p.grupo, "Grupo");
            produto_campo_compacto(ui, "Subgrupo", &mut p.subgrupo, "Subgrupo");
        });
        columns[1].vertical(|ui| {
            produto_campo_compacto(ui, "Fabricante", &mut p.fabricante, "Fabricante");
            produto_campo_compacto(ui, "Fornecedor", &mut p.fornecedor, "Fornecedor principal");
            produto_campo_compacto(ui, "Local", &mut p.local, "Local no estoque");
            produto_campo_compacto(ui, "Embalagem", &mut p.embalagem, "Ex.: unitário");
            produto_campo_compacto(ui, "Aplicação", &mut p.aplicacao, "Veículo ou modelo");
        });
        columns[2].vertical(|ui| {
            produto_campo_num_compacto(ui, "Estoque", &mut p.estoque, 3);
            produto_campo_num_compacto(ui, "Mínimo", &mut p.estoque_minimo, 3);
            produto_campo_num_compacto(ui, "Máximo", &mut p.estoque_maximo, 3);
            produto_campo_compacto(ui, "Produção", &mut p.producao, "Local de produção");
            produto_campo_compacto(ui, "Destino", &mut p.destino, "Local de destino");
        });
        columns[3].vertical(|ui| {
            produto_campo_compacto(ui, "Montadora", &mut p.montadora, "Montadora");
            produto_campo_compacto(ui, "Loja", &mut p.codigo_loja, "Código na loja");
            produto_campo_compacto(ui, "Fabricante", &mut p.codigo_fabricante, "Referência");
            produto_campo_compacto(ui, "Código de barras", &mut p.codigo_barras, "EAN / GTIN");
            produto_campo_compacto(ui, "Sistema", &mut p.codigo_sistema, "Código externo");
            ui.label(
                RichText::new(if p.foto {
                    "Imagem vinculada"
                } else {
                    "Sem imagem vinculada"
                })
                .size(11.0)
                .color(Color32::from_rgb(83, 98, 115)),
            );
        });
    });
    ui.checkbox(&mut p.inativo, "Produto inativo");
}

fn produto_aba_variacoes(ui: &mut Ui, p: &mut Produto) {
    ui.label(
        RichText::new("Variações e equivalências")
            .strong()
            .size(15.0),
    );
    ui.label(
        RichText::new("Registre aplicações, modelos ou códigos equivalentes deste item.")
            .color(Color32::from_rgb(103, 116, 132)),
    );
    ui.add_space(8.0);
    produto_campo(ui, "Montadora", &mut p.montadora, "Montadora");
    produto_campo(ui, "Código de barras", &mut p.codigo_barras, "EAN / GTIN");
    ui.label(RichText::new("Variações (tamanho, cor, aplicação)").strong());
    ui.add(
        egui::TextEdit::multiline(&mut p.variacoes)
            .desired_width(ui.available_width())
            .desired_rows(2)
            .hint_text("Uma variação ou aplicação por linha"),
    );
}

fn produto_aba_fiscal(ui: &mut Ui, p: &mut Produto) {
    ui.label(RichText::new("Valores e tributação").strong().size(15.0));
    ui.label(
        RichText::new("Defina o custo, o preço de venda e os dados fiscais do produto.")
            .color(Color32::from_rgb(103, 116, 132)),
    );
    ui.add_space(10.0);
    egui::Frame::group(ui.style()).show(ui, |ui| {
        ui.label(RichText::new("Formação de preço").strong());
        ui.columns(3, |columns| {
            produto_campo_num(&mut columns[0], "Custo (R$)", &mut p.custo, 2);
            produto_campo_num(&mut columns[1], "Margem (%)", &mut p.margem, 2);
            produto_campo_num(&mut columns[2], "Preço de venda (R$)", &mut p.preco, 2);
        });
    });
    ui.add_space(8.0);
    egui::Frame::group(ui.style()).show(ui, |ui| {
        ui.label(RichText::new("Tributação").strong());
        ui.columns(3, |columns| {
            produto_campo_num(&mut columns[0], "ICMS (%)", &mut p.icms, 2);
            produto_campo_num(&mut columns[1], "IPI (%)", &mut p.ipi, 2);
            produto_campo(&mut columns[2], "CFOP", &mut p.cfop, "Ex.: 5.102");
        });
        ui.checkbox(&mut p.st, "Produto sujeito à substituição tributária");
    });
}

fn produto_aba_unidades(ui: &mut Ui, p: &mut Produto) {
    ui.label(RichText::new("Unidade e embalagem").strong().size(15.0));
    ui.label(
        RichText::new("Defina como o item é identificado e apresentado no estoque.")
            .color(Color32::from_rgb(103, 116, 132)),
    );
    ui.add_space(10.0);
    egui::Frame::group(ui.style()).show(ui, |ui| {
        ui.columns(2, |columns| {
            produto_campo(&mut columns[0], "Unidade de medida", &mut p.unidade, "Un");
            produto_campo(
                &mut columns[1],
                "Embalagem",
                &mut p.embalagem,
                "Ex.: Unitário",
            );
        });
    });
    egui::Frame::group(ui.style()).show(ui, |ui| {
        ui.label(RichText::new("Controle de estoque").strong());
        ui.columns(3, |columns| {
            produto_campo_num(&mut columns[0], "Quantidade atual", &mut p.estoque, 3);
            produto_campo_num(&mut columns[1], "Estoque mínimo", &mut p.estoque_minimo, 3);
            produto_campo_num(&mut columns[2], "Estoque máximo", &mut p.estoque_maximo, 3);
        });
    });
}

fn produto_aba_fornecedores(ui: &mut Ui, p: &mut Produto) {
    ui.label(RichText::new("Origem e fornecimento").strong().size(15.0));
    ui.add_space(10.0);
    egui::Frame::group(ui.style()).show(ui, |ui| {
        produto_campo(
            ui,
            "Fornecedor principal",
            &mut p.fornecedor,
            "Nome do fornecedor",
        );
        produto_campo(ui, "Fabricante", &mut p.fabricante, "Nome do fabricante");
        produto_campo(ui, "Montadora", &mut p.montadora, "Montadora");
    });
}

fn produto_aba_estatisticas(ui: &mut Ui, p: &mut Produto) {
    ui.label(RichText::new("Resumo do produto").strong().size(15.0));
    ui.add_space(10.0);
    ui.columns(3, |columns| {
        columns[0].vertical(|ui| {
            ui.label("Estoque atual");
            ui.heading(f2s(p.estoque));
        });
        columns[1].vertical(|ui| {
            ui.label("Estoque mínimo");
            ui.heading(f2s(p.estoque_minimo));
        });
        columns[2].vertical(|ui| {
            ui.label("Preço de venda");
            ui.heading(format!("R$ {}", f2s(p.preco)));
        });
    });
    ui.add_space(8.0);
    let status = if p.inativo {
        "Inativo"
    } else if p.estoque <= p.estoque_minimo {
        "Estoque no mínimo ou abaixo"
    } else if p.estoque_maximo > 0.0 && p.estoque >= p.estoque_maximo {
        "Estoque no máximo ou acima"
    } else {
        "Disponível"
    };
    ui.label(RichText::new(format!("Situação: {status}")).strong());
    ui.label(
        RichText::new("Indicadores calculados a partir dos dados deste cadastro.")
            .color(Color32::from_rgb(103, 116, 132)),
    );
}

fn produto_aba_aplicacoes(ui: &mut Ui, p: &mut Produto) {
    ui.label(
        RichText::new("Aplicações e equivalências")
            .strong()
            .size(15.0),
    );
    ui.add_space(8.0);
    produto_campo(
        ui,
        "Aplicação",
        &mut p.aplicacao,
        "Veículo, modelo ou observação de aplicação",
    );
    produto_campo(ui, "Montadora", &mut p.montadora, "Montadora");
    ui.label(RichText::new("Variações e códigos equivalentes").strong());
    ui.add(
        egui::TextEdit::multiline(&mut p.variacoes)
            .desired_width(ui.available_width())
            .desired_rows(5)
            .hint_text("Uma variação ou aplicação por linha"),
    );
}

fn produto_aba_complementos(ui: &mut Ui, p: &mut Produto) {
    ui.label(
        RichText::new("Códigos e informações complementares")
            .strong()
            .size(15.0),
    );
    ui.add_space(8.0);
    egui::Frame::group(ui.style()).show(ui, |ui| {
        ui.columns(2, |columns| {
            produto_campo(
                &mut columns[0],
                "Código da loja",
                &mut p.codigo_loja,
                "Código na loja",
            );
            produto_campo(
                &mut columns[1],
                "Código do fabricante",
                &mut p.codigo_fabricante,
                "Referência do fabricante",
            );
        });
        ui.columns(2, |columns| {
            produto_campo(
                &mut columns[0],
                "Código de barras",
                &mut p.codigo_barras,
                "EAN / GTIN",
            );
            produto_campo(
                &mut columns[1],
                "Código do sistema",
                &mut p.codigo_sistema,
                "Código legado ou externo",
            );
        });
    });
    ui.add_space(8.0);
    produto_aba_observacoes(ui, p);
}

fn produto_aba_precos(ui: &mut Ui, p: &mut Produto) {
    ui.label(RichText::new("Preços e tributação").strong().size(15.0));
    ui.add_space(8.0);
    produto_aba_fiscal(ui, p);
}

fn produto_aba_mercado_livre(ui: &mut Ui) {
    ui.label(
        RichText::new("Integração com Mercado Livre")
            .strong()
            .size(15.0),
    );
    ui.add_space(8.0);
    ui.label("A integração não está disponível nesta versão.");
}

fn produto_aba_observacoes(ui: &mut Ui, p: &mut Produto) {
    ui.label(RichText::new("Observações internas").strong().size(15.0));
    ui.label(
        RichText::new("Estas informações ficam disponíveis para consulta no cadastro.")
            .color(Color32::from_rgb(103, 116, 132)),
    );
    ui.add_space(8.0);
    ui.add(
        egui::TextEdit::multiline(&mut p.obs)
            .desired_width(ui.available_width())
            .desired_rows(2)
            .hint_text("Adicione detalhes importantes sobre o produto"),
    );
}

fn produto_campo(ui: &mut Ui, label: &str, value: &mut String, hint: &str) {
    ui.vertical(|ui| {
        ui.label(RichText::new(label).strong().size(12.0));
        ui.add(
            egui::TextEdit::singleline(value)
                .desired_width(ui.available_width())
                .hint_text(hint),
        );
        ui.add_space(4.0);
    });
}

fn produto_campo_compacto(ui: &mut Ui, label: &str, value: &mut String, hint: &str) {
    let label_width = (ui.available_width() * 0.38).clamp(48.0, 108.0);
    ui.horizontal(|ui| {
        ui.add_sized(
            [label_width, 24.0],
            egui::Label::new(RichText::new(label).strong().size(11.0)).truncate(),
        );
        ui.add(
            egui::TextEdit::singleline(value)
                .desired_width(ui.available_width())
                .hint_text(hint),
        );
    });
}

fn produto_campo_num_compacto<T: egui::emath::Numeric>(
    ui: &mut Ui,
    label: &str,
    value: &mut T,
    decimals: usize,
) {
    let label_width = (ui.available_width() * 0.38).clamp(48.0, 108.0);
    ui.horizontal(|ui| {
        ui.add_sized(
            [label_width, 24.0],
            egui::Label::new(RichText::new(label).strong().size(11.0)).truncate(),
        );
        ui.add_sized(
            [ui.available_width(), 24.0],
            egui::DragValue::new(value).max_decimals(decimals),
        );
    });
}

fn produto_campo_num<T: egui::emath::Numeric>(
    ui: &mut Ui,
    label: &str,
    value: &mut T,
    decimals: usize,
) {
    ui.vertical(|ui| {
        ui.label(RichText::new(label).strong().size(12.0));
        ui.add_sized(
            [ui.available_width(), 24.0],
            egui::DragValue::new(value).max_decimals(decimals),
        );
        ui.add_space(4.0);
    });
}

// ---------------------------------------------------------------- profissionais

fn profissionais(app: &mut App, ui: &mut Ui) {
    heading(ui, "Profissionais");
    ui.horizontal(|ui| {
        if ui.button("Novo Profissional").clicked() {
            app.form = Some(Form::NovoProfissional);
        }
    });
    let profs = app.data.profissionais.clone();
    ScrollArea::vertical().show(ui, |ui| {
        egui::Grid::new("grid_profissionais")
            .striped(true)
            .show(ui, |ui| {
                headers(
                    ui,
                    &["Nome", "Telefone", "Especialidade", "Comissão", "Ativo", ""],
                );
                for (i, p) in profs.iter().enumerate() {
                    ui.label(&p.nome);
                    ui.label(&p.telefone);
                    ui.label(&p.especialidade);
                    ui.label(f2s(p.comissao));
                    ui.label(if p.ativo { "Sim" } else { "Não" });
                    if ui.button("Detalhes").clicked() {
                        app.form = Some(Form::Profissional(i));
                    }
                    ui.end_row();
                }
            });
    });
    if let Some(Form::NovoProfissional) = app.form {
        profissional_form(app, ui, None);
    }
    if let Some(Form::Profissional(i)) = app.form {
        profissional_form(app, ui, Some(i));
    }
}

fn profissional_form(app: &mut App, ui: &mut Ui, i: Option<usize>) {
    heading(
        ui,
        if i.is_some() {
            "Profissional — Gravar / Alterar"
        } else {
            "Novo Profissional"
        },
    );
    let form = i.map(Form::Profissional).unwrap_or(Form::NovoProfissional);
    let initial = match i {
        Some(i) => app.data.profissionais[i].clone(),
        None => Profissional::default(),
    };
    let mut p = match app.take_form_draft(form, FormDraft::Profissional(initial)) {
        FormDraft::Profissional(p) => p,
        _ => unreachable!(),
    };
    text(ui, &mut p.nome, "Nome");
    text(ui, &mut p.telefone, "Telefone");
    text(ui, &mut p.especialidade, "Especialidade");
    num(ui, &mut p.comissao, "Comissão (%)");
    check(ui, &mut p.ativo, "Ativo");
    match gravar_cancela(ui) {
        FormAction::Save => {
            match i {
                Some(i) => app.data.profissionais[i] = p,
                None => {
                    p.id = next_id(
                        "PF",
                        app.data
                            .profissionais
                            .iter()
                            .map(|record| record.id.clone()),
                    );
                    app.data.profissionais.push(p);
                }
            }
            app.clear_form_draft();
            app.form = None;
        }
        FormAction::Cancel => {
            app.clear_form_draft();
            app.form = None;
        }
        FormAction::None => app.keep_form_draft(form, FormDraft::Profissional(p)),
    }
}

// ---------------------------------------------------------------- fornecedores

fn fornecedores(app: &mut App, ui: &mut Ui) {
    heading(ui, "Fornecedores");
    ui.horizontal(|ui| {
        if ui.button("Novo Fornecedor").clicked() {
            app.form = Some(Form::NovoFornecedor);
        }
    });
    let fornecs = app.data.fornecedors.clone();
    ScrollArea::vertical().show(ui, |ui| {
        egui::Grid::new("grid_fornecedores")
            .striped(true)
            .show(ui, |ui| {
                headers(ui, &["Código", "Nome", "Telefone", "E-mail", ""]);
                for (i, f) in fornecs.iter().enumerate() {
                    ui.label(&f.id);
                    ui.label(&f.nome);
                    ui.label(&f.telefone);
                    ui.label(&f.email);
                    if ui.button("Detalhes").clicked() {
                        app.form = Some(Form::Fornecedor(i));
                    }
                    ui.end_row();
                }
            });
    });
    if let Some(Form::NovoFornecedor) = app.form {
        fornecedor_form(app, ui, None);
    }
    if let Some(Form::Fornecedor(i)) = app.form {
        fornecedor_form(app, ui, Some(i));
    }
    if let Some(Form::ContaBanc(fi, ci)) = app.form {
        conta_form(app, ui, fi, ci);
    }
}

fn fornecedor_form(app: &mut App, ui: &mut Ui, i: Option<usize>) {
    heading(
        ui,
        if i.is_some() {
            "Fornecedor — Gravar / Alterar"
        } else {
            "Novo Fornecedor"
        },
    );
    let form = i.map(Form::Fornecedor).unwrap_or(Form::NovoFornecedor);
    let initial = match i {
        Some(i) => app.data.fornecedors[i].clone(),
        None => Fornecedor::default(),
    };
    let mut f = match app.take_form_draft(form, FormDraft::Fornecedor(initial)) {
        FormDraft::Fornecedor(f) => f,
        _ => unreachable!(),
    };
    text(ui, &mut f.nome, "Nome");
    text(ui, &mut f.telefone, "Telefone");
    text(ui, &mut f.email, "E-mail");
    text_w(ui, &mut f.endereco, "Endereço", 330.0);
    ui.label("Contas Bancárias:");
    for (ci, c) in f.contas.iter().enumerate() {
        ui.horizontal(|ui| {
            ui.label(c.banco.clone());
            ui.label(c.agencia.clone());
            ui.label(c.conta.clone());
            ui.label(c.email.clone());
            if i.is_some() {
                if ui.button("Editar Conta").clicked() {
                    app.form = Some(Form::ContaBanc(i.unwrap(), ci));
                }
            }
        });
    }
    if ui.button("Nova Conta Bancária").clicked() {
        f.contas.push(ContaBancaria::default());
    }
    match gravar_cancela(ui) {
        FormAction::Save => {
            match i {
                Some(i) => app.data.fornecedors[i] = f,
                None => {
                    f.id = next_id(
                        "FN",
                        app.data.fornecedors.iter().map(|record| record.id.clone()),
                    );
                    app.data.fornecedors.push(f);
                }
            }
            app.clear_form_draft();
            app.form = None;
        }
        FormAction::Cancel => {
            app.clear_form_draft();
            app.form = None;
        }
        FormAction::None => app.keep_form_draft(form, FormDraft::Fornecedor(f)),
    }
}

fn conta_form(app: &mut App, ui: &mut Ui, fi: usize, ci: usize) {
    heading(ui, "Conta Bancária");
    let form = Form::ContaBanc(fi, ci);
    let initial = match app.data.fornecedors.get(fi).and_then(|f| f.contas.get(ci)) {
        Some(c) => c.clone(),
        None => ContaBancaria::default(),
    };
    let mut c = match app.take_form_draft(form, FormDraft::ContaBancaria(initial)) {
        FormDraft::ContaBancaria(c) => c,
        _ => unreachable!(),
    };
    text(ui, &mut c.banco, "Banco");
    text(ui, &mut c.agencia, "Agência");
    text(ui, &mut c.conta, "Conta");
    text(ui, &mut c.email, "E-mail");
    match gravar_cancela(ui) {
        FormAction::Save => {
            if let Some(slot) = app
                .data
                .fornecedors
                .get_mut(fi)
                .and_then(|supplier| supplier.contas.get_mut(ci))
            {
                *slot = c;
            }
            app.form_drafts.remove(&form);
            app.form = Some(Form::Fornecedor(fi));
        }
        FormAction::Cancel => {
            app.form_drafts.remove(&form);
            app.form = Some(Form::Fornecedor(fi));
        }
        FormAction::None => app.keep_form_draft(form, FormDraft::ContaBancaria(c)),
    }
}

// ---------------------------------------------------------------- financeiro

fn financeiro(app: &mut App, ui: &mut Ui) {
    heading(ui, "Financeiro");
    ui.label(RichText::new("Contas Bancárias").strong());
    ScrollArea::vertical().max_height(130.0).show(ui, |ui| {
        egui::Grid::new("grid_contas_fin")
            .striped(true)
            .show(ui, |ui| {
                headers(ui, &["Banco", "Agência", "Conta", "E-mail"]);
                for f in app.data.fornecedors.iter() {
                    for c in f.contas.iter() {
                        ui.label(&c.banco);
                        ui.label(&c.agencia);
                        ui.label(&c.conta);
                        ui.label(&c.email);
                        ui.end_row();
                    }
                }
            });
    });
    ui.add_space(4.0);
    ui.label(RichText::new("Eventos Financeiros").strong());
    let eventos = app.data.eventos.clone();
    ScrollArea::vertical().max_height(200.0).show(ui, |ui| {
        egui::Grid::new("grid_eventos")
            .striped(true)
            .show(ui, |ui| {
                headers(
                    ui,
                    &["Código", "Data", "Tipo", "Descrição", "Valor", "Status", ""],
                );
                for (i, e) in eventos.iter().enumerate() {
                    ui.label(&e.id);
                    ui.label(&e.data);
                    ui.label(&e.tipo);
                    ui.label(&e.descricao);
                    ui.label(f2s(e.valor));
                    ui.label(&e.status);
                    if ui.button("Detalhes").clicked() {
                        app.form = Some(Form::EventoFin(i));
                    }
                    ui.end_row();
                }
            });
    });
    if ui.button("Novo Evento").clicked() {
        app.form = Some(Form::NovoEvento);
    }
    if let Some(Form::NovoEvento) = app.form {
        evento_fin_form(app, ui, None);
    }
    if let Some(Form::EventoFin(i)) = app.form {
        evento_fin_form(app, ui, Some(i));
    }
}

fn evento_fin_form(app: &mut App, ui: &mut Ui, i: Option<usize>) {
    heading(
        ui,
        if i.is_some() {
            "Evento Financeiro — Gravar / Alterar"
        } else {
            "Novo Evento Financeiro"
        },
    );
    let form = i.map(Form::EventoFin).unwrap_or(Form::NovoEvento);
    let mut initial = match i {
        Some(i) => app.data.eventos[i].clone(),
        None => EventoFin::default(),
    };
    if i.is_none() {
        initial.id = next_id(
            "EV",
            app.data.eventos.iter().map(|record| record.id.clone()),
        );
        initial.data = today();
    }
    let mut e = match app.take_form_draft(form, FormDraft::EventoFinanceiro(initial)) {
        FormDraft::EventoFinanceiro(e) => e,
        _ => unreachable!(),
    };
    let tipos = [
        "Entrada".to_string(),
        "Saída".to_string(),
        "Receber".to_string(),
        "Pagar".to_string(),
    ];
    combo(ui, "e_tipo", "Tipo", &mut e.tipo, &tipos);
    text(ui, &mut e.descricao, "Descrição");
    num(ui, &mut e.valor, "Valor");
    let status = [
        "Aberto".to_string(),
        "Pago".to_string(),
        "Recebido".to_string(),
        "Concluído".to_string(),
    ];
    combo(ui, "e_status", "Status", &mut e.status, &status);
    match gravar_cancela(ui) {
        FormAction::Save => {
            match i {
                Some(i) => app.data.eventos[i] = e,
                None => app.data.eventos.push(e),
            }
            app.clear_form_draft();
            app.form = None;
        }
        FormAction::Cancel => {
            app.clear_form_draft();
            app.form = None;
        }
        FormAction::None => app.keep_form_draft(form, FormDraft::EventoFinanceiro(e)),
    }
}

// ---------------------------------------------------------------- administração

fn administra(app: &mut App, ui: &mut Ui) {
    heading(ui, "Administração");
    ui.label(RichText::new("Usuários").strong());
    let usus = app.data.usuarios.clone();
    ScrollArea::vertical().max_height(150.0).show(ui, |ui| {
        egui::Grid::new("grid_usuarios")
            .striped(true)
            .show(ui, |ui| {
                headers(ui, &["Nome", "Login", "Tipo", ""]);
                for (i, u) in usus.iter().enumerate() {
                    ui.label(&u.nome);
                    ui.label(&u.login);
                    ui.label(&u.tipo);
                    if ui.button("Detalhes").clicked() {
                        app.form = Some(Form::Usuario(i));
                    }
                    ui.end_row();
                }
            });
    });
    ui.add_space(6.0);
    if ui.button("Novo Usuário").clicked() {
        app.form = Some(Form::NovoUsuario);
    }
    if let Some(Form::NovoUsuario) = app.form {
        usuario_form(app, ui, None);
    }
    if let Some(Form::Usuario(i)) = app.form {
        usuario_form(app, ui, Some(i));
    }
    ui.add_space(10.0);
    ui.label(RichText::new("Sincronização Automática").strong());
    ui.horizontal(|ui| {
        if ui
            .button("Testar Sincronização de Estoque Online")
            .clicked()
        {
            ui.label("Teste enviado: aguarde alerta no monitor.");
        }
        if ui.button("Sincronizar Estoque Online").clicked() {
            ui.label("Sincronização iniciada.");
        }
    });
    ui.add_space(4.0);
    ui.add_space(10.0);
    egui::Frame::group(ui.style()).show(ui, |ui| {
        ui.label(RichText::new("Banco de dados local").strong().size(15.0));
        ui.label(app.storage.path().display().to_string());
        ui.label(RichText::new("Cadastros são salvos automaticamente neste arquivo SQLite.")
            .color(Color32::from_rgb(103, 116, 132)));
        ui.add_space(10.0);
        ui.label(RichText::new("Criar backup portátil").strong());
        ui.horizontal(|ui| {
            ui.add(
                egui::TextEdit::singleline(&mut app.backup_path)
                    .hint_text("Caminho do arquivo .zip")
                    .desired_width(470.0),
            );
            if ui.button("Criar backup ZIP").clicked() {
                app.create_backup();
            }
        });
        ui.add_space(8.0);
        ui.label(RichText::new("Importar dados de um ZIP").strong());
        ui.horizontal(|ui| {
            ui.add(
                egui::TextEdit::singleline(&mut app.import_path)
                    .hint_text("Caminho do arquivo .zip")
                    .desired_width(470.0),
            );
            if ui.button("Importar ZIP").clicked() {
                app.restore_pending = true;
                app.storage_notice = None;
            }
        });
        ui.label(
            RichText::new("Aceita o intercâmbio JSON versionado dentro do ZIP; a importação substitui os dados atuais.")
                .size(11.0)
                .color(Color32::from_rgb(103, 116, 132)),
        );
        if app.restore_pending {
            ui.add_space(6.0);
            ui.colored_label(
                Color32::from_rgb(170, 52, 48),
                "Atenção: os cadastros atuais serão substituídos. Um backup será criado antes da importação.",
            );
            ui.horizontal(|ui| {
                if ui.button("Confirmar importação").clicked() {
                    app.restore_backup();
                }
                if ui.button("Cancelar").clicked() {
                    app.restore_pending = false;
                }
            });
        }
        if let Some(notice) = app.storage_notice.as_ref() {
            ui.add_space(5.0);
            ui.label(notice);
        }
        if let Some(error) = app.storage_error.as_ref() {
            ui.add_space(5.0);
            ui.colored_label(
                Color32::from_rgb(170, 52, 48),
                format!("Banco indisponível: {error}"),
            );
        }
        ui.add_space(8.0);
        ui.checkbox(&mut app.data.auto_backup_on, "Backup local automático diário");
    });
}

fn usuario_form(app: &mut App, ui: &mut Ui, i: Option<usize>) {
    heading(
        ui,
        if i.is_some() {
            "Usuário — Gravar / Alterar"
        } else {
            "Novo Usuário"
        },
    );
    let form = i.map(Form::Usuario).unwrap_or(Form::NovoUsuario);
    let initial = match i {
        Some(i) => app.data.usuarios[i].clone(),
        None => Usuario::default(),
    };
    let mut u = match app.take_form_draft(form, FormDraft::Usuario(initial)) {
        FormDraft::Usuario(u) => u,
        _ => unreachable!(),
    };
    text(ui, &mut u.nome, "Nome");
    text(ui, &mut u.login, "Login");
    text(ui, &mut u.senha, "Senha");
    let tipos = [
        "Administrador".to_string(),
        "Atendente".to_string(),
        "Vendedor".to_string(),
    ];
    combo(ui, "u_tipo", "Tipo de Usuário", &mut u.tipo, &tipos);
    if u.tipo.is_empty() {
        u.tipo = "Atendente".into();
    }
    match gravar_cancela(ui) {
        FormAction::Save => {
            match i {
                Some(i) => app.data.usuarios[i] = u,
                None => {
                    u.id = next_id(
                        "US",
                        app.data.usuarios.iter().map(|record| record.id.clone()),
                    );
                    app.data.usuarios.push(u);
                }
            }
            app.clear_form_draft();
            app.form = None;
        }
        FormAction::Cancel => {
            app.clear_form_draft();
            app.form = None;
        }
        FormAction::None => app.keep_form_draft(form, FormDraft::Usuario(u)),
    }
}

// ---------------------------------------------------------------- fiscal

fn fiscal(app: &mut App, ui: &mut Ui) {
    heading(ui, "Fiscal");
    ui.label(RichText::new("Emissão de NFe/NFCe").strong());
    if ui.button("Gerar NFE de Estoque").clicked() {
        let mut v = Venda {
            id: next_id("V", app.data.vendas.iter().map(|record| record.id.clone())),
            data: today(),
            cliente: "(Público) - Estoque".into(),
            status: "NFe emitida".into(),
            ..Default::default()
        };
        for p in app.data.produtos.iter() {
            if !p.inativo && p.estoque > 0.0 {
                let mut it = Item {
                    descricao: p.descricao.clone(),
                    unidade: p.unidade.clone(),
                    qtd: p.estoque,
                    valor: p.custo,
                    ..Default::default()
                };
                it.icms = p.icms;
                it.ipi = p.ipi;
                v.itens.push(it);
                break;
            }
        }
        v.subtotal = v.itens.iter().map(|i| i.total()).sum();
        v.total = v.subtotal;
        app.data.vendas.push(v);
    }
    ui.add_space(4.0);
    ui.label("Emissão Automática (Sinteg)");
    ui.label("Status: sem conexão com SEFAZ / Sinteg (est.).");
    ui.add_space(6.0);
    ui.label(RichText::new("Importação de NFC-e / NF-Sé").strong());
    ScrollArea::vertical().max_height(140.0).show(ui, |ui| {
        egui::Grid::new("grid_nfe_import")
            .striped(true)
            .show(ui, |ui| {
                headers(ui, &["Código", "Cliente", "Valor", "Status", ""]);
                for v in app
                    .data
                    .vendas
                    .iter()
                    .filter(|v| v.status.starts_with("NFe"))
                {
                    ui.label(&v.id);
                    ui.label(&v.cliente);
                    ui.label(f2s(v.total));
                    ui.label(&v.status);
                    ui.label("");
                    ui.end_row();
                }
            });
    });
    ui.add_space(6.0);
    ui.label(RichText::new("Emissores").strong());
    ScrollArea::vertical().max_height(110.0).show(ui, |ui| {
        egui::Grid::new("grid_emissores")
            .striped(true)
            .show(ui, |ui| {
                headers(ui, &["CNPJ / Nome", "Obs"]);
                ui.label("CICOM");
                ui.label("Sistema de gestão (est.)");
                ui.end_row();
                ui.label("SINCRO");
                ui.label("Sistema Nacional de Informação e Vendas (est.)");
                ui.end_row();
            });
    });
}

// ---------------------------------------------------------------- favoritos

fn favoritos(app: &mut App, ui: &mut Ui) {
    heading(ui, "Favoritos");
    if app.data.favoritos.is_empty() {
        ui.label("Nenhum favorito cadastrado.");
    }
    let favoritos = app.data.favoritos.clone();
    ScrollArea::vertical().show(ui, |ui| {
        egui::Grid::new("grid_favoritos")
            .striped(true)
            .show(ui, |ui| {
                headers(ui, &["Favorito", "Abrir", "Remover"]);
                for f in favoritos.iter() {
                    ui.label(f);
                    if ui.button("Abrir").clicked() {
                        if let Some(s) = Section::from_title(f) {
                            app.active = Some(s);
                        }
                    }
                    if ui.button("Remover").clicked() {
                        app.data.favoritos.retain(|x| x != f);
                    }
                    ui.end_row();
                }
            });
    });
    ui.add_space(6.0);
    ui.label("Favoritar Tela:");
    let mut fav = app.data.favorito_picker.clone();
    let all: Vec<String> = Section::ALL.iter().map(|s| s.title().to_string()).collect();
    combo(ui, "fav_combo", "", &mut fav, &all);
    app.data.favorito_picker = fav;
    if ui.button("Favoritar").clicked() {
        let name = app.data.favorito_picker.clone();
        if !name.is_empty() && !app.data.favoritos.contains(&name) {
            app.data.favoritos.push(name);
        }
    }
}

// ---------------------------------------------------------------- data

fn today() -> String {
    let secs = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_secs() as i64)
        .unwrap_or(0);
    let days = secs.div_euclid(86400);
    let (y, m, d) = days_to_date(days);
    format!("{y:04}-{m:02}-{d:02}")
}

fn days_to_date(days: i64) -> (i64, u32, u32) {
    let mut d = days;
    let mut y = 1970i64;
    loop {
        let len = if is_leap(y) { 366 } else { 365 };
        if d < len {
            break;
        }
        d -= len;
        y += 1;
    }
    let month_lens = [
        31u32,
        if is_leap(y) { 29 } else { 28 },
        31,
        30,
        31,
        30,
        31,
        31,
        30,
        31,
        30,
        31,
    ];
    let mut m = 1u32;
    let mut day = d as u32 + 1;
    for l in month_lens {
        if day <= l {
            break;
        }
        day -= l;
        m += 1;
    }
    (y, m, day)
}

fn is_leap(y: i64) -> bool {
    (y % 4 == 0 && y % 100 != 0) || y % 400 == 0
}
