use resend_rs::types::SendEmail;
use resend_rs::{Client, Result as ResendResult};
use std::sync::Arc;

use crate::configuration::EmailSettings;

#[derive(Clone)]
pub struct EmailService {
    client: Option<Arc<Client>>,
    from_email: String,
    from_name: String,
    product_name: String,
}

impl EmailService {
    pub fn new(settings: &EmailSettings) -> Self {
        let client = if settings.api_key.trim().is_empty() {
            log::info!("Email: RESEND API key empty — welcome emails disabled");
            None
        } else {
            Some(Arc::new(Client::new(settings.api_key.trim())))
        };
        Self {
            client,
            from_email: settings.from_email.clone(),
            from_name: settings.from_name.clone(),
            product_name: settings.product_name.clone(),
        }
    }

    pub async fn send_waitlist_confirmation(&self, to_email: &str) -> ResendResult<()> {
        let Some(client) = &self.client else {
            return Ok(());
        };

        let from = format!("{} <{}>", self.from_name, self.from_email);
        let to = [to_email];
        let subject = format!("You're on the {} list", self.product_name);

        let html = format!(
            r#"<!DOCTYPE html>
<html>
<head><meta charset="UTF-8"><title>{product}</title></head>
<body style="font-family: system-ui, sans-serif; max-width: 560px; margin: 0 auto; padding: 24px; color: #111;">
  <h1 style="font-size: 1.5rem;">Thanks for signing up</h1>
  <p style="line-height: 1.6; color: #444;">
    You're on the early list for <strong>{product}</strong>. We'll email you when we open up.
  </p>
  <p style="line-height: 1.6; color: #666; font-size: 0.9rem;">
    If you didn't request this, you can ignore this message.
  </p>
</body>
</html>"#,
            product = self.product_name
        );

        let email = SendEmail::new(from, to, &subject).with_html(&html);
        client.emails.send(email).await?;
        Ok(())
    }
}
