use uuid::Uuid;

/// Generate a new UUID
pub fn new_id() -> Uuid {
    Uuid::new_v4()
}

/// Generate a new ID with a prefix for readability
pub fn new_prefixed_id(prefix: &str) -> String {
    format!("{}_{}", prefix, Uuid::new_v4().simple())
}
