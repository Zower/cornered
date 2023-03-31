use std::{path::Path, time::Duration};

use anyhow::anyhow;
use rusqlite::params;

use crate::{
    books::CONNECTION,
    types::{CodeResponse, DeviceFlowResponse, FileResponse, GithubUser},
};

// static CONNECTION: OnceCell<Mutex<Connection>> = OnceCell::new();

static CLIENT_ID: &str = "bc2ede3adf378ac47e57";

static API_URL: &str = "https://api.github.com";

/// Use the response to display the user_code to the user, asking them to go to verification_uri, then call poll().
pub fn auth() -> anyhow::Result<DeviceFlowResponse> {
    let response = ureq::post("https://github.com/login/device/code")
        .query("client_id", CLIENT_ID)
        .query("scope", "workflow,repo,user")
        .set("accept", "application/json")
        .call()?
        .into_json::<DeviceFlowResponse>()?;

    Ok(response)
}

pub fn poll(ongoing: DeviceFlowResponse) -> anyhow::Result<GithubUser> {
    let response = loop {
        std::thread::sleep(Duration::from_secs(ongoing.interval));
        let response = ureq::post("https://github.com/login/oauth/access_token")
            .query("client_id", CLIENT_ID)
            .query("device_code", &ongoing.device_code)
            .query("grant_type", "urn:ietf:params:oauth:grant-type:device_code")
            .set("accept", "application/json")
            .call()?
            .into_json::<CodeResponse>();

        if let Ok(response) = response {
            break response;
        }
    };

    let user = ureq::get(&github("/user"))
        .set(
            "Authorization",
            format!("Bearer {}", response.access_token).as_str(),
        )
        .call()?
        .into_json::<GithubUser>()?;

    CONNECTION
        .get()
        .ok_or(anyhow!("No connection"))?
        .lock()
        .execute(
            "INSERT INTO tokens (github_id, token) VALUES (?1, ?2)",
            params!(user.id, &response.access_token),
        )?;

    Ok(user)
}

pub fn get_token(user: GithubUser) -> anyhow::Result<String> {
    let token: String = CONNECTION
        .get()
        .ok_or(anyhow!("No connection"))?
        .lock()
        .query_row(
            "SELECT token FROM tokens WHERE github_id = ?1",
            params!(user.id),
            |row| row.get(0),
        )?;

    Ok(token)
}

pub fn upload_file(repo: String, path: String, user: GithubUser) -> anyhow::Result<()> {
    let file_name = Path::new(&path)
        .file_name()
        .ok_or(anyhow!("No file name"))?;

    #[allow(deprecated)]
    let file = base64::encode(std::fs::read(&path).unwrap());

    let token: String = CONNECTION
        .get()
        .ok_or(anyhow!("No connection"))?
        .lock()
        .query_row(
            "SELECT token FROM tokens WHERE github_id = ?1",
            params!(user.id),
            |row| row.get(0),
        )?;

    ureq::put(&format!(
        "https://api.github.com/repos/{}/{}/contents/{:?}",
        user.login, repo, file_name
    ))
    .set("Authorization", format!("Bearer {}", token).as_str())
    .set("accept", "application/vnd.github+json")
    .send_json(ureq::json!({
        "message": format!("Synchronize {:?}", &file_name),
        "content": file
    }))?;

    Ok(())
}

pub fn get_files(repo: String, user: GithubUser) -> anyhow::Result<Vec<FileResponse>> {
    let token: String = CONNECTION
        .get()
        .ok_or(anyhow!("No connection"))?
        .lock()
        .query_row(
            "SELECT token FROM tokens WHERE github_id = ?1",
            params!(user.id),
            |row| row.get(0),
        )?;

    Ok(ureq::get(&format!(
        "https://api.github.com/repos/{}/{}/contents",
        user.login, repo
    ))
    .set("Authorization", &format!("Bearer {}", token))
    .set("accept", "application/vnd.github+json")
    .call()?
    .into_json::<Vec<FileResponse>>()?)
}

fn github(path: &str) -> String {
    format!("{}{}", API_URL, path)
}
