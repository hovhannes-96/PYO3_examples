// use arangors::AqlQuery;
// use arangors::client::ClientExt;
use arangors::connection::Connection;
use serde::{Deserialize, Serialize};
use pyo3::prelude::*;
use pyo3_asyncio::tokio::future_into_py;

#[derive(Deserialize, Serialize)]
#[pyclass]
pub struct Contract {
    pub relative_filepath: String,
    #[pyo3(get, set)]
    pub name: String,
    pub _id: String,
    pub derived: Vec<serde_json::Value>,
    pub bases: Vec<serde_json::Value>,
    pub direct_bases: Vec<serde_json::Value>,
    pub first_source_line: i32,
    pub last_source_line: i32,
    pub start_column: i32,
    pub end_column: i32,
    pub chain_id: i32,
    pub address: String,
    pub is_main: bool,
    pub is_library: bool,
}


#[pyfunction]
fn get_documents(py: Python<'_>) -> PyResult<&PyAny> {
    future_into_py(py, async {
        let conn = Connection::establish_basic_auth("http://192.168.205.100:8529/", "root", "root").await.unwrap();
        let db = conn.db("hexens_project").await.unwrap();
        let result: Vec<Contract> = db.aql_str(r#"FOR c in contracts LIMIT 50001 RETURN c"#).await.unwrap();
        Ok::<_, PyErr>(result)
    })
}


#[pymodule]
fn my_rust_lib(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<Contract>()?;
    m.add_function(wrap_pyfunction!(get_documents, m)?)?;
    Ok(())
}
