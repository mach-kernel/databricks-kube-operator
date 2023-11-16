/*
 * Databricks Accounts and Workspace REST API on ALL
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */


use reqwest;

use crate::apis::ResponseContent;
use super::{Error, configuration};


/// struct for typed errors of method [`dbfsadd_block`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DbfsaddBlockError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`dbfsclose`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DbfscloseError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`dbfscreate`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DbfscreateError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`dbfsdelete`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DbfsdeleteError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`dbfsget_status`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DbfsgetStatusError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`dbfslist`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DbfslistError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`dbfsmkdirs`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DbfsmkdirsError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`dbfsmove`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DbfsmoveError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`dbfsput`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DbfsputError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`dbfsread`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DbfsreadError {
    UnknownValue(serde_json::Value),
}


/// Appends a block of data to the stream specified by the input handle. If the handle does not exist,  this call will throw an exception with `RESOURCE_DOES_NOT_EXIST`.  If the block of data exceeds 1 MB, this call will throw an exception with `MAX_BLOCK_SIZE_EXCEEDED`. 
pub async fn dbfsadd_block(configuration: &configuration::Configuration, files_add_block: Option<crate::models::FilesAddBlock>) -> Result<serde_json::Value, Error<DbfsaddBlockError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/api/2.0/dbfs/add-block", local_var_configuration.base_path);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    local_var_req_builder = local_var_req_builder.json(&files_add_block);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<DbfsaddBlockError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Closes the stream specified by the input handle. If the handle does not exist,  this call throws an exception with `RESOURCE_DOES_NOT_EXIST`. 
pub async fn dbfsclose(configuration: &configuration::Configuration, files_close: Option<crate::models::FilesClose>) -> Result<serde_json::Value, Error<DbfscloseError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/api/2.0/dbfs/close", local_var_configuration.base_path);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    local_var_req_builder = local_var_req_builder.json(&files_close);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<DbfscloseError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Opens a stream to write to a file and returns a handle to this stream. There is a 10 minute idle timeout on this handle. If a file or directory already exists on the given path  and __overwrite__ is set to `false`, this call throws an exception with `RESOURCE_ALREADY_EXISTS`.  A typical workflow for file upload would be:  1. Issue a `create` call and get a handle. 2. Issue one or more `add-block` calls with the handle you have. 3. Issue a `close` call with the handle you have. 
pub async fn dbfscreate(configuration: &configuration::Configuration, files_create: Option<crate::models::FilesCreate>) -> Result<crate::models::FilesCreateResponse, Error<DbfscreateError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/api/2.0/dbfs/create", local_var_configuration.base_path);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    local_var_req_builder = local_var_req_builder.json(&files_create);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<DbfscreateError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Delete the file or directory (optionally recursively delete all files in the directory). This call throws an exception with `IO_ERROR` if the path is a non-empty directory and `recursive` is set to `false` or on other similar errors.  When you delete a large number of files, the delete operation is done in increments. The call returns a response after approximately 45 seconds with an error message (503 Service Unavailable) asking you to re-invoke the delete operation until the directory structure is fully deleted.  For operations that delete more than 10K files, we discourage using the DBFS REST API, but advise you to perform such operations in the context of a cluster, using the [File system utility (Dbutilsfs)](/dev-tools/databricks-utils.html#dbutils-fs). `Dbutilsfs` covers the functional scope of the DBFS REST API, but from notebooks. Running such operations using notebooks provides better control and manageability, such as selective deletes, and the possibility to automate periodic delete jobs. 
pub async fn dbfsdelete(configuration: &configuration::Configuration, files_delete: Option<crate::models::FilesDelete>) -> Result<serde_json::Value, Error<DbfsdeleteError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/api/2.0/dbfs/delete", local_var_configuration.base_path);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    local_var_req_builder = local_var_req_builder.json(&files_delete);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<DbfsdeleteError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Gets the file information for a file or directory.  If the file or directory does not exist, this call throws an exception with `RESOURCE_DOES_NOT_EXIST`. 
pub async fn dbfsget_status(configuration: &configuration::Configuration, path: &str) -> Result<crate::models::FilesFileInfo, Error<DbfsgetStatusError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/api/2.0/dbfs/get-status", local_var_configuration.base_path);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    local_var_req_builder = local_var_req_builder.query(&[("path", &path.to_string())]);
    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<DbfsgetStatusError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// List the contents of a directory, or details of the file. If the file or directory does not exist, this call throws an exception with `RESOURCE_DOES_NOT_EXIST`.  When calling list on a large directory, the list operation will time out after approximately 60 seconds. We strongly recommend using list only on directories containing less than 10K files and discourage using the DBFS REST API for operations that list more than 10K files. Instead, we recommend that you perform such operations in the context of a cluster, using the [File system utility (Dbutilsfs)](/dev-tools/databricks-utils.html#dbutils-fs), which provides the same functionality without timing out. 
pub async fn dbfslist(configuration: &configuration::Configuration, path: &str) -> Result<crate::models::FilesListStatusResponse, Error<DbfslistError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/api/2.0/dbfs/list", local_var_configuration.base_path);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    local_var_req_builder = local_var_req_builder.query(&[("path", &path.to_string())]);
    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<DbfslistError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Creates the given directory and necessary parent directories if they do not exist.  If a file (not a directory) exists at any prefix of the input path, this call throws an exception with `RESOURCE_ALREADY_EXISTS`. **Note**: If this operation fails, it might have succeeded in creating some of the necessary parent directories. 
pub async fn dbfsmkdirs(configuration: &configuration::Configuration, files_mk_dirs: Option<crate::models::FilesMkDirs>) -> Result<serde_json::Value, Error<DbfsmkdirsError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/api/2.0/dbfs/mkdirs", local_var_configuration.base_path);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    local_var_req_builder = local_var_req_builder.json(&files_mk_dirs);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<DbfsmkdirsError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Moves a file from one location to another location within DBFS.  If the source file does not exist, this call throws an exception with `RESOURCE_DOES_NOT_EXIST`. If a file already exists in the destination path, this call throws an exception with `RESOURCE_ALREADY_EXISTS`. If the given source path is a directory, this call always recursively moves all files.\", 
pub async fn dbfsmove(configuration: &configuration::Configuration, files_move: Option<crate::models::FilesMove>) -> Result<serde_json::Value, Error<DbfsmoveError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/api/2.0/dbfs/move", local_var_configuration.base_path);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    local_var_req_builder = local_var_req_builder.json(&files_move);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<DbfsmoveError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Uploads a file through the use of multipart form post.  It is mainly used for streaming uploads, but can also be used as a convenient single call for data upload.  Alternatively you can pass contents as base64 string.  The amount of data that can be passed (when not streaming) using the __contents__  parameter is limited to 1 MB. `MAX_BLOCK_SIZE_EXCEEDED` will be thrown if this limit is exceeded.  If you want to upload large files, use the streaming upload. For details, see :method:dbfs/create, :method:dbfs/addBlock, :method:dbfs/close. 
pub async fn dbfsput(configuration: &configuration::Configuration, files_put: Option<crate::models::FilesPut>) -> Result<serde_json::Value, Error<DbfsputError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/api/2.0/dbfs/put", local_var_configuration.base_path);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    local_var_req_builder = local_var_req_builder.json(&files_put);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<DbfsputError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Returns the contents of a file. If the file does not exist, this call throws an exception with `RESOURCE_DOES_NOT_EXIST`. If the path is a directory, the read length is negative, or if the offset is negative, this call throws an exception with  `INVALID_PARAMETER_VALUE`. If the read length exceeds 1 MB, this call throws an exception with `MAX_READ_SIZE_EXCEEDED`.  If `offset + length` exceeds the number of bytes in a file, it reads the contents until the end of file.\", 
pub async fn dbfsread(configuration: &configuration::Configuration, path: &str, offset: Option<i32>, length: Option<i32>) -> Result<crate::models::FilesReadResponse, Error<DbfsreadError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/api/2.0/dbfs/read", local_var_configuration.base_path);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    local_var_req_builder = local_var_req_builder.query(&[("path", &path.to_string())]);
    if let Some(ref local_var_str) = offset {
        local_var_req_builder = local_var_req_builder.query(&[("offset", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = length {
        local_var_req_builder = local_var_req_builder.query(&[("length", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<DbfsreadError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

