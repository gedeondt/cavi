pub use crate::error::KvResult;

/// Interfaz genérica para un almacén clave-valor
pub trait KvStore {
    /// Obtiene el valor asociado a una clave, o `None` si no existe.
    fn get(&self, key: &str) -> KvResult<Option<String>>;

    /// Establece el valor de una clave (crea o reemplaza).
    fn set(&mut self, key: String, value: String) -> KvResult<()>;

    /// Elimina una clave. No falla si no existe.
    fn delete(&mut self, key: &str) -> KvResult<()>;

    /// Devuelve todas las claves y valores cuyo prefijo coincida.
    fn search_by_prefix(&self, prefix: &str) -> KvResult<Vec<(String, String)>>;
}