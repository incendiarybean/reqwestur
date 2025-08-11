use crate::ui::widgets::notification::Notification;

#[derive(Default, serde::Deserialize, serde::Serialize, Clone, Eq, PartialEq)]
pub enum CertificateStatus {
    /// If the certificate has not been loaded
    #[default]
    UNCONFIRMED,

    /// If the certificate has loaded successfully
    OK,

    /// If loading the certificate caused an error
    ERROR,
}

/// A struct to contain all certificate related information
#[derive(Default, serde::Deserialize, serde::Serialize, Clone)]
#[serde(default)]
pub struct Certificate {
    /// Whether Certificates are required, used to suggest whether certificates are loaded or not.
    // pub required: bool,

    /// The file path to the selected certificate.
    pub file_path: std::path::PathBuf,

    /// The certificate passphrase.
    pub passphrase: String,

    /// The status of the loaded certificate.
    pub status: CertificateStatus,

    /// A notification style message to indicate information.
    pub notification: Option<Notification>,

    /// The identity loaded from the certificates.
    #[serde(skip)]
    pub identity: Option<reqwest::Identity>,
}

impl Certificate {
    /// Import the certificate based on the file path and passphrase
    pub fn import(&self) -> Result<reqwest::Identity, String> {
        match std::fs::read(&self.file_path) {
            Ok(file_bytes) => {
                match reqwest::Identity::from_pkcs12_der(&file_bytes, &self.passphrase) {
                    Ok(identity) => Ok(identity),
                    Err(error) => Err(error.to_string()),
                }
            }
            Err(error) => Err(error.to_string()),
        }
    }
}
