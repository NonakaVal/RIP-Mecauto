use crate::data::*;
use rusqlite::{params, Connection, OptionalExtension, Transaction};
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fs::{self, File, OpenOptions};
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};
use zip::write::SimpleFileOptions;
use zip::{CompressionMethod, ZipArchive, ZipWriter};

pub type StorageResult<T> = Result<T, Box<dyn Error + Send + Sync>>;

const SCHEMA_VERSION: i64 = 1;
const PORTABLE_FORMAT: &str = "mecauto-portable-data";
const PORTABLE_VERSION: u32 = 1;
const DEMO_PRODUCTS_MIGRATION_KEY: &str = "demo_products_removed_v1";
const MAX_ZIP_ENTRIES: usize = 1024;
const MAX_ZIP_ENTRY_BYTES: u64 = 64 * 1024 * 1024;
const MAX_ZIP_TOTAL_BYTES: u64 = 128 * 1024 * 1024;

#[derive(Serialize, Deserialize)]
struct PortableBackup {
    format: String,
    version: u32,
    exported_at_unix: u64,
    data: MecAutoData,
}

pub struct Storage {
    connection: Connection,
    path: PathBuf,
}

impl Storage {
    pub fn open_default() -> StorageResult<(Self, MecAutoData)> {
        Self::open_at(database_path()?)
    }

    fn open_at(path: PathBuf) -> StorageResult<(Self, MecAutoData)> {
        if let Some(parent) = path.parent().filter(|path| !path.as_os_str().is_empty()) {
            fs::create_dir_all(parent)?;
        }
        let connection = Connection::open(&path)?;
        connection.execute_batch(
            "PRAGMA journal_mode = DELETE;
             PRAGMA synchronous = FULL;
             CREATE TABLE IF NOT EXISTS records (
                 entity TEXT NOT NULL,
                 record_id TEXT NOT NULL,
                 position INTEGER NOT NULL,
                 payload TEXT NOT NULL,
                 PRIMARY KEY (entity, record_id)
             );
             CREATE INDEX IF NOT EXISTS records_by_entity_position
                 ON records (entity, position);
             CREATE TABLE IF NOT EXISTS settings (
                 key TEXT PRIMARY KEY,
                 value TEXT NOT NULL
             );",
        )?;

        let version: i64 = connection.query_row("PRAGMA user_version", [], |row| row.get(0))?;
        if version != 0 && version != SCHEMA_VERSION {
            return Err(invalid_data(format!(
                "Versão do banco de dados não suportada: {version}."
            )));
        }
        connection.pragma_update(None, "user_version", SCHEMA_VERSION)?;

        let mut storage = Self { connection, path };
        let initialized: Option<i64> = storage
            .connection
            .query_row(
                "SELECT CAST(value AS INTEGER) FROM settings WHERE key = 'initialized'",
                [],
                |row| row.get(0),
            )
            .optional()?;

        let data = if initialized == Some(1) {
            let mut data = load_data(&storage.connection)?;
            let demo_products_removed: Option<String> = storage
                .connection
                .query_row(
                    "SELECT value FROM settings WHERE key = ?1",
                    [DEMO_PRODUCTS_MIGRATION_KEY],
                    |row| row.get(0),
                )
                .optional()?;
            if demo_products_removed.as_deref() != Some("1") {
                data.produtos
                    .retain(|product| !is_demo_seed_product_id(&product.id));
                data.movimentos_estoque
                    .retain(|movement| !is_demo_seed_product_id(&movement.produto_id));
                storage.save(&data)?;
            }
            data
        } else {
            let data = MecAutoData::seed();
            storage.save(&data)?;
            data
        };
        Ok((storage, data))
    }

    pub fn path(&self) -> &Path {
        &self.path
    }

    pub fn suggested_backup_path(&self) -> PathBuf {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|duration| duration.as_millis())
            .unwrap_or_default();
        let parent = self.path.parent().unwrap_or_else(|| Path::new("."));
        parent
            .join("backups")
            .join(format!("mecauto-backup-{timestamp}.zip"))
    }

    pub fn save(&mut self, data: &MecAutoData) -> StorageResult<()> {
        let transaction = self.connection.transaction()?;
        transaction.execute("DELETE FROM records", [])?;
        transaction.execute("DELETE FROM settings", [])?;

        save_collection(&transaction, "clientes", &data.clientes, |record| {
            record.id.clone()
        })?;
        save_collection(&transaction, "veiculos", &data.veiculos, |record| {
            record.placa.clone()
        })?;
        save_collection(&transaction, "produtos", &data.produtos, |record| {
            record.id.clone()
        })?;
        save_collection(&transaction, "vendas", &data.vendas, |record| {
            record.id.clone()
        })?;
        save_collection(&transaction, "ordens_servico", &data.ordens, |record| {
            record.id.clone()
        })?;
        save_collection(&transaction, "caixas", &data.caixas, |record| {
            record.id.clone()
        })?;
        save_collection(
            &transaction,
            "movimentos_caixa",
            &data.movimentos,
            |record| record.id.clone(),
        )?;
        save_collection(
            &transaction,
            "movimentos_estoque",
            &data.movimentos_estoque,
            |record| record.id.clone(),
        )?;
        save_collection(
            &transaction,
            "profissionais",
            &data.profissionais,
            |record| record.id.clone(),
        )?;
        save_collection(&transaction, "fornecedores", &data.fornecedors, |record| {
            record.id.clone()
        })?;
        save_collection(&transaction, "usuarios", &data.usuarios, |record| {
            record.id.clone()
        })?;
        save_collection(
            &transaction,
            "eventos_financeiros",
            &data.eventos,
            |record| record.id.clone(),
        )?;

        save_setting(&transaction, "favoritos", &data.favoritos)?;
        save_setting(&transaction, "auto_backup_on", &data.auto_backup_on)?;
        save_setting(&transaction, "favorito_picker", &data.favorito_picker)?;
        transaction.execute(
            "INSERT INTO settings (key, value) VALUES ('initialized', '1')",
            [],
        )?;
        transaction.execute(
            "INSERT INTO settings (key, value) VALUES (?1, '1')",
            [DEMO_PRODUCTS_MIGRATION_KEY],
        )?;
        transaction.commit()?;
        Ok(())
    }

    pub fn export_zip(&self, data: &MecAutoData, destination: &Path) -> StorageResult<()> {
        if destination.exists() {
            return Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::AlreadyExists,
                "O arquivo de destino já existe; escolha outro nome para preservar os backups.",
            )));
        }
        if let Some(parent) = destination
            .parent()
            .filter(|path| !path.as_os_str().is_empty())
        {
            fs::create_dir_all(parent)?;
        }
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_nanos();
        let temporary =
            destination.with_extension(format!("tmp-{}-{timestamp}", std::process::id()));
        let backup = PortableBackup {
            format: PORTABLE_FORMAT.into(),
            version: PORTABLE_VERSION,
            exported_at_unix: timestamp as u64 / 1_000_000_000,
            data: data.clone(),
        };
        let bytes = serde_json::to_vec(&backup)?;

        let result = (|| -> StorageResult<()> {
            let file = OpenOptions::new()
                .write(true)
                .create_new(true)
                .open(&temporary)?;
            let mut archive = ZipWriter::new(file);
            archive.start_file(
                "mecauto-export.json",
                SimpleFileOptions::default().compression_method(CompressionMethod::Deflated),
            )?;
            archive.write_all(&bytes)?;
            let file = archive.finish()?;
            file.sync_all()?;
            fs::rename(&temporary, destination)?;
            Ok(())
        })();
        if result.is_err() {
            let _ = fs::remove_file(&temporary);
        }
        result?;
        Ok(())
    }

    pub fn read_zip(source_path: &Path) -> StorageResult<MecAutoData> {
        if !source_path.is_file() {
            return Err(invalid_data("O arquivo ZIP não foi encontrado."));
        }
        let file = File::open(source_path)?;
        if file.metadata()?.len() > MAX_ZIP_TOTAL_BYTES {
            return Err(invalid_data(
                "O arquivo ZIP excede o limite seguro de tamanho.",
            ));
        }
        let mut archive = ZipArchive::new(file)?;
        if archive.len() > MAX_ZIP_ENTRIES {
            return Err(invalid_data("O ZIP excede o limite seguro de arquivos."));
        }

        let mut target_entry = None;
        let mut is_mecauto_native = false;
        let mut total_uncompressed = 0u64;
        for index in 0..archive.len() {
            let entry = archive.by_index(index)?;
            let name = entry.name().to_ascii_lowercase();
            total_uncompressed = total_uncompressed.saturating_add(entry.size());
            if entry.size() > MAX_ZIP_ENTRY_BYTES || total_uncompressed > MAX_ZIP_TOTAL_BYTES {
                return Err(invalid_data("O ZIP excede o limite seguro de tamanho."));
            }
            if name == "infobck.ini" {
                is_mecauto_native = true;
            } else if name.ends_with(".fbk") {
                is_mecauto_native = true;
            } else if name == "mecauto-export.json" || name == "mecauto.json" {
                if target_entry.replace(entry.name().to_owned()).is_some() {
                    return Err(invalid_data(
                        "O ZIP contém mais de um arquivo de dados MecAuto.",
                    ));
                }
            }
        }

        let Some(entry_name) = target_entry else {
            return if is_mecauto_native {
                Err(invalid_data(
                    "Este ZIP contém o backup nativo do MecAuto legado (InfoBck.ini + arquivo .FBK, Firebird 2.0). O conteúdo foi reconhecido, mas esta versão não converte esse formato; nenhum dado foi alterado.",
                ))
            } else {
                Err(invalid_data(
                    "ZIP sem mecauto-export.json ou mecauto.json. O formato deve ser exportado como JSON de intercâmbio.",
                ))
            };
        };

        let entry = archive.by_name(&entry_name)?;
        let mut bytes = Vec::with_capacity(entry.size() as usize);
        entry
            .take(MAX_ZIP_ENTRY_BYTES + 1)
            .read_to_end(&mut bytes)?;
        if bytes.len() as u64 > MAX_ZIP_ENTRY_BYTES {
            return Err(invalid_data(
                "O arquivo de dados dentro do ZIP excede o limite seguro.",
            ));
        }
        let value: serde_json::Value = serde_json::from_slice(&bytes)?;
        if entry_name.eq_ignore_ascii_case("mecauto-export.json") {
            let backup: PortableBackup = serde_json::from_value(value)?;
            if backup.format != PORTABLE_FORMAT || backup.version != PORTABLE_VERSION {
                return Err(invalid_data(format!(
                    "Formato ou versão do backup incompatível ({} v{}).",
                    backup.format, backup.version
                )));
            }
            Ok(backup.data)
        } else {
            let recognized = [
                "clientes",
                "veiculos",
                "produtos",
                "vendas",
                "ordens",
                "caixas",
                "movimentos",
                "movimentos_estoque",
                "profissionais",
                "fornecedors",
                "fornecedores",
                "usuarios",
                "eventos",
            ]
            .iter()
            .any(|key| value.get(key).is_some());
            if !recognized {
                return Err(invalid_data(
                    "mecauto.json não contém campos de cadastro reconhecidos.",
                ));
            }
            Ok(serde_json::from_value(value)?)
        }
    }
}

fn database_path() -> StorageResult<PathBuf> {
    if let Some(path) = std::env::var_os("MECAUTO_DB_PATH") {
        return Ok(PathBuf::from(path));
    }

    #[cfg(target_os = "windows")]
    let data_dir = std::env::var_os("APPDATA")
        .or_else(|| std::env::var_os("LOCALAPPDATA"))
        .map(PathBuf::from)
        .ok_or_else(|| invalid_data("As pastas de dados do Windows não foram encontradas."))?;

    #[cfg(target_os = "macos")]
    let data_dir = std::env::var_os("HOME")
        .map(PathBuf::from)
        .map(|home| home.join("Library").join("Application Support"))
        .ok_or_else(|| invalid_data("A pasta pessoal do usuário não foi encontrada."))?;

    #[cfg(all(unix, not(target_os = "macos")))]
    let data_dir = match std::env::var_os("XDG_DATA_HOME") {
        Some(path) => PathBuf::from(path),
        None => std::env::var_os("HOME")
            .map(PathBuf::from)
            .map(|home| home.join(".local").join("share"))
            .ok_or_else(|| invalid_data("A pasta pessoal do usuário não foi encontrada."))?,
    };

    Ok(data_dir.join("MecAuto").join("mecauto.sqlite3"))
}

fn save_collection<T, F>(
    transaction: &Transaction<'_>,
    entity: &str,
    records: &[T],
    id: F,
) -> StorageResult<()>
where
    T: Serialize,
    F: Fn(&T) -> String,
{
    let mut statement = transaction.prepare_cached(
        "INSERT INTO records (entity, record_id, position, payload) VALUES (?1, ?2, ?3, ?4)",
    )?;
    for (position, record) in records.iter().enumerate() {
        let payload = serde_json::to_string(record)?;
        statement.execute(params![entity, id(record), position as i64, payload])?;
    }
    Ok(())
}

fn save_setting<T: Serialize>(
    transaction: &Transaction<'_>,
    key: &str,
    value: &T,
) -> StorageResult<()> {
    transaction.execute(
        "INSERT INTO settings (key, value) VALUES (?1, ?2)",
        params![key, serde_json::to_string(value)?],
    )?;
    Ok(())
}

fn load_collection<T: DeserializeOwned>(
    connection: &Connection,
    entity: &str,
) -> rusqlite::Result<Vec<T>> {
    let mut statement = connection
        .prepare("SELECT payload FROM records WHERE entity = ?1 ORDER BY position, record_id")?;
    let rows = statement.query_map([entity], |row| {
        let payload: String = row.get(0)?;
        serde_json::from_str(&payload).map_err(|error| {
            rusqlite::Error::FromSqlConversionFailure(
                0,
                rusqlite::types::Type::Text,
                Box::new(error),
            )
        })
    })?;
    rows.collect()
}

fn load_setting<T: DeserializeOwned>(
    connection: &Connection,
    key: &str,
) -> rusqlite::Result<Option<T>> {
    let value: Option<String> = connection
        .query_row("SELECT value FROM settings WHERE key = ?1", [key], |row| {
            row.get(0)
        })
        .optional()?;
    value
        .map(|value| {
            serde_json::from_str(&value).map_err(|error| {
                rusqlite::Error::FromSqlConversionFailure(
                    0,
                    rusqlite::types::Type::Text,
                    Box::new(error),
                )
            })
        })
        .transpose()
}

fn load_data(connection: &Connection) -> rusqlite::Result<MecAutoData> {
    let mut data = MecAutoData {
        clientes: load_collection(connection, "clientes")?,
        veiculos: load_collection(connection, "veiculos")?,
        produtos: load_collection(connection, "produtos")?,
        vendas: load_collection(connection, "vendas")?,
        ordens: load_collection(connection, "ordens_servico")?,
        caixas: load_collection(connection, "caixas")?,
        movimentos: load_collection(connection, "movimentos_caixa")?,
        movimentos_estoque: load_collection(connection, "movimentos_estoque")?,
        profissionais: load_collection(connection, "profissionais")?,
        fornecedors: load_collection(connection, "fornecedores")?,
        usuarios: load_collection(connection, "usuarios")?,
        eventos: load_collection(connection, "eventos_financeiros")?,
        ..Default::default()
    };
    data.favoritos = load_setting(connection, "favoritos")?.unwrap_or_default();
    data.auto_backup_on = load_setting(connection, "auto_backup_on")?.unwrap_or_default();
    data.favorito_picker = load_setting(connection, "favorito_picker")?.unwrap_or_default();
    Ok(data)
}

fn invalid_data(message: impl Into<String>) -> Box<dyn Error + Send + Sync> {
    Box::new(std::io::Error::new(
        std::io::ErrorKind::InvalidData,
        message.into(),
    ))
}

fn is_demo_seed_product_id(id: &str) -> bool {
    matches!(id, "P-001" | "P-002" | "P-003" | "P-004" | "P-005")
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_directory() -> PathBuf {
        std::env::temp_dir().join(format!(
            "mecauto-storage-test-{}-{}",
            std::process::id(),
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ))
    }

    fn test_product(id: &str) -> Produto {
        Produto {
            id: id.into(),
            codigo: id.into(),
            descricao: "Produto de teste".into(),
            estoque: 5.0,
            ..Default::default()
        }
    }

    #[test]
    fn records_survive_reopen_and_backup_restore() {
        let directory = test_directory();
        fs::create_dir_all(&directory).unwrap();
        let database = directory.join("mecauto.sqlite3");
        let backup = directory.join("backup.zip");
        let modified_data;
        {
            let (storage, mut data) = Storage::open_at(database.clone()).unwrap();
            data.produtos.push(test_product("TEST-001"));
            storage.export_zip(&data, &backup).unwrap();
            assert_eq!(Storage::read_zip(&backup).unwrap(), data);
            modified_data = data;
        }
        {
            let (mut storage, _) = Storage::open_at(database.clone()).unwrap();
            let mut updated = modified_data.clone();
            updated.produtos[0].descricao = "Produto persistido".into();
            storage.save(&updated).unwrap();
        }
        {
            let (_, reopened) = Storage::open_at(database.clone()).unwrap();
            let mut expected = modified_data.clone();
            expected.produtos[0].descricao = "Produto persistido".into();
            assert_eq!(reopened, expected);
        }
        {
            let restored = Storage::read_zip(&backup).unwrap();
            let (mut storage, _) = Storage::open_at(database.clone()).unwrap();
            storage.save(&restored).unwrap();
        }
        {
            let (_, reopened) = Storage::open_at(database).unwrap();
            assert_eq!(reopened, modified_data);
        }
        fs::remove_dir_all(directory).unwrap();
    }

    #[test]
    fn stock_exits_survive_database_reopen_and_portable_backup() {
        let directory = test_directory();
        fs::create_dir_all(&directory).unwrap();
        let database = directory.join("mecauto.sqlite3");
        let backup = directory.join("backup.zip");
        let (mut storage, mut data) = Storage::open_at(database.clone()).unwrap();
        data.produtos.push(test_product("TEST-001"));

        data.registrar_saida_estoque(0, 1.5, "Uso em serviço".into(), "2026-11-23".into())
            .unwrap();
        storage.save(&data).unwrap();
        storage.export_zip(&data, &backup).unwrap();

        let (_, reopened) = Storage::open_at(database).unwrap();
        assert_eq!(reopened, data);
        assert_eq!(Storage::read_zip(&backup).unwrap(), data);
        fs::remove_dir_all(directory).unwrap();
    }

    #[test]
    fn removes_only_demo_products_once_from_existing_database() {
        let directory = test_directory();
        fs::create_dir_all(&directory).unwrap();
        let database = directory.join("mecauto.sqlite3");
        {
            let (mut storage, mut data) = Storage::open_at(database.clone()).unwrap();
            data.produtos = vec![test_product("P-001"), test_product("CUSTOM-001")];
            data.movimentos_estoque.push(MovEstoque {
                id: "ME-001".into(),
                produto_id: "P-001".into(),
                ..Default::default()
            });
            storage.save(&data).unwrap();
            storage
                .connection
                .execute(
                    "DELETE FROM settings WHERE key = ?1",
                    [DEMO_PRODUCTS_MIGRATION_KEY],
                )
                .unwrap();
        }

        let (mut storage, data) = Storage::open_at(database.clone()).unwrap();
        assert_eq!(
            data.produtos
                .iter()
                .map(|product| product.id.as_str())
                .collect::<Vec<_>>(),
            vec!["CUSTOM-001"]
        );
        assert!(data.movimentos_estoque.is_empty());
        storage
            .save(&MecAutoData {
                produtos: vec![test_product("P-001"), test_product("CUSTOM-001")],
                ..data.clone()
            })
            .unwrap();
        let (_, reopened) = Storage::open_at(database).unwrap();
        assert_eq!(reopened.produtos.len(), 2);
        fs::remove_dir_all(directory).unwrap();
    }

    #[test]
    fn identifies_native_firebird_backup_without_extracting_or_mutating_data() {
        let directory = test_directory();
        fs::create_dir_all(&directory).unwrap();
        let archive_path = directory.join("legacy.zip");
        let file = File::create(&archive_path).unwrap();
        let mut archive = ZipWriter::new(file);
        archive
            .start_file("InfoBck.ini", SimpleFileOptions::default())
            .unwrap();
        archive.write_all(b"[BACKUP]\nFIREBIRD=2.0\n").unwrap();
        archive
            .start_file("backup1.fbk", SimpleFileOptions::default())
            .unwrap();
        archive.write_all(b"Firebird backup").unwrap();
        archive.finish().unwrap();

        let error = Storage::read_zip(&archive_path).unwrap_err();
        assert!(error.to_string().contains("Firebird 2.0"));
        fs::remove_dir_all(directory).unwrap();
    }

    #[test]
    fn imports_json_exchange_with_common_supplier_key() {
        let directory = test_directory();
        fs::create_dir_all(&directory).unwrap();
        let archive_path = directory.join("interchange.zip");
        let file = File::create(&archive_path).unwrap();
        let mut archive = ZipWriter::new(file);
        archive
            .start_file("mecauto.json", SimpleFileOptions::default())
            .unwrap();
        archive
            .write_all(
                br#"{"fornecedores":[{"id":"F1","nome":"Fornecedor de teste"}],"produtos":[{"id":"P1","descricao":"Filtro"}]}"#,
            )
            .unwrap();
        archive.finish().unwrap();

        let imported = Storage::read_zip(&archive_path).unwrap();
        assert_eq!(imported.fornecedors.len(), 1);
        assert_eq!(imported.fornecedors[0].nome, "Fornecedor de teste");
        assert_eq!(imported.produtos[0].descricao, "Filtro");
        fs::remove_dir_all(directory).unwrap();
    }
}
