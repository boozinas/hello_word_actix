use crate::user::User;
use std::ops::Deref;
use std::sync::Arc;
use uuid::Uuid;

pub trait Repository: Send + Sync + 'static {
    fn get_user(&self, user_id: &uuid::Uuid) -> Result<User, String>;
}

pub struct MemoryRepository {
    users: Vec<User>,
}

impl Default for MemoryRepository {
    fn default() -> Self {
        Self {
            users: vec![User::new("Rob".to_string(), (1977, 03, 10))],
        }
    }
}

impl Repository for MemoryRepository {
    fn get_user(&self, user_id: &uuid::Uuid) -> Result<User, String> {
        self.users
            .iter()
            .find(|u| &u.id == user_id)
            // .map(|u| u.clone())
            .cloned()
            .ok_or_else(|| "Invalid UUID".to_string())
            .clone()
    }
}
// Struct de tipo tupla para poder almacenar y enviar cualquier objeto que implemente el trait
// Repository, para eso se utiliza dyn, por eso para acceder al método es necesario acceder con un
// cero y se encierra en un Box porque el tipo data tiene que saber exactamente de que tamaño será la
// Memoria almacenada en el stack, Box utiliza un puntero en el stack que apunta al heap
// esto se llama newType pattern
pub struct RepositoryInjector(Box<dyn Repository>);

// La implementacion del new se queja de que el trait no tiene tiempo de vida definido
impl RepositoryInjector {
    pub fn new(repo: impl Repository) -> Self {
        Self(Box::new(repo))
    }

    pub fn new_shared(repo: impl Repository) -> Arc<Self> {
        Arc::new(Self::new(repo))
    }
}

//Derrferenciar implementando el trait, con target colocamos a lo que vamos a derreferenciar
// as_ref utilizado para derrreferenciar a la instancia y poder acceder a sus métodos.

impl Deref for RepositoryInjector {
    type Target = dyn Repository;

    fn deref(&self) -> &Self::Target {
        self.0.as_ref()
    }
}
