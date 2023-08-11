// This file was generated by gir (https://github.com/gtk-rs/gir)
// from ..
// from ../gir-files
// DO NOT EDIT

mod account;
pub use self::account::Account;

mod calendar;
pub use self::calendar::Calendar;

mod chat;
pub use self::chat::Chat;

mod client;
pub use self::client::Client;

mod contacts;
pub use self::contacts::Contacts;

mod documents;
pub use self::documents::Documents;

#[cfg(feature = "v3_6")]
#[cfg_attr(docsrs, doc(cfg(feature = "v3_6")))]
mod exchange;
#[cfg(feature = "v3_6")]
#[cfg_attr(docsrs, doc(cfg(feature = "v3_6")))]
pub use self::exchange::Exchange;

#[cfg(feature = "v3_8")]
#[cfg_attr(docsrs, doc(cfg(feature = "v3_8")))]
mod files;
#[cfg(feature = "v3_8")]
#[cfg_attr(docsrs, doc(cfg(feature = "v3_8")))]
pub use self::files::Files;

mod mail;
pub use self::mail::Mail;

mod manager;
pub use self::manager::Manager;

#[cfg(feature = "v3_14")]
#[cfg_attr(docsrs, doc(cfg(feature = "v3_14")))]
mod maps;
#[cfg(feature = "v3_14")]
#[cfg_attr(docsrs, doc(cfg(feature = "v3_14")))]
pub use self::maps::Maps;

#[cfg(feature = "v3_14")]
#[cfg_attr(docsrs, doc(cfg(feature = "v3_14")))]
mod media_server;
#[cfg(feature = "v3_14")]
#[cfg_attr(docsrs, doc(cfg(feature = "v3_14")))]
pub use self::media_server::MediaServer;

#[cfg(feature = "v3_18")]
#[cfg_attr(docsrs, doc(cfg(feature = "v3_18")))]
mod music;
#[cfg(feature = "v3_18")]
#[cfg_attr(docsrs, doc(cfg(feature = "v3_18")))]
pub use self::music::Music;

mod oauth2_based;
pub use self::oauth2_based::OAuth2Based;

mod oauth_based;
pub use self::oauth_based::OAuthBased;

mod object;
pub use self::object::Object;

mod object_manager_client;
pub use self::object_manager_client::ObjectManagerClient;

#[cfg(feature = "v3_6")]
#[cfg_attr(docsrs, doc(cfg(feature = "v3_6")))]
mod password_based;
#[cfg(feature = "v3_6")]
#[cfg_attr(docsrs, doc(cfg(feature = "v3_6")))]
pub use self::password_based::PasswordBased;

#[cfg(feature = "v3_8")]
#[cfg_attr(docsrs, doc(cfg(feature = "v3_8")))]
mod photos;
#[cfg(feature = "v3_8")]
#[cfg_attr(docsrs, doc(cfg(feature = "v3_8")))]
pub use self::photos::Photos;

#[cfg(feature = "v3_12")]
#[cfg_attr(docsrs, doc(cfg(feature = "v3_12")))]
mod printers;
#[cfg(feature = "v3_12")]
#[cfg_attr(docsrs, doc(cfg(feature = "v3_12")))]
pub use self::printers::Printers;

#[cfg(feature = "v3_12")]
#[cfg_attr(docsrs, doc(cfg(feature = "v3_12")))]
mod read_later;
#[cfg(feature = "v3_12")]
#[cfg_attr(docsrs, doc(cfg(feature = "v3_12")))]
pub use self::read_later::ReadLater;

#[cfg(feature = "v3_6")]
#[cfg_attr(docsrs, doc(cfg(feature = "v3_6")))]
mod ticketing;
#[cfg(feature = "v3_6")]
#[cfg_attr(docsrs, doc(cfg(feature = "v3_6")))]
pub use self::ticketing::Ticketing;

#[cfg(feature = "v3_26")]
#[cfg_attr(docsrs, doc(cfg(feature = "v3_26")))]
mod todo;
#[cfg(feature = "v3_26")]
#[cfg_attr(docsrs, doc(cfg(feature = "v3_26")))]
pub use self::todo::Todo;

mod enums;
pub use self::enums::Error;

#[doc(hidden)]
pub mod traits {
    pub use super::account::AccountExt;
    pub use super::calendar::CalendarExt;
    pub use super::chat::ChatExt;
    pub use super::contacts::ContactsExt;
    pub use super::documents::DocumentsExt;
    #[cfg(feature = "v3_6")]
#[cfg_attr(docsrs, doc(cfg(feature = "v3_6")))]
    pub use super::exchange::ExchangeExt;
    #[cfg(feature = "v3_8")]
#[cfg_attr(docsrs, doc(cfg(feature = "v3_8")))]
    pub use super::files::FilesExt;
    pub use super::mail::MailExt;
    pub use super::manager::ManagerExt;
    #[cfg(feature = "v3_14")]
#[cfg_attr(docsrs, doc(cfg(feature = "v3_14")))]
    pub use super::maps::MapsExt;
    #[cfg(feature = "v3_14")]
#[cfg_attr(docsrs, doc(cfg(feature = "v3_14")))]
    pub use super::media_server::MediaServerExt;
    #[cfg(feature = "v3_18")]
#[cfg_attr(docsrs, doc(cfg(feature = "v3_18")))]
    pub use super::music::MusicExt;
    pub use super::oauth2_based::OAuth2BasedExt;
    pub use super::oauth_based::OAuthBasedExt;
    pub use super::object::GoaObjectExt;
    #[cfg(feature = "v3_6")]
#[cfg_attr(docsrs, doc(cfg(feature = "v3_6")))]
    pub use super::password_based::PasswordBasedExt;
    #[cfg(feature = "v3_8")]
#[cfg_attr(docsrs, doc(cfg(feature = "v3_8")))]
    pub use super::photos::PhotosExt;
    #[cfg(feature = "v3_12")]
#[cfg_attr(docsrs, doc(cfg(feature = "v3_12")))]
    pub use super::printers::PrintersExt;
    #[cfg(feature = "v3_12")]
#[cfg_attr(docsrs, doc(cfg(feature = "v3_12")))]
    pub use super::read_later::ReadLaterExt;
    #[cfg(feature = "v3_6")]
#[cfg_attr(docsrs, doc(cfg(feature = "v3_6")))]
    pub use super::ticketing::TicketingExt;
    #[cfg(feature = "v3_26")]
#[cfg_attr(docsrs, doc(cfg(feature = "v3_26")))]
    pub use super::todo::TodoExt;
}
