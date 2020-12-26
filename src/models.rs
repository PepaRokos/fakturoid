use chrono::{DateTime, Local, NaiveDate};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::Read;
use std::path::Path;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SubjectType {
    Customer,
    Supplier,
    Both,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Subject {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sub_type: Option<SubjectType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub street: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub street2: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zip: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registration_no: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vat_no: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub local_vat_no: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bank_account: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iban: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub variable_symbol: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled_reminders: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub full_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email_copy: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub web: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub private_note: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avatar_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub html_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<DateTime<Local>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<DateTime<Local>>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum InvoiceState {
    Open,
    Sent,
    Overdue,
    Paid,
    Cancelled,
}

impl ToString for InvoiceState {
    fn to_string(&self) -> String {
        match self {
            InvoiceState::Open => "open".to_string(),
            InvoiceState::Sent => "sent".to_string(),
            InvoiceState::Overdue => "overdue".to_string(),
            InvoiceState::Paid => "paid".to_string(),
            InvoiceState::Cancelled => "cancelled".to_string(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PaymentMethod {
    Bank,
    Cash,
    Cod,
    Paypal,
    Card,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum InvoiceLanguage {
    Cz,
    Sk,
    En,
    De,
    Fr,
    It,
    Es,
    Ru,
    Hu,
    Pl,
    Ro,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum VatPriceMode {
    WithoutVat,
    FromTotalWithVat,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum EetStatus {
    Waiting,
    Pkp,
    Fik,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EetRecord {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    pub vat_no: String,
    pub number: String,
    pub store: i32,
    pub cash_register: String,
    pub paid_at: DateTime<Local>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vat_base0: Option<Decimal>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vat_base1: Option<Decimal>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vat1: Option<Decimal>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vat_base2: Option<Decimal>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vat2: Option<Decimal>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vat_base3: Option<Decimal>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vat3: Option<Decimal>,
    pub total: Decimal,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fik: Option<String>,
    pub bkp: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pkp: Option<String>,
    pub status: EetStatus,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fik_received_at: Option<DateTime<Local>>,
    pub external: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attempts: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_attempt_at: Option<DateTime<Local>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_uuid: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub playground: Option<bool>,
    pub invoice_id: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<DateTime<Local>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<DateTime<Local>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RemoteAttachment {
    file_name: String,
    content_type: String,
    download_url: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
enum Attachment {
    Update(String),
    Received(RemoteAttachment),
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Invoice {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proforma: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub partial_proforma: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub variable_symbol: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub your_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub your_street: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub your_street2: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub your_city: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub your_zip: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub your_country: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub your_registration_no: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub your_vat_no: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub your_local_vat_no: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_street: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_street2: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_city: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_zip: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_country: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_registration_no: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_vat_no: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_local_vat_no: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subject_id: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subject_custom_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub generator_id: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub related_id: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub correction: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub correction_id: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<InvoiceState>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_number: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub issued_on: Option<NaiveDate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub taxable_fulfillment_due: Option<NaiveDate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub due: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub due_on: Option<NaiveDate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sent_at: Option<DateTime<Local>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paid_at: Option<DateTime<Local>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reminder_sent_at: Option<DateTime<Local>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accepted_at: Option<DateTime<Local>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cancelled_at: Option<DateTime<Local>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub note: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub footer_note: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub private_note: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bank_account_id: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bank_account: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iban: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub swift_bic: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_method: Option<PaymentMethod>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exchange_rate: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paypal: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gopay: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<InvoiceLanguage>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transferred_tax_liability: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supply_code: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eu_electronic_service: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vat_price_mode: Option<VatPriceMode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub round_total: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subtotal: Option<Decimal>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub native_subtotal: Option<Decimal>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total: Option<Decimal>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub native_total: Option<Decimal>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remaining_amount: Option<Decimal>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remaining_native_amount: Option<Decimal>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paid_amount: Option<Decimal>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eet: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eet_cash_register: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eet_store: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eet_records: Option<Vec<EetRecord>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    attachment: Option<Attachment>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub html_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub public_html_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pdf_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subject_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<DateTime<Local>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<DateTime<Local>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lines: Option<Vec<InvoiceLine>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InvoiceLine {
    pub id: Option<i32>,
    pub name: String,
    pub quantity: Decimal,
    pub unit_name: Option<String>,
    pub unit_price: Decimal,
    pub vat_rate: i32,
    pub unit_price_without_vat: Option<Decimal>,
    pub unit_price_with_vat: Option<Decimal>,
}

impl InvoiceLine {
    pub fn new(
        name: &str,
        quantity: Decimal,
        unit_name: Option<&str>,
        unit_price: Decimal,
        vat_rate: i32,
    ) -> Self {
        Self {
            id: None,
            name: name.to_string(),
            quantity,
            unit_name: unit_name.map(|n| n.to_string()),
            unit_price,
            vat_rate,
            unit_price_without_vat: None,
            unit_price_with_vat: None,
        }
    }
}

impl Invoice {
    pub fn set_attachment(&mut self, path: &Path) -> Result<(), ()> {
        if path.is_file() {
            let mut file = File::open(path).map_err(|_| ())?;
            let mut file_content: Vec<u8> = Vec::new();
            file.read_to_end(&mut file_content).map_err(|_| ())?;
            self.attachment = Some(Attachment::Update(format!(
                "data:{};base64,{}",
                tree_magic::from_u8(&file_content),
                base64::encode_config(file_content, base64::STANDARD_NO_PAD)
            )));
            return Ok(());
        }
        Err(())
    }

    pub fn attachment(&self) -> Option<&RemoteAttachment> {
        if let Some(attachment) = self.attachment.as_ref() {
            if let Attachment::Received(rcv) = attachment {
                Some(rcv)
            } else {
                None
            }
        } else {
            None
        }
    }
}

#[derive(Serialize)]
pub struct InvoicePayData {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paid_at: Option<DateTime<Local>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paid_amount: Option<Decimal>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub variable_symbol: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bank_account_id: Option<i32>
}

pub enum InvoiceAction {
    MarkAsSent,
    Deliver,
    Pay,
    PayProforma,
    PayPartialProforma,
    RemovePayment,
    DeliverReminder,
    Cancel,
    UndoCancel,
    Lock,
    Unlock
}

impl ToString for InvoiceAction {
    fn to_string(&self) -> String {
        match self {
            InvoiceAction::MarkAsSent => { "mark_as_sent" }
            InvoiceAction::Deliver => { "deliver" }
            InvoiceAction::Pay => { "pay" }
            InvoiceAction::PayProforma => { "pay_proforma" }
            InvoiceAction::PayPartialProforma => { "pay_partial_proforma" }
            InvoiceAction::RemovePayment => { "remove_payment" }
            InvoiceAction::DeliverReminder => { "deliver_reminder" }
            InvoiceAction::Cancel => { "cancel" }
            InvoiceAction::UndoCancel => { "undo_cancel" }
            InvoiceAction::Lock => { "lock" }
            InvoiceAction::Unlock => { "unlock" }
        }.to_string()
    }
}