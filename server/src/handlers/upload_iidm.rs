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

// Modifiez la fonction process_upload pour optimiser la gestion des gros fichiers
async fn process_upload(multipart: &mut Multipart) -> Result<Network, UploadError> {
    while let Some(mut field) = multipart
        .next_field()
        .await
        .map_err(UploadError::MultipartError)?
    {
        if field.name() == Some("iidm_file") {
            // Augmenter la taille du buffer pour les fichiers volumineux
            const CHUNK_SIZE: usize = 8 * 1024 * 1024; // 8MB par morceau pour plus d'efficacité

            // Créer un fichier temporaire nommé
            let named_temp_file = NamedTempFile::new()?;
            let temp_path = named_temp_file.path().to_owned();
            let file_path = temp_path.to_string_lossy().to_string();

            // Créer le fichier avec tokio
            let mut file = tokio::fs::File::create(&file_path).await?;

            // Lire et écrire par morceaux
            let mut bytes_read = 0;

            // Pour les fichiers très volumineux, utilisez une approche efficace
            while let Some(chunk) = field.chunk().await.map_err(UploadError::MultipartError)? {
                file.write_all(&chunk).await?;
                bytes_read += chunk.len();

                // Évitez de flush après chaque morceau pour plus d'efficacité
                // sauf si c'est un petit morceau (qui pourrait être le dernier)
                if chunk.len() < CHUNK_SIZE {
                    file.flush().await?;
                }
            }

            // Finaliser l'écriture
            file.flush().await?;
            file.sync_all().await?;
            drop(file);

            if bytes_read == 0 {
                return Err(UploadError::NoFile);
            }

            // Pour les fichiers volumineux, utilisez une approche de parsing plus efficace
            // Par exemple, utiliser un Reader au lieu de charger tout le contenu en mémoire
            let std_file = std::fs::File::open(&file_path)?;
            let reader = BufReader::with_capacity(CHUNK_SIZE, std_file); // Augmenter la capacité du buffer

            // Utiliser quick_xml avec configuration optimisée
            let network = match quick_xml::de::from_reader(reader) {
                Ok(network) => network,
                Err(e) => {
                    eprintln!("Erreur lors du parsing XML: {}", e);

                    // Approche alternative si nécessaire
                    // Pour les fichiers vraiment volumineux, il peut être plus efficace d'utiliser
                    // un parser événementiel plutôt que de_from_str qui chargerait tout en mémoire
                    let fallback_reader = std::fs::File::open(&file_path)?;
                    let fallback_buf_reader = BufReader::with_capacity(CHUNK_SIZE, fallback_reader);
                    quick_xml::de::from_reader(fallback_buf_reader)
                        .map_err(UploadError::XmlError)?
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
