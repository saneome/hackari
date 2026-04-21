use thiserror::Error;
use lettre::{
    SmtpTransport, Transport, Message,
    message::{header::ContentType, Mailbox},
    transport::smtp::authentication::Credentials,
};

#[derive(Error, Debug)]
pub enum EmailError {
    #[error("Failed to send email: {0}")]
    SendError(String),
    #[error("Invalid email address")]
    InvalidEmail,
}

pub struct EmailService {
    from_email: String,
    smtp_transport: SmtpTransport,
    frontend_url: String,
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

        Ok(Self {
            from_email: from_email.to_string(),
            smtp_transport,
            frontend_url: frontend_url.to_string(),
        })
    }

    fn base_styles(&self) -> &'static str {
        r#"
<style>
@import url('https://fonts.googleapis.com/css2?family=Inter:wght@400;500;600;700&display=swap');
* { margin: 0; padding: 0; box-sizing: border-box; }
body { font-family: 'Inter', -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif; }
.container { max-width: 520px; margin: 0 auto; padding: 48px 24px; background: linear-gradient(180deg, #0a0a0a 0%, #111111 100%); border: 1px solid rgba(255,255,255,0.08); border-radius: 16px; }
.logo { text-align: center; margin-bottom: 40px; }
.logo-text { font-size: 28px; font-weight: 700; background: linear-gradient(135deg, #D4FF00 0%, #a8cc00 100%); -webkit-background-clip: text; -webkit-text-fill-color: transparent; background-clip: text; }
.title { font-size: 24px; font-weight: 600; color: #ffffff; text-align: center; margin-bottom: 16px; letter-spacing: -0.02em; }
.subtitle { font-size: 15px; color: rgba(255,255,255,0.55); text-align: center; margin-bottom: 32px; line-height: 1.6; }
.button-wrapper { text-align: center; margin: 32px 0; }
.button { display: inline-block; padding: 16px 32px; background: #D4FF00; color: #0a0a0a; font-size: 15px; font-weight: 600; text-decoration: none; border-radius: 8px; transition: all 0.2s; box-shadow: 0 4px 24px rgba(212, 255, 0, 0.25); }
.button:hover { background: #e0ff33; box-shadow: 0 6px 32px rgba(212, 255, 0, 0.35); }
.link-fallback { background: rgba(212, 255, 0, 0.06); border: 1px solid rgba(212, 255, 0, 0.2); border-radius: 8px; padding: 16px; margin: 16px 0; word-break: break-all; text-align: center; }
.link-fallback a { color: #D4FF00; font-size: 13px; text-decoration: none; font-family: 'SF Mono', monospace; }
.footer { margin-top: 40px; padding-top: 24px; border-top: 1px solid rgba(255,255,255,0.08); text-align: center; }
.footer-text { font-size: 12px; color: rgba(255,255,255,0.35); line-height: 1.6; }
.footer-text a { color: rgba(255,255,255,0.55); text-decoration: none; }
.highlight { color: #D4FF00; }
.info-box { background: rgba(255,255,255,0.03); border-radius: 10px; padding: 20px; margin: 20px 0; border: 1px solid rgba(255,255,255,0.06); }
.info-item { display: flex; align-items: center; gap: 12px; margin: 8px 0; color: rgba(255,255,255,0.7); font-size: 14px; }
.info-icon { width: 20px; height: 20px; background: rgba(212, 255, 0, 0.15); border-radius: 50%; display: flex; align-items: center; justify-content: center; color: #D4FF00; font-size: 11px; }
</style>
"#
    }

    fn wrap_email(&self, content: &str) -> String {
        format!(
            r#"<!DOCTYPE html>
<html lang="ru">
<head>
 <meta charset="UTF-8">
 <meta name="viewport" content="width=device-width, initial-scale=1.0">
 {}
</head>
<body style="background-color: #050505; padding: 24px 16px;">
 <div class="container">
 <div class="logo">
 <span class="logo-text">HACKARI</span>
 </div>
 {}
 </div>
</body>
</html>"#,
            self.base_styles(),
            content
        )
    }

    pub async fn send_password_reset_link(&self, to_email: &str, token: &str) -> Result<(), EmailError> {
        if !to_email.contains('@') {
            return Err(EmailError::InvalidEmail);
        }

        let reset_url = format!("{}/auth/reset-password?token={}", self.frontend_url, token);

        let subject = "Сброс пароля — Hackari";
        let html_content = format!(
            r#"<h1 class="title">Сброс пароля</h1>
<p class="subtitle">Мы получили запрос на сброс пароля для вашего аккаунта. Нажмите кнопку ниже, чтобы создать новый пароль.</p>
<div class="button-wrapper"><a href="{}" class="button">Сбросить пароль</a></div>
<div class="link-fallback"><a href="{}">{}</a></div>
<div class="info-box">
<div class="info-item"><div class="info-icon">&#8986;</div><span>Ссылка действительна <span class="highlight">15 минут</span></span></div>
<div class="info-item"><div class="info-icon">&#128274;</div><span>Если вы не запрашивали сброс, просто проигнорируйте это письмо</span></div>
</div>
<div class="footer"><p class="footer-text">Вы получили это письмо, так как указали этот email при регистрации на Hackari.<br><a href="{}">hackari.ru</a></p></div>"#,
            reset_url, reset_url, reset_url, self.frontend_url
        );

        let html = self.wrap_email(&html_content);

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

        let subject = "Подтверждение email — Hackari";
        let html_content = format!(
            r#"<h1 class="title">Добро пожаловать!</h1>
<p class="subtitle">Спасибо за регистрацию на Hackari. Для активации аккаунта подтвердите ваш email, нажав кнопку ниже.</p>
<div class="button-wrapper"><a href="{}" class="button">Подтвердить email</a></div>
<div class="link-fallback"><a href="{}">{}</a></div>
<div class="info-box">
<div class="info-item"><div class="info-icon">&#8986;</div><span>Ссылка действительна <span class="highlight">24 часа</span></span></div>
<div class="info-item"><div class="info-icon">&#128274;</div><span>Без подтверждения вы не сможете войти в аккаунт</span></div>
</div>
<div class="footer"><p class="footer-text">Вы получили это письмо, так как указали этот email при регистрации на Hackari.<br><a href="{}">hackari.ru</a></p></div>"#,
            verify_url, verify_url, verify_url, self.frontend_url
        );

        let html = self.wrap_email(&html_content);

        let text = format!(
            "Подтверждение email — Hackari\n\nСпасибо за регистрацию! Перейдите по ссылке для подтверждения email:\n{}\n\nСсылка действительна 24 часа.\nБез подтверждения вы не сможете войти в аккаунт.",
            verify_url
        );

        self.send_email(to_email, subject, &html, &text).await
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
