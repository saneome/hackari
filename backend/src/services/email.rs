use thiserror::Error;
use reqwest::Client;

#[derive(Error, Debug)]
pub enum EmailError {
    #[error("Failed to send email: {0}")]
    SendError(String),
    #[error("Invalid email address")]
    InvalidEmail,
}

pub struct EmailService {
    client: Client,
    api_key: String,
    from_email: String,
}

#[derive(serde::Serialize)]
struct EmailRequest {
    from: String,
    to: Vec<String>,
    subject: String,
    html: String,
    text: String,
}

impl EmailService {
    pub fn new(api_key: &str, from_email: &str) -> Self {
        let client = Client::new();
        Self {
            client,
            api_key: api_key.to_string(),
            from_email: from_email.to_string(),
        }
    }

    pub async fn send_reset_code(&self, to_email: &str, code: &str) -> Result<(), EmailError> {
        if !to_email.contains('@') {
            return Err(EmailError::InvalidEmail);
        }

        let subject = "Код сброса пароля - Hackari";
        let html = format!(
            r#"
            <div style="font-family: system-ui, -apple-system, sans-serif; max-width: 400px; margin: 0 auto; padding: 40px 20px; text-align: center; background: #0a0a0a; color: #ffffff; border-radius: 12px; border: 1px solid rgba(255,255,255,0.1);">
                <h2 style="font-size: 24px; margin-bottom: 8px; font-weight: 600;">Сброс пароля</h2>
                <p style="color: rgba(255,255,255,0.6); margin-bottom: 32px; font-size: 14px;">
                    Введите этот код для сброса пароля:
                </p>

                <div style="background: rgba(212, 255, 0, 0.1); border: 1px solid #D4FF00; border-radius: 8px; padding: 20px; margin-bottom: 32px;">
                    <span style="font-size: 32px; font-weight: 600; letter-spacing: 4px; color: #D4FF00; font-family: monospace;">{}</span>
                </div>

                <p style="color: rgba(255,255,255,0.4); font-size: 12px; margin-top: 24px;">
                    Код действителен 15 минут.<br>
                    Если вы не запрашивали сброс пароля, проигнорируйте это письмо.
                </p>
            </div>
            "#,
            code
        );

        let text = format!(
            "Код сброса пароля: {}\n\nКод действителен 15 минут.",
            code
        );

        let email_request = EmailRequest {
            from: self.from_email.clone(),
            to: vec![to_email.to_string()],
            subject: subject.to_string(),
            html,
            text,
        };

        let response = self.client
            .post("https://api.resend.com/emails")
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("Content-Type", "application/json")
            .json(&email_request)
            .send()
            .await
            .map_err(|e| EmailError::SendError(format!("Request failed: {}", e)))?;

        if response.status().is_success() {
            Ok(())
        } else {
            let error_text = response.text().await.unwrap_or_default();
            Err(EmailError::SendError(format!("Resend API error: {}", error_text)))
        }
    }
}
