use uuid::Uuid;

pub fn default_uid() -> String {
    Uuid::new_v4().to_string()
}