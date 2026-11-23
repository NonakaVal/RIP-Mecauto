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
    pub fn legacy_products() -> Vec<Produto> {
        serde_json::from_str(include_str!("products_legacy.json"))
            .expect("catálogo legado de produtos deve ser JSON válido")
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

        d.produtos = vec![
            Produto {
                id: "P-001".into(),
                codigo: "001".into(),
                descricao: "Óleo de Motor 5W40, 20 L".into(),
                unidade: "Litro".into(),
                grupo: "Óleos".into(),
                subgrupo: "Motor".into(),
                fabricante: "Shell".into(),
                fornecedor: "Distribuidora Meca".into(),
                local: "Depósito A".into(),
                embalagem: "20 L".into(),
                custo: 118.00,
                preco: 185.00,
                margem: 45.0,
                icms: 18.0,
                ipi: 0.0,
                st: false,
                cfop: "5.949".into(),
                estoque: 12.0,
                inativo: false,
                foto: false,
                ..Default::default()
            },
            Produto {
                id: "P-002".into(),
                codigo: "002".into(),
                descricao: "Filtro de Ar 1F20".into(),
                unidade: "Un".into(),
                grupo: "Filtros".into(),
                subgrupo: "Cabine".into(),
                fabricante: "Mahle".into(),
                fornecedor: "Distribuidora Meca".into(),
                local: "Depósito A".into(),
                embalagem: "Cx. 2".into(),
                custo: 44.00,
                preco: 89.00,
                margem: 51.0,
                icms: 18.0,
                ipi: 0.0,
                st: false,
                cfop: "5.949".into(),
                estoque: 8.0,
                inativo: false,
                foto: false,
                ..Default::default()
            },
            Produto {
                id: "P-003".into(),
                codigo: "003".into(),
                descricao: "Freio de Disco 11011430".into(),
                unidade: "Un".into(),
                grupo: "Freios".into(),
                subgrupo: "Disco".into(),
                fabricante: "Graynl".into(),
                fornecedor: "Distribuidora Meca".into(),
                local: "Depósito B".into(),
                embalagem: "Unitário".into(),
                custo: 76.00,
                preco: 141.00,
                margem: 46.0,
                icms: 18.0,
                ipi: 5.0,
                st: false,
                cfop: "5.949".into(),
                estoque: 3.0,
                inativo: false,
                foto: false,
                ..Default::default()
            },
            Produto {
                id: "P-004".into(),
                codigo: "004".into(),
                descricao: "Pastilha de Freio P 01.012.064".into(),
                unidade: "Conjunto".into(),
                grupo: "Freios".into(),
                subgrupo: "Pastilha".into(),
                fabricante: "Duralast".into(),
                fornecedor: "Distribuidora Meca".into(),
                local: "Depósito B".into(),
                embalagem: "Unitário".into(),
                custo: 52.00,
                preco: 97.00,
                margem: 46.0,
                icms: 18.0,
                ipi: 5.0,
                st: false,
                cfop: "5.949".into(),
                estoque: 5.0,
                inativo: false,
                foto: false,
                ..Default::default()
            },
            Produto {
                id: "P-005".into(),
                codigo: "005".into(),
                descricao: "Correia de Distribuição 6PK1200".into(),
                unidade: "Un".into(),
                grupo: "Correias".into(),
                subgrupo: "Distribuição".into(),
                fabricante: "Continental".into(),
                fornecedor: "Distribuidora Meca".into(),
                local: "Depósito A".into(),
                embalagem: "Unitário".into(),
                custo: 30.00,
                preco: 58.00,
                margem: 48.0,
                icms: 18.0,
                ipi: 0.0,
                st: false,
                cfop: "5.949".into(),
                estoque: 10.0,
                inativo: false,
                foto: false,
                ..Default::default()
            },
        ];
        d.produtos.extend(Self::legacy_products());

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
