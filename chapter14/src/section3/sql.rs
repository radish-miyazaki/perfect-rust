use std::env;
use std::sync::Mutex;

use once_cell::sync::Lazy;
use yaml_rust::{Yaml, YamlLoader};
use anyhow::{Result, Error};

static SQLS: Lazy<Mutex<Yaml>> = Lazy::new(|| {
    init_sqls().unwrap_or_else(|err| panic!("{:?}", err))
});

fn init_sqls() -> Result<Mutex<Yaml>> {
    let current = env::current_dir().map(|path| path.join("resources/sql.yml"))?;
    let str_data = std::fs::read_to_string(current.as_os_str())?;
    let values: Vec<Yaml> = YamlLoader::load_from_str(&str_data)?;
    let result = Mutex::new(values[0].clone());
    Ok(result)
}

pub async fn get_sql(table_name: &str, method_name: &str) -> Result<String> {
    let yml = SQLS.lock().unwrap();
    let sqls = match yml[table_name].as_hash() {
        Some(sqls) => sqls,
        None => return Err(Error::msg(
            format!("Table name: {} not found in sql.yml", table_name)
        ))
    };

    let sql = match sqls.get(&Yaml::String(method_name.to_string())) {
        Some(sql) => sql.as_str().unwrap(),
        None => return Err(Error::msg(
            format!("Method name: {} not found in sql.yml", method_name)
        ))
    };

    Ok(sql.to_string())
}
