use crate::states::AppState;
use askama::Template;
use axum::{
    extract::{Multipart, State},
    http::StatusCode,
    response::{Html, IntoResponse, Response},
};
use iidm::{xml::Network, Identifiable};
use std::{io::BufReader, sync::Arc};
use tempfile::NamedTempFile;
use thiserror::Error;
use tokio::io::AsyncWriteExt;

#[derive(Error, Debug)]
pub enum UploadError {
    #[error("Multipart field error: {0}")]
    MultipartError(#[from] axum::extract::multipart::MultipartError),
    #[error("Template rendering error: {0}")]
    TemplateError(#[from] askama::Error),
    #[error("No IIDM file provided")]
    NoFile,
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
    #[error("XML parsing error: {0}")]
    XmlError(#[from] quick_xml::DeError),
}

// Implement IntoResponse for our error type
impl IntoResponse for UploadError {
    fn into_response(self) -> Response {
        let message = self.to_string();
        (StatusCode::BAD_REQUEST, message).into_response()
    }
}

#[derive(Template)]
#[template(path = "iidm_table.html")]
struct IIdmTableTemplate {
    message: String,
    network: Option<Network>,
}

impl IIdmTableTemplate {
    fn new(message: String, network: Option<Network>) -> Self {
        Self { message, network }
    }
}

pub async fn upload_iidm(
    State(state): State<Arc<AppState>>,
    mut multipart: Multipart,
) -> Result<impl IntoResponse, UploadError> {
    let result = process_upload(&mut multipart).await;
    match result {
        Ok(network) => {
            update_ecs_state(&state, &network).await;
            let template = IIdmTableTemplate::new("".to_string(), Some(network));
            let html = template.render().map_err(UploadError::TemplateError)?;
            Ok(Html(html))
        }
        Err(err) => {
            let error_message = err.to_string();
            let template = IIdmTableTemplate::new(error_message, None);
            let html = template.render().map_err(UploadError::TemplateError)?;
            Ok(Html(html))
        }
    }
}

async fn process_upload(multipart: &mut Multipart) -> Result<Network, UploadError> {
    while let Some(mut field) = multipart
        .next_field()
        .await
        .map_err(UploadError::MultipartError)?
    {
        if field.name() == Some("iidm_file") {
            // Créer un fichier temporaire nommé
            let named_temp_file = NamedTempFile::new()?;
            let temp_path = named_temp_file.path().to_owned();

            // Convertir en fichier Tokio
            let file_path = temp_path.to_string_lossy().to_string();
            let mut file = tokio::fs::File::create(&file_path).await?;

            // Lire le contenu du champ et l'écrire dans le fichier
            // Au lieu de stream, on utilise bytes() qui est une méthode existante
            // mais on traite par morceaux pour éviter de charger tout en mémoire
            const CHUNK_SIZE: usize = 1024 * 1024; // 1MB par morceau
            let mut bytes_read = 0;
            loop {
                let chunk_data = field.chunk().await.map_err(UploadError::MultipartError)?;

                if let Some(data) = chunk_data {
                    file.write_all(&data).await?;
                    bytes_read += data.len();

                    // On continue à lire le prochain morceau
                    if data.len() < CHUNK_SIZE {
                        break; // C'était probablement le dernier morceau
                    }
                } else {
                    break; // Plus de données à lire
                }
            }

            // S'assurer que toutes les données sont écrites et fermer le fichier
            file.flush().await?;
            file.sync_all().await?;
            drop(file); // Fermer explicitement le fichier

            // Si aucun octet n'a été lu, on a un problème
            if bytes_read == 0 {
                return Err(UploadError::NoFile);
            }

            // Ouvrir avec std::fs pour le parsing
            let std_file = std::fs::File::open(&file_path)?;
            let reader = BufReader::new(std_file);

            // Utiliser quick_xml standard
            let network = match quick_xml::de::from_reader(reader) {
                Ok(network) => network,
                Err(_e) => {
                    // En cas d'erreur, on peut essayer une approche alternative
                    // Par exemple, lire le fichier en tant que chaîne de caractères
                    let xml_content = std::fs::read_to_string(&file_path)?;
                    quick_xml::de::from_str(&xml_content).map_err(UploadError::XmlError)?
                }
            };

            // Nettoyer le fichier temporaire
            if let Err(e) = std::fs::remove_file(&file_path) {
                eprintln!("Erreur lors de la suppression du fichier temporaire: {}", e);
            }

            return Ok(network);
        }
    }

    Err(UploadError::NoFile)
}

async fn update_ecs_state(state: &Arc<AppState>, network: &Network) {
    let ecs = state.ecs.read().await;
    let mut world = ecs.world.write().await;
    let mut schedule = ecs.schedule.write().await;
    network.register(&mut world, &mut schedule);
}
