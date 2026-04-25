use thiserror::Error;
use lettre::{
    SmtpTransport, Transport, Message,
    message::{header::ContentType, Mailbox},
    transport::smtp::authentication::Credentials,
};
use handlebars::Handlebars;
use std::collections::HashMap;
use std::fs;
use std::path::Path;

#[derive(Error, Debug)]
pub enum EmailError {
    #[error("Failed to send email: {0}")]
    SendError(String),
    #[error("Invalid email address")]
    InvalidEmail,
    #[error("Template error: {0}")]
    TemplateError(String),
}

pub struct EmailService {
    from_email: String,
    smtp_transport: SmtpTransport,
    frontend_url: String,
    templates: Handlebars<'static>,
}

impl EmailService {
    pub fn new(smtp_user: &str, smtp_password: &str, from_email: &str, frontend_url: &str) -> Result<Self, EmailError> {
        let creds = Credentials::new(smtp_user.to_string(), smtp_password.to_string());

        // Yandex SMTP configuration
        let smtp_transport = SmtpTransport::relay("smtp.yandex.ru")
            .map_err(|e| EmailError::SendError(format!("SMTP relay error: {}", e)))?
            .credentials(creds)
            .port(465)
            .build();

        let templates = Self::load_templates()?;

        Ok(Self {
            from_email: from_email.to_string(),
            smtp_transport,
            frontend_url: frontend_url.to_string(),
            templates,
        })
    }

    fn load_templates() -> Result<Handlebars<'static>, EmailError> {
        let mut handlebars = Handlebars::new();
        handlebars.set_strict_mode(false);

        let templates_dir = Path::new("templates/email");

        // Load base template
        let base_template = fs::read_to_string(templates_dir.join("base.html"))
            .map_err(|e| EmailError::TemplateError(format!("Failed to load base template: {}", e)))?;
        handlebars.register_template_string("base", base_template)
            .map_err(|e| EmailError::TemplateError(format!("Failed to register base template: {}", e)))?;

        // Load verification template
        let verification_template = fs::read_to_string(templates_dir.join("verification.html"))
            .map_err(|e| EmailError::TemplateError(format!("Failed to load verification template: {}", e)))?;
        handlebars.register_template_string("verification", verification_template)
            .map_err(|e| EmailError::TemplateError(format!("Failed to register verification template: {}", e)))?;

        // Load password reset template
        let password_reset_template = fs::read_to_string(templates_dir.join("password_reset.html"))
            .map_err(|e| EmailError::TemplateError(format!("Failed to load password reset template: {}", e)))?;
        handlebars.register_template_string("password_reset", password_reset_template)
            .map_err(|e| EmailError::TemplateError(format!("Failed to register password reset template: {}", e)))?;

        // Load moderation templates
        let hackathon_approved_template = fs::read_to_string(templates_dir.join("hackathon_approved.html"))
            .map_err(|e| EmailError::TemplateError(format!("Failed to load hackathon_approved template: {}", e)))?;
        handlebars.register_template_string("hackathon_approved", hackathon_approved_template)
            .map_err(|e| EmailError::TemplateError(format!("Failed to register hackathon_approved template: {}", e)))?;

        let hackathon_rejected_template = fs::read_to_string(templates_dir.join("hackathon_rejected.html"))
            .map_err(|e| EmailError::TemplateError(format!("Failed to load hackathon_rejected template: {}", e)))?;
        handlebars.register_template_string("hackathon_rejected", hackathon_rejected_template)
            .map_err(|e| EmailError::TemplateError(format!("Failed to register hackathon_rejected template: {}", e)))?;

        let organizer_verified_template = fs::read_to_string(templates_dir.join("organizer_verified.html"))
            .map_err(|e| EmailError::TemplateError(format!("Failed to load organizer_verified template: {}", e)))?;
        handlebars.register_template_string("organizer_verified", organizer_verified_template)
            .map_err(|e| EmailError::TemplateError(format!("Failed to register organizer_verified template: {}", e)))?;

        let report_resolved_template = fs::read_to_string(templates_dir.join("report_resolved.html"))
            .map_err(|e| EmailError::TemplateError(format!("Failed to load report_resolved template: {}", e)))?;
        handlebars.register_template_string("report_resolved", report_resolved_template)
            .map_err(|e| EmailError::TemplateError(format!("Failed to register report_resolved template: {}", e)))?;

        Ok(handlebars)
    }

    fn render_email(&self, template_name: &str, data: &HashMap<&str, String>) -> Result<String, EmailError> {
        // Render the content template
        let content = self.templates.render(template_name, data)
            .map_err(|e| EmailError::TemplateError(format!("Failed to render {} template: {}", template_name, e)))?;

        // Prepare data for base template
        let mut base_data: HashMap<&str, String> = HashMap::new();
        base_data.insert("content", content);

        // Render the base template with content
        self.templates.render("base", &base_data)
            .map_err(|e| EmailError::TemplateError(format!("Failed to render base template: {}", e)))
    }

    pub async fn send_password_reset_link(&self, to_email: &str, token: &str) -> Result<(), EmailError> {
        if !to_email.contains('@') {
            return Err(EmailError::InvalidEmail);
        }

        let reset_url = format!("{}/auth/reset-password?token={}", self.frontend_url, token);

        let mut data: HashMap<&str, String> = HashMap::new();
        data.insert("reset_url", reset_url.clone());
        data.insert("frontend_url", self.frontend_url.clone());

        let subject = "Сброс пароля — Hackari";
        let html = self.render_email("password_reset", &data)?;

        let text = format!(
            "Сброс пароля — Hackari\n\nПерейдите по ссылке для сброса пароля:\n{}\n\nСсылка действительна 15 минут.\nЕсли вы не запрашивали сброс, проигнорируйте это письмо.",
            reset_url
        );

        self.send_email(to_email, subject, &html, &text).await
    }

    pub async fn send_email_verification(&self, to_email: &str, token: &str) -> Result<(), EmailError> {
        if !to_email.contains('@') {
            return Err(EmailError::InvalidEmail);
        }

        let verify_url = format!("{}/auth/verify-email?token={}", self.frontend_url, token);

        let mut data: HashMap<&str, String> = HashMap::new();
        data.insert("verify_url", verify_url.clone());
        data.insert("frontend_url", self.frontend_url.clone());

        let subject = "Подтверждение email — Hackari";
        let html = self.render_email("verification", &data)?;

        let text = format!(
            "Подтверждение email — Hackari\n\nСпасибо за регистрацию! Перейдите по ссылке для подтверждения email:\n{}\n\nСсылка действительна 24 часа.\nБез подтверждения вы не сможете войти в аккаунт.",
            verify_url
        );

        self.send_email(to_email, subject, &html, &text).await
    }

    pub async fn send_hackathon_approved(&self, to_email: &str, hackathon_title: &str, hackathon_id: &str) -> Result<(), EmailError> {
        if !to_email.contains('@') {
            return Err(EmailError::InvalidEmail);
        }

        let hackathon_url = format!("{}/hackathons/{}", self.frontend_url, hackathon_id);

        let mut data: HashMap<&str, String> = HashMap::new();
        data.insert("hackathon_title", hackathon_title.to_string());
        data.insert("hackathon_url", hackathon_url.clone());
        data.insert("frontend_url", self.frontend_url.clone());

        let subject = format!("Ваш хакатон «{}» одобрен — Hackari", hackathon_title);
        let html = self.render_email("hackathon_approved", &data)?;

        let text = format!(
            "Ваш хакатон «{}» одобрен — Hackari\n\nВаш хакатон был успешно проверен и опубликован на платформе.\n\nПосмотреть: {}\n\nТеперь участники могут регистрироваться на ваш хакатон.",
            hackathon_title, hackathon_url
        );

        self.send_email(to_email, &subject, &html, &text).await
    }

    pub async fn send_hackathon_rejected(&self, to_email: &str, hackathon_title: &str, reason: Option<&str>) -> Result<(), EmailError> {
        if !to_email.contains('@') {
            return Err(EmailError::InvalidEmail);
        }

        let dashboard_url = format!("{}/organizers/dashboard", self.frontend_url);

        let reason_text = reason.unwrap_or("Причина не указана");

        let mut data: HashMap<&str, String> = HashMap::new();
        data.insert("hackathon_title", hackathon_title.to_string());
        data.insert("reason", reason_text.to_string());
        data.insert("dashboard_url", dashboard_url.clone());
        data.insert("frontend_url", self.frontend_url.clone());

        let subject = format!("Ваш хакатон «{}» требует доработки — Hackari", hackathon_title);
        let html = self.render_email("hackathon_rejected", &data)?;

        let text = format!(
            "Ваш хакатон «{}» требует доработки — Hackari\n\nК сожалению, ваш хакатон не прошел модерацию.\n\nПричина: {}\n\nПерейдите в дашборд для редактирования: {}\n\nПосле внесения изменений хакатон будет повторно рассмотрен.",
            hackathon_title, reason_text, dashboard_url
        );

        self.send_email(to_email, &subject, &html, &text).await
    }

    pub async fn send_organizer_verified(&self, to_email: &str, organizer_name: &str) -> Result<(), EmailError> {
        if !to_email.contains('@') {
            return Err(EmailError::InvalidEmail);
        }

        let create_url = format!("{}/hackathons/create", self.frontend_url);

        let mut data: HashMap<&str, String> = HashMap::new();
        data.insert("organizer_name", organizer_name.to_string());
        data.insert("create_url", create_url.clone());
        data.insert("frontend_url", self.frontend_url.clone());

        let subject = "Поздравляем! Ваш профиль верифицирован — Hackari".to_string();
        let html = self.render_email("organizer_verified", &data)?;

        let text = format!(
            "Поздравляем! Ваш профиль верифицирован — Hackari\n\nВаша организация «{}» прошла верификацию на платформе Hackari.\n\nТеперь вы можете создавать хакатоны и получать повышенный уровень доверия от участников.\n\nСоздать хакатон: {}\n\nСпасибо, что работаете с нами!",
            organizer_name, create_url
        );

        self.send_email(to_email, &subject, &html, &text).await
    }

    pub async fn send_report_resolved(&self, to_email: &str, report_id: &str) -> Result<(), EmailError> {
        if !to_email.contains('@') {
            return Err(EmailError::InvalidEmail);
        }

        let profile_url = format!("{}/profile", self.frontend_url);

        let mut data: HashMap<&str, String> = HashMap::new();
        data.insert("report_id", report_id.to_string());
        data.insert("profile_url", profile_url.clone());
        data.insert("frontend_url", self.frontend_url.clone());

        let subject = "Ваша жалоба рассмотрена — Hackari".to_string();
        let html = self.render_email("report_resolved", &data)?;

        let text = format!(
            "Ваша жалоба рассмотрена — Hackari\n\nСпасибо за содействие в поддержании качества контента Hackari.\n\nИдентификатор жалобы: {}\n\nНаша команда модераторов рассмотрела вашу жалобу и приняла соответствующие меры.\n\nПросмотреть статус: {}",
            report_id, profile_url
        );

        self.send_email(to_email, &subject, &html, &text).await
    }

    async fn send_email(&self, to_email: &str, subject: &str, html: &str, text: &str) -> Result<(), EmailError> {
        let from: Mailbox = self.from_email.parse()
            .map_err(|e| EmailError::SendError(format!("Invalid from address: {}", e)))?;
        let to: Mailbox = to_email.parse()
            .map_err(|e| EmailError::SendError(format!("Invalid to address: {}", e)))?;

        let email = Message::builder()
            .from(from)
            .to(to)
            .subject(subject)
            .header(ContentType::TEXT_HTML)
            .body(html.to_string())
            .map_err(|e| EmailError::SendError(format!("Failed to build email: {}", e)))?;

        self.smtp_transport.send(&email)
            .map_err(|e| EmailError::SendError(format!("SMTP send failed: {}", e)))?;

        Ok(())
    }
}
