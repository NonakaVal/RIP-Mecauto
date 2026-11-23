//! Dados do MecAuto (réplica). Modelo de dados e dados de semente para as telas.
//!
//! Os nomes, campos e semântica vêm da observação da aplicação original
//! (formulários como TFRMPRINCIPAL, TFRMCLIENTES, TFRMSALDOSESTOQUE, ...).

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct Cliente {
    pub id: String,
    pub nome: String,
    pub telefone: String,
    pub email: String,
    pub endereco: String,
    pub bairro: String,
    pub cidade: String,
    pub uf: String,
    pub cep: String,
    pub grupos: Vec<String>,
    pub saldo: f32,
    pub fotos: bool,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct Veiculo {
    pub placa: String,
    pub modelo: String,
    pub tipo: String,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct Produto {
    pub id: String,
    pub codigo: String,
    pub codigo_loja: String,
    pub codigo_fabricante: String,
    pub codigo_barras: String,
    pub codigo_sistema: String,
    pub descricao: String,
    pub aplicacao: String,
    pub montadora: String,
    pub producao: String,
    pub destino: String,
    pub unidade: String,
    pub grupo: String,
    pub subgrupo: String,
    pub variacoes: String,
    pub fabricante: String,
    pub fornecedor: String,
    pub local: String,
    pub embalagem: String,
    pub obs: String,
    pub custo: f32,
    pub preco: f32,
    pub margem: f32,
    pub icms: f32,
    pub ipi: f32,
    pub st: bool,
    pub cfop: String,
    pub estoque: f64,
    pub estoque_minimo: f64,
    pub estoque_maximo: f64,
    pub inativo: bool,
    pub foto: bool,
    pub imagem: String,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct Item {
    pub descricao: String,
    pub unidade: String,
    pub qtd: f64,
    pub valor: f32,
    pub icms: f32,
    pub ipi: f32,
}

impl Item {
    pub fn total(&self) -> f32 {
        (self.qtd as f32) * self.valor
    }
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct Venda {
    pub id: String,
    pub data: String,
    pub cliente: String,
    pub veiculo: String,
    pub status: String,
    pub itens: Vec<Item>,
    pub labor: Vec<Item>,
    pub subtotal: f32,
    pub icms_total: f32,
    pub ipi_total: f32,
    pub total: f32,
    pub pagou_pix: bool,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct OS {
    pub id: String,
    pub data_abertura: String,
    pub data_conclusao: String,
    pub status: String,
    pub cliente: String,
    pub veiculo: String,
    pub tecnico: String,
    pub itens: Vec<Item>,
    pub obs: String,
    pub total: f32,
    pub pagou_pix: bool,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct BoxCaixa {
    pub id: String,
    pub descricao: String,
    pub aberto: bool,
    pub valor_inicial: f32,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct MovCaixa {
    pub id: String,
    pub data: String,
    pub tipo: String,
    pub descricao: String,
    pub valor: f32,
    pub caixa: String,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct MovEstoque {
    pub id: String,
    pub produto_id: String,
    pub produto_codigo: String,
    pub produto_descricao: String,
    pub data: String,
    pub tipo: String,
    pub quantidade: f64,
    pub observacao: String,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct Profissional {
    pub id: String,
    pub nome: String,
    pub telefone: String,
    pub especialidade: String,
    pub comissao: f32,
    pub ativo: bool,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct ContaBancaria {
    pub banco: String,
    pub agencia: String,
    pub conta: String,
    pub email: String,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct Fornecedor {
    pub id: String,
    pub nome: String,
    pub telefone: String,
    pub email: String,
    pub endereco: String,
    pub contas: Vec<ContaBancaria>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct Usuario {
    pub id: String,
    pub nome: String,
    pub login: String,
    pub senha: String,
    pub tipo: String,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct EventoFin {
    pub id: String,
    pub data: String,
    pub tipo: String,
    pub descricao: String,
    pub valor: f32,
    pub status: String,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct MecAutoData {
    pub clientes: Vec<Cliente>,
    pub produtos: Vec<Produto>,
    pub veiculos: Vec<Veiculo>,
    pub vendas: Vec<Venda>,
    pub ordens: Vec<OS>,
    pub caixas: Vec<BoxCaixa>,
    pub movimentos: Vec<MovCaixa>,
    pub movimentos_estoque: Vec<MovEstoque>,
    pub profissionais: Vec<Profissional>,
    #[serde(alias = "fornecedores")]
    pub fornecedors: Vec<Fornecedor>,
    pub usuarios: Vec<Usuario>,
    pub eventos: Vec<EventoFin>,
    pub favoritos: Vec<String>,
    pub auto_backup_on: bool,
    pub favorito_picker: String,
}

fn item(descricao: &str, unidade: &str, qtd: f64, valor: f32) -> Item {
    Item {
        descricao: descricao.into(),
        unidade: unidade.into(),
        qtd,
        valor,
        icms: 0.0,
        ipi: 0.0,
    }
}

impl MecAutoData {
    pub fn registrar_entrada_estoque(
        &mut self,
        produto_index: usize,
        quantidade: f64,
        observacao: String,
        data: String,
    ) -> Result<(), &'static str> {
        if !quantidade.is_finite() || quantidade <= 0.0 {
            return Err("Informe uma quantidade maior que zero.");
        }
        let produto = self
            .produtos
            .get(produto_index)
            .ok_or("O produto selecionado não existe mais.")?;
        let next_id = self
            .movimentos_estoque
            .iter()
            .filter_map(|movimento| {
                movimento
                    .id
                    .strip_prefix("ME-")
                    .and_then(|s| s.parse::<u64>().ok())
            })
            .max()
            .unwrap_or(0)
            + 1;
        let movement = MovEstoque {
            id: format!("ME-{next_id:03}"),
            produto_id: produto.id.clone(),
            produto_codigo: produto.codigo.clone(),
            produto_descricao: produto.descricao.clone(),
            data,
            tipo: "Entrada".into(),
            quantidade,
            observacao: observacao.trim().to_owned(),
        };
        self.produtos[produto_index].estoque += quantidade;
        self.movimentos_estoque.push(movement);
        Ok(())
    }

    pub fn registrar_saida_estoque(
        &mut self,
        produto_index: usize,
        quantidade: f64,
        observacao: String,
        data: String,
    ) -> Result<(), &'static str> {
        if !quantidade.is_finite() || quantidade <= 0.0 {
            return Err("Informe uma quantidade maior que zero.");
        }
        let produto = self
            .produtos
            .get(produto_index)
            .ok_or("O produto selecionado não existe mais.")?;
        if !produto.estoque.is_finite() || quantidade > produto.estoque {
            return Err("A quantidade não pode ser maior que o estoque disponível.");
        }

        let next_id = self
            .movimentos_estoque
            .iter()
            .filter_map(|movimento| {
                movimento
                    .id
                    .strip_prefix("ME-")
                    .and_then(|suffix| suffix.parse::<u64>().ok())
            })
            .max()
            .unwrap_or(0)
            + 1;
        let movement = MovEstoque {
            id: format!("ME-{next_id:03}"),
            produto_id: produto.id.clone(),
            produto_codigo: produto.codigo.clone(),
            produto_descricao: produto.descricao.clone(),
            data,
            tipo: "Saída".into(),
            quantidade,
            observacao: observacao.trim().to_owned(),
        };

        self.produtos[produto_index].estoque -= quantidade;
        self.movimentos_estoque.push(movement);
        Ok(())
    }

    pub fn seed() -> Self {
        let mut d = MecAutoData::default();

        d.clientes = vec![
            Cliente {
                id: "C-001".into(),
                nome: "Fernanda Bulletin".into(),
                telefone: "(11) 98888-7777".into(),
                email: "fernanda@bulletin.com.br".into(),
                endereco: "Av. Paulista, 1500 - Bairro Bela Vista".into(),
                bairro: "Bela Vista".into(),
                cidade: "São Paulo".into(),
                uf: "SP".into(),
                cep: "01000-000".into(),
                grupos: vec!["Clientes Preferenciais".into()],
                saldo: 250.00,
                fotos: true,
            },
            Cliente {
                id: "C-002".into(),
                nome: "Márcio Chaves".into(),
                telefone: "(11) 97777-6666".into(),
                email: "marcio.chaves@email.com".into(),
                endereco: "Rua das Flores, 200 - Jd. São Paulo".into(),
                bairro: "Jd. São Paulo".into(),
                cidade: "São Paulo".into(),
                uf: "SP".into(),
                cep: "01234-567".into(),
                grupos: vec!["Meu Garagem".into(), "Grupo Trucks".into()],
                saldo: 0.0,
                fotos: false,
            },
            Cliente {
                id: "C-003".into(),
                nome: "Oficina Vizinha Ltda.".into(),
                telefone: "(11) 96666-5555".into(),
                email: "ofvizinha@gmail.com".into(),
                endereco: "Vila Nova Conceição, 1500".into(),
                bairro: "Vila Nova Conceição".into(),
                cidade: "São Paulo".into(),
                uf: "SP".into(),
                cep: "04578-110".into(),
                grupos: vec!["Grupo Trucks".into()],
                saldo: -120.00,
                fotos: false,
            },
        ];

        d.veiculos = vec![
            Veiculo {
                placa: "ABC-1234".into(),
                modelo: "Toyota Corolla 2.0 Flex".into(),
                tipo: "Carro".into(),
            },
            Veiculo {
                placa: "DEF-5678".into(),
                modelo: "Volkswagen Virtus 1.0".into(),
                tipo: "Carro".into(),
            },
        ];

        d.produtos = Vec::new();

        d.vendas = vec![Venda {
            id: "V-001".into(),
            data: "2026-11-10".into(),
            cliente: "Fernanda Bulletin".into(),
            veiculo: "ABC-1234".into(),
            status: "Concluída".into(),
            itens: vec![item("Filtro de Ar 1F20", "Un", 1.0, 89.00)],
            labor: vec![item("Manutenção de Chegada", "h", 2.0, 180.00)],
            subtotal: 449.00,
            icms_total: 0.0,
            ipi_total: 0.0,
            total: 449.00,
            pagou_pix: true,
        }];

        d.ordens = vec![OS {
            id: "OS-001".into(),
            data_abertura: "2026-11-12".into(),
            data_conclusao: "2026-11-13".into(),
            status: "Concluída".into(),
            cliente: "Márcio Chaves".into(),
            veiculo: "ABC-1234".into(),
            tecnico: "Roberto Rios".into(),
            itens: vec![
                item("Troca de pastilha traseira", "h", 2.5, 140.00),
                item("Pastilha de Freio P 01.012.064", "Conjunto", 1.0, 97.00),
            ],
            obs: "Foi observado ruído na carcaça traseira, dovado atendimento.".into(),
            total: 497.00,
            pagou_pix: false,
        }];

        d.caixas = vec![
            BoxCaixa {
                id: "CX-001".into(),
                descricao: "Caixa A - Loja Principal".into(),
                aberto: false,
                valor_inicial: 0.00,
            },
            BoxCaixa {
                id: "CX-002".into(),
                descricao: "Caixa B - Av. Paulista".into(),
                aberto: true,
                valor_inicial: 150.00,
            },
        ];

        d.movimentos = vec![
            MovCaixa {
                id: "MV-001".into(),
                data: "2026-11-08".into(),
                tipo: "Entrada".into(),
                descricao: "Abertura de Caixa".into(),
                valor: 150.00,
                caixa: "CX-002".into(),
            },
            MovCaixa {
                id: "MV-002".into(),
                data: "2026-11-12".into(),
                tipo: "Entrada".into(),
                descricao: "Recebimento de cliente".into(),
                valor: 642.00,
                caixa: "CX-002".into(),
            },
        ];

        d.profissionais = vec![
            Profissional {
                id: "PF-001".into(),
                nome: "Roberto Rios".into(),
                telefone: "(11) 98888-6666".into(),
                especialidade: "Mecânico de Manutenção".into(),
                comissao: 5.0,
                ativo: true,
            },
            Profissional {
                id: "PF-002".into(),
                nome: "Tânia Alves".into(),
                telefone: "(11) 97777-5555".into(),
                especialidade: "Diagnóstico".into(),
                comissao: 3.0,
                ativo: true,
            },
            Profissional {
                id: "PF-003".into(),
                nome: "Luís Mendes".into(),
                telefone: "(11) 96666-4444".into(),
                especialidade: "Elétrico".into(),
                comissao: 4.0,
                ativo: true,
            },
        ];

        d.fornecedors = vec![
            Fornecedor {
                id: "FN-001".into(),
                nome: "Distribuidora Meca".into(),
                telefone: "(11) 3042-5368".into(),
                email: "vendas@distribuidora-meca.com.br".into(),
                endereco: "Rua das Indústrias, km 12, industrial".into(),
                contas: vec![ContaBancaria {
                    banco: "Banrisul".into(),
                    agencia: "0121".into(),
                    conta: "5678-2".into(),
                    email: "financeiro@distribuidora-meca.com.br".into(),
                }],
            },
            Fornecedor {
                id: "FN-002".into(),
                nome: "Valeos Filtros".into(),
                telefone: "(11) 98888-3333".into(),
                email: "comercial@valeos.com.br".into(),
                endereco: "Avenida Paulista, 3000 - 12o andar".into(),
                contas: vec![ContaBancaria {
                    banco: "Itaú".into(),
                    agencia: "2345".into(),
                    conta: "6789-5".into(),
                    email: "compras@valeos.com.br".into(),
                }],
            },
        ];

        d.usuarios = vec![
            Usuario {
                id: "US-001".into(),
                nome: "Ana Silva".into(),
                login: "ana.silva".into(),
                senha: "12345".into(),
                tipo: "Administrador".into(),
            },
            Usuario {
                id: "US-002".into(),
                nome: "Carlos Dulce".into(),
                login: "c.dulce".into(),
                senha: "67890".into(),
                tipo: "Atendente".into(),
            },
        ];

        d.eventos = vec![EventoFin {
            id: "EV-001".into(),
            data: "2026-11-10".into(),
            tipo: "Receber".into(),
            descricao: "Recebimento de cliente - V-001".into(),
            valor: 449.00,
            status: "Aberto".into(),
        }];

        d.favoritos = vec![
            "Vendas".into(),
            "Oficina".into(),
            "Estoque".into(),
            "Produtos".into(),
            "Clientes".into(),
            "Financeiro".into(),
        ];

        d
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn seed_starts_without_demo_products() {
        assert!(MecAutoData::seed().produtos.is_empty());
    }

    fn data_with_product() -> MecAutoData {
        let mut data = MecAutoData::seed();
        data.produtos.push(Produto {
            id: "TEST-001".into(),
            codigo: "TEST-001".into(),
            descricao: "Produto de teste".into(),
            estoque: 5.0,
            ..Default::default()
        });
        data
    }

    #[test]
    fn older_products_without_image_paths_keep_the_default() {
        let product: Produto = serde_json::from_str(r#"{"codigo":"123","foto":true}"#).unwrap();

        assert!(product.foto);
        assert!(product.imagem.is_empty());
    }

    #[test]
    fn stock_exit_rejects_invalid_quantities_without_changing_stock() {
        let mut data = data_with_product();
        let original = data.clone();

        assert!(data
            .registrar_saida_estoque(0, 0.0, String::new(), "2026-11-23".into())
            .is_err());
        assert!(data
            .registrar_saida_estoque(0, f64::NAN, String::new(), "2026-11-23".into())
            .is_err());
        assert!(data
            .registrar_saida_estoque(
                0,
                data.produtos[0].estoque + 1.0,
                String::new(),
                "2026-11-23".into()
            )
            .is_err());
        assert_eq!(data, original);
    }

    #[test]
    fn stock_exit_decrements_quantity_and_records_auditable_movement() {
        let mut data = data_with_product();
        let original_stock = data.produtos[0].estoque;

        data.registrar_saida_estoque(
            0,
            2.0,
            "  Uso em ordem de serviço  ".into(),
            "2026-11-23".into(),
        )
        .unwrap();

        assert_eq!(data.produtos[0].estoque, original_stock - 2.0);
        assert_eq!(
            data.movimentos_estoque,
            vec![MovEstoque {
                id: "ME-001".into(),
                produto_id: data.produtos[0].id.clone(),
                produto_codigo: data.produtos[0].codigo.clone(),
                produto_descricao: data.produtos[0].descricao.clone(),
                data: "2026-11-23".into(),
                tipo: "Saída".into(),
                quantidade: 2.0,
                observacao: "Uso em ordem de serviço".into(),
            }]
        );
    }
}
