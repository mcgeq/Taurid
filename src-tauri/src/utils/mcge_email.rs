// -----------------------------------------------------------------------------
//    Copyright (C) 2024 mcge. All rights reserved.
// Author:         mcge
// Email:          <mcgeq@outlook.com>
// File:           mcge_email.rs
// Description:    Custom email
// Create   Date:  2024-11-20 19:43:57
// Last Modified:  2024-12-21 23:09:36
// Modified   By:  mcge <mcgeq@outlook.com>
// -----------------------------------------------------------------------------

use std::path::Path;

use lettre::{
    message::{Attachment, Mailbox},
    transport::smtp::authentication::Credentials,
    Message, SmtpTransport, Transport,
};

use crate::mgerror::McgResult;

use super::{mcge_files::McgeUtils, mcge_toml::McgEmail};

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct MgMailer {
    credentials: Credentials,
    mailer: SmtpTransport,
    pub email_config: McgEmail,
}

#[allow(dead_code)]
impl MgMailer {
    pub fn new(email_config: McgEmail) -> McgResult<Self> {
        let credentials =
            Credentials::new(email_config.username.clone(), email_config.password.clone());
        let mailer = SmtpTransport::relay(&email_config.stmp_server)?
            .credentials(credentials.clone())
            .build();

        Ok(Self {
            email_config,
            credentials,
            mailer,
        })
    }

    // 邮件发送
    pub fn send_email_with_attachment(
        &self,
        from: &str,
        to: &str,
        subject: &str,
        body: &str,
        attachment_path: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let attachment = McgeUtils::read_file_to_string(Path::new(attachment_path))?;
        let file_ext = McgeUtils::get_file_name_extension(attachment_path)?;

        let email = Message::builder()
            .from(from.parse::<Mailbox>()?)
            .to(to.parse::<Mailbox>()?)
            .subject(subject)
            .multipart(
                lettre::message::MultiPart::mixed()
                    .singlepart(lettre::message::SinglePart::plain(body.to_string()))
                    .singlepart(Attachment::new(file_ext).body(attachment, "text/plain".parse()?)),
            )?;

        // 发送邮件
        self.mailer.send(&email)?;

        Ok(())
    }
}
