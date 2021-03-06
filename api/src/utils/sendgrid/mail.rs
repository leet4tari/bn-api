use errors::*;
use futures::future::Either;
use reqwest::async::Client as AsyncClient;
use reqwest::Client;
use serde_json;
use std::collections::HashMap;
use tokio::prelude::*;
use utils::communication::*;

const SENDGRID_API_URL: &'static str = "https://api.sendgrid.com/v3/mail/send";

pub fn send_email(
    sg_api_key: &str,
    source_email_address: String,
    dest_email_addresses: Vec<String>,
    title: String,
    body: Option<String>,
) -> Result<(), BigNeonError> {
    let mut sg_message = SGMailMessage::new();
    sg_message.subject = Some(title);
    sg_message.from = SGEmail::from(source_email_address);

    let mut msg_personalization = SGPersonalization::new();
    for email_address in dest_email_addresses {
        msg_personalization.to.push(SGEmail::from(email_address));
    }
    sg_message.personalizations.push(msg_personalization);

    let mut msg_content = SGContent::new();
    if body.is_some() {
        msg_content.value = body.clone().unwrap();
    }
    sg_message.content.push(msg_content);

    match sg_message.send(sg_api_key) {
        Ok(_body) => Ok(()),
        Err(err) => Err(ApplicationError::new(err.to_string()).into()),
    }
}

pub fn send_email_async(
    sg_api_key: &str,
    source_email_address: String,
    dest_email_addresses: Vec<String>,
    title: String,
    body: Option<String>,
) -> Box<Future<Item = (), Error = BigNeonError>> {
    let mut sg_message = SGMailMessage::new();
    sg_message.subject = Some(title);
    sg_message.from = SGEmail::from(source_email_address);

    let mut msg_personalization = SGPersonalization::new();
    for email_address in dest_email_addresses {
        msg_personalization.to.push(SGEmail::from(email_address));
    }
    sg_message.personalizations.push(msg_personalization);

    let mut msg_content = SGContent::new();
    if let Some(body) = body {
        msg_content.value = body;
    }
    sg_message.content.push(msg_content);

    Box::new(sg_message.send_async(sg_api_key))
}

pub fn send_email_template(
    sg_api_key: &str,
    source_email_address: String,
    dest_email_addresses: Vec<&str>,
    template_id: String,
    template_data: &[&TemplateData],
) -> Result<(), BigNeonError> {
    if dest_email_addresses.len() != template_data.len() {
        return Err(ApplicationError::new(
            "Destination addresses mismatched with template data".to_string(),
        )
        .into());
    }
    let mut sg_message = SGMailMessage::new();
    sg_message.from = SGEmail::from(source_email_address);
    sg_message.template_id = Some(template_id);

    for i in 0..dest_email_addresses.len() {
        let mut msg_personalization = SGPersonalization::new();
        msg_personalization
            .to
            .push(SGEmail::from(dest_email_addresses[i].to_string()));
        msg_personalization.add_template_data(template_data[i].clone());
        sg_message.personalizations.push(msg_personalization);
    }

    let msg_content = SGContent::new();
    sg_message.content.push(msg_content);

    match sg_message.send(&sg_api_key) {
        Ok(_body) => Ok(()),
        Err(err) => Err(ApplicationError::new(err.to_string()).into()),
    }
}

pub fn send_email_template_async(
    sg_api_key: &str,
    source_email_address: String,
    dest_email_addresses: &[String],
    template_id: String,
    template_data: &[TemplateData],
) -> Box<Future<Item = (), Error = BigNeonError>> {
    Box::new(if dest_email_addresses.len() != template_data.len() {
        Either::A(future::err(
            ApplicationError::new(
                "Destination addresses mismatched with template data".to_string(),
            )
            .into(),
        ))
    } else {
        let mut sg_message = SGMailMessage::new();
        sg_message.from = SGEmail::from(source_email_address);
        sg_message.template_id = Some(template_id);

        for i in 0..dest_email_addresses.len() {
            let mut msg_personalization = SGPersonalization::new();
            msg_personalization
                .to
                .push(SGEmail::from(dest_email_addresses[i].to_string()));
            msg_personalization.add_template_data(template_data[i].clone());
            sg_message.personalizations.push(msg_personalization);
        }

        let msg_content = SGContent::new();
        sg_message.content.push(msg_content);

        Either::B(sg_message.send_async(&sg_api_key))
    })
}

#[derive(Clone, Default, Serialize)]
pub struct SGEmail {
    pub email: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

impl SGEmail {
    pub fn new() -> SGEmail {
        SGEmail {
            email: String::new(),
            name: None,
        }
    }

    pub fn from(email: String) -> SGEmail {
        SGEmail { email, name: None }
    }
}

#[derive(Clone, Default, Serialize)]
pub struct SGContent {
    #[serde(rename = "type")]
    pub content_type: String,
    pub value: String,
}

impl SGContent {
    pub fn new() -> SGContent {
        SGContent {
            content_type: "text/html".to_string(),
            value: " ".to_string(), //sendgrid requires atleast 1 valid char
        }
    }

    pub fn from(content_type: &String, value: &String) -> SGContent {
        SGContent {
            content_type: content_type.clone(),
            value: value.clone(),
        }
    }
}

#[derive(Serialize)]
pub struct SGPersonalization {
    pub to: Vec<SGEmail>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subject: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dynamic_template_data: Option<TemplateData>,
}

impl SGPersonalization {
    pub fn new() -> SGPersonalization {
        SGPersonalization {
            to: Vec::new(),
            subject: None,
            dynamic_template_data: None,
        }
    }

    pub fn add_template_data(&mut self, template_data: TemplateData) {
        match self.dynamic_template_data {
            None => {
                let mut h = HashMap::new();
                for (name, value) in template_data {
                    h.insert(name, value);
                }
                self.dynamic_template_data = Some(h);
            }
            Some(ref mut h) => {
                h.extend(template_data);
            }
        }
    }
}

#[derive(Serialize)]
pub struct SGMailMessage {
    pub from: SGEmail,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subject: Option<String>,
    pub content: Vec<SGContent>,
    pub personalizations: Vec<SGPersonalization>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_id: Option<String>,
}

impl SGMailMessage {
    pub fn new() -> SGMailMessage {
        SGMailMessage {
            from: SGEmail::new(),
            subject: None,
            content: Vec::new(),
            personalizations: Vec::new(),
            template_id: None,
        }
    }

    fn send(&self, sq_api_key: &str) -> Result<(), BigNeonError> {
        let reqwest_client = Client::new();
        let msg_body = self.to_json();
        match reqwest_client
            .post(SENDGRID_API_URL)
            //.headers(reqwest_headers)
            .header("Authorization", format!("Bearer {}", sq_api_key))
            .header("Content-Type", "application/json")
            .header("user-agent", "sendgrid-rs")
            .body(msg_body)
            .send()
        {
            Ok(_res) => Ok(()),
            Err(err) => Err(ApplicationError::new(err.to_string()).into()),
        }
    }

    fn send_async(&self, sq_api_key: &str) -> impl Future<Item = (), Error = BigNeonError> {
        let reqwest_client = AsyncClient::new();
        let msg_body = self.to_json();
        reqwest_client
            .post(SENDGRID_API_URL)
            //.headers(reqwest_headers)
            .header("Authorization", format!("Bearer {}", sq_api_key))
            .header("Content-Type", "application/json")
            .header("user-agent", "sendgrid-rs")
            .body(msg_body)
            .send()
            .and_then(|r| future::result(r.error_for_status()))
            .map(|_| ())
            .map_err(|err| ApplicationError::new(err.to_string()).into())
    }

    fn to_json(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
}
