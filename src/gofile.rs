use crate::data::{headers, Accounts, CreateFolder, CreateFolderPayload, MyError, UploadFile};
use mime::Mime;
use mime_guess::from_path;
use reqwest::{
    multipart::{Form, Part},
    Client,
};
use std::str::FromStr;

pub async fn gofile(url: &str) -> Result<String, MyError> {
    let client = Client::builder()
        .default_headers(headers())
        .build()
        .unwrap();

    let resp = client.get(url).send().await?;
    if let Some(length) = resp.content_length() {
        if length > 1 {
            log::debug!(target: "gofile", "file was fetched");

            let mut file_name: String = "video.mp4".to_owned();
            if let Some(last) = url.split("/").last() {
                if last != "" {
                    file_name = last.split("?").next().unwrap().to_owned()
                }
            }

            let acc: Accounts = client
                .post("https://api.gofile.io/accounts")
                .send()
                .await?
                .json()
                .await?;

            let payload = CreateFolderPayload {
                parent_folder_id: acc.data.root_folder,
                public: true,
            };

            let folder: CreateFolder = client
                .post("https://api.gofile.io/contents/createfolder")
                .bearer_auth(&acc.data.token)
                .json(&payload)
                .send()
                .await?
                .json()
                .await?;

            let form = Form::new()
                .text("token", acc.data.token)
                .text("folderId", folder.data.id)
                .part(
                    "file",
                    Part::stream(resp.bytes().await?)
                        .file_name(file_name.to_owned())
                        .mime_str(
                            from_path(file_name)
                                .first()
                                .unwrap_or(Mime::from_str("video/mp4").unwrap())
                                .essence_str(),
                        )?,
                );

            let result: UploadFile = client
                .post("https://upload.gofile.io/uploadfile")
                .multipart(form)
                .send()
                .await?
                .json()
                .await?;

            return Ok(result.data.download_page);
        }
    }

    return Err(MyError::ResourceNotAccessible);
}
