use anyhow::{Result, anyhow};
use keyring_core::Entry;

const SERVICE: &str = "talgonite";

pub fn set_password(cred_id: &str, password: &str) -> Result<()> {
    Entry::new(SERVICE, cred_id)
        .map_err(|e| anyhow!("keyring error: {}", e))?
        .set_password(password)
        .map_err(|e| anyhow!("failed to store password: {}", e))
}

pub fn get_password(cred_id: &str) -> Result<String> {
    Entry::new(SERVICE, cred_id)
        .map_err(|e| anyhow!("keyring error: {}", e))?
        .get_password()
        .map_err(|e| anyhow!("password not found: {}", e))
}

pub fn delete_password(cred_id: &str) -> Result<()> {
    let entry = Entry::new(SERVICE, cred_id).map_err(|e| anyhow!("keyring error: {}", e))?;
    match entry.delete_credential() {
        Ok(()) => Ok(()),
        Err(keyring_core::Error::NoEntry) => Ok(()),
        Err(e) => Err(anyhow!("failed to delete password: {}", e)),
    }
}
