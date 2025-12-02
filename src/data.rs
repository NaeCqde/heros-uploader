use reqwest::header::{self, HeaderMap, HeaderValue};
use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub enum MyError {
    ResourceNotAccessible,
    RequestError(reqwest::Error),
}

impl From<reqwest::Error> for MyError {
    fn from(e: reqwest::Error) -> MyError {
        MyError::RequestError(e)
    }
}

impl std::fmt::Display for MyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MyError::ResourceNotAccessible => write!(f, "resource not accessible"),
            MyError::RequestError(e) => write!(f, "{}", e),
        }
    }
}

pub fn headers() -> HeaderMap {
    let mut header = HeaderMap::new();
    header.insert(header::USER_AGENT, HeaderValue::from_static("Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/142.0.0.0 Safari/537.36 Edg/142.0.0.0"));
    return header;
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Video {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumbnail: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub src: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Accounts {
    pub status: String,
    pub data: AccountsData,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AccountsData {
    pub id: String,
    pub root_folder: String,
    pub tier: String,
    pub token: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateFolderPayload {
    pub parent_folder_id: String,
    pub public: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateFolder {
    pub status: String,
    pub data: CreateFolderData,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateFolderData {
    pub id: String,
    pub owner: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub name: String,
    pub parent_folder: String,
    pub create_time: i64,
    pub mod_time: i64,
    pub code: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UploadFile {
    pub data: UploadFileData,
    pub status: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UploadFileData {
    pub create_time: i64,
    pub download_page: String,
    pub guest_token: String,
    pub id: String,
    pub md5: String,
    pub mimetype: String,
    pub mod_time: i64,
    pub name: String,
    pub parent_folder: String,
    pub parent_folder_code: String,
    pub servers: Vec<String>,
    pub size: i64,
    #[serde(rename = "type")]
    pub type_field: String,
}
