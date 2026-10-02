use chrono::{DateTime, Utc};

pub enum TypeAction {
    Ingress,
    Exit
}

pub struct Alert{
    ip_origen: String,
    ip_destination: String,
    remote_user: String,
    type_action: TypeAction,
    time_action: DateTime<Utc>,
}

impl Alert {
    pub fn new(ip_origen: &str, ip_destination: &str, remote_user: &str, type_action: TypeAction) -> Self{
        Self {
            ip_origen: ip_origen.to_string(),
            ip_destination: ip_destination.to_string(),
            remote_user: remote_user.to_string(),
            type_action,
            time_action: Utc::now(),
        }
    }
}