use std::{path::Path, time::Duration};

use anyhow::anyhow;
use r2d2_sqlite::rusqlite::params;
use rayon::prelude::{IntoParallelIterator, ParallelIterator};

use crate::{
    books::{Database, DATA_DIR, POOL},
    fonts::FONTS,
    helpers::ResponseOkStatus,
    types::{
        CodeResponse, Definitions, DeviceFlowResponse, FileResponse, GithubUser, GithubUserJson,
        UploadedFile,
    },
};

static CLIENT_ID: &str = "bc2ede3adf378ac47e57";
static API_URL: &str = "https://api.github.com";

pub fn get_users() -> anyhow::Result<Vec<GithubUser>> {
    let connection = POOL.get().unwrap().get().unwrap();

    let Some(primary) = connection
        .prepare("SELECT github_id FROM current_token")?
        .query_map([], |row| Ok(row.get::<_, u64>(0)?))?
        .next()
        .map(|it| it.unwrap()) else {
            return Ok(vec![]);
        };

    let mut stmt = connection.prepare("SELECT github_id, display_name, token FROM tokens")?;

    let users = stmt
        .query_map(params![], |row| {
            Ok(GithubUserJson {
                id: row.get(0)?,
                display_name: row.get(1)?,
            })
        })?
        .collect::<Result<Vec<_>, _>>()?;

    Ok(users
        .into_iter()
        .map(|json| GithubUser::from(json.id == primary, json))
        .collect())
}

pub fn get_primary_user() -> anyhow::Result<Option<GithubUser>> {
    Ok(get_users()?.into_iter().find(|user| user.is_primary))
}

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

// TODO: Consider StreamSink and doing this from auth()
pub fn poll(ongoing: DeviceFlowResponse) -> anyhow::Result<GithubUser> {
    let response = loop {
        std::thread::sleep(Duration::from_secs(ongoing.interval));

        let response = ureq::post("https://github.com/login/oauth/access_token")
            .query("client_id", CLIENT_ID)
            .query("device_code", &ongoing.device_code)
            .query("grant_type", "urn:ietf:params:oauth:grant-type:device_code")
            .set("accept", "application/json")
            .call();

        match response {
            Ok(response) => {
                if let Ok(json) = response.into_json::<CodeResponse>() {
                    break json;
                }
            }
            Err(e) => return Err(anyhow!("Error: {}", e)),
        }
    };

    let user = ureq::get(&github("/user"))
        .set(
            "Authorization",
            format!("Bearer {}", response.access_token).as_str(),
        )
        .call()?
        .into_json::<GithubUserJson>()?;

    let connection = POOL.get().unwrap().get()?;

    connection.execute("BEGIN TRANSACTION", [])?;

    connection.execute(
        "INSERT INTO tokens (github_id, display_name, token) VALUES (?1, ?2, ?3)",
        params!(user.id, user.display_name, &response.access_token),
    )?;

    connection.execute(
        "INSERT INTO current_token (github_id) VALUES (?1)",
        params!(user.id),
    )?;

    connection.execute("COMMIT", [])?;

    Ok(GithubUser::from(true, user))
}

pub fn upload_file(repo: String, uuid: String, user: GithubUserJson) -> anyhow::Result<()> {
    let book = Database {}.get_book(uuid.clone())?;

    let file_name = Path::new(&book.path)
        .file_name()
        .ok_or(anyhow!("No file name"))?;

    let content = std::fs::read(&book.path)?;

    let upload = UploadedFile {
        uuid,
        file_name: file_name.to_str().unwrap().to_string(),
        content,
        position: book.position,
    };

    #[allow(deprecated)]
    let file = base64::encode(bincode::serialize(&upload)?);

    let token: String = POOL
        .get()
        .ok_or(anyhow!("No connection"))?
        .get()?
        .query_row(
            "SELECT token FROM tokens WHERE github_id = ?1",
            params!(user.id),
            |row| row.get(0),
        )?;

    let old = get_file(&user.display_name, &repo, &book.uuid, &token)?;

    let json = ureq::json!({
        "message": format!("Synchronize {:?}", &file_name),
        "content": file,
        "sha": old.map(|v| v.sha),
    });

    ureq::put(&format!(
        "https://api.github.com/repos/{}/{}/contents/{}.crn",
        user.display_name, repo, book.uuid
    ))
    .set("Authorization", format!("Bearer {}", token).as_str())
    .set("accept", "application/vnd.github+json")
    .send_json(json)?;

    Ok(())
}

pub fn update_files(repo: String, user: GithubUserJson) -> anyhow::Result<()> {
    let token: String = POOL
        .get()
        .ok_or(anyhow!("No connection"))?
        .get()?
        .query_row(
            "SELECT token FROM tokens WHERE github_id = ?1",
            params!(user.id),
            |row| row.get(0),
        )?;

    let responses = ureq::get(&format!(
        "https://api.github.com/repos/{}/{}/contents",
        user.display_name, repo
    ))
    .set("Authorization", &format!("Bearer {}", token))
    .set("accept", "application/vnd.github+json")
    .call()?
    .into_json::<Vec<FileResponse>>()?;

    let files = responses
        .into_par_iter()
        .filter(|response| response.name.ends_with(".crn"))
        .map(|file| {
            ureq::get(&file.download_url)
                .call()
                .map_err(|err| anyhow!("Could not download file: {err:?}"))
                .and_then(|response| {
                    bincode::deserialize_from::<_, UploadedFile>(response.into_reader())
                        .map_err(|err| anyhow!("Could not deserialize: {err:?}"))
                })
        })
        .filter_map(Result::ok)
        .collect::<Vec<_>>();

    let app_dir = DATA_DIR.get().ok_or(anyhow!("No data dir"))?.join("books");

    std::fs::create_dir_all(&app_dir).expect(&format!("able to create directory: {:?}", &app_dir));

    files.into_par_iter().for_each(|file| {
        let path = app_dir.join(&file.file_name);
        std::fs::write(&path, &file.content).expect("able to write to file");

        Database {}
            .add_synced_book(file, &path)
            .expect("synced book to be added");
    });

    Ok(())
}

pub fn font_search(query: String) -> Vec<String> {
    FONTS
        .into_par_iter()
        .filter(|font| {
            let font = font.to_lowercase();
            let query = query.to_lowercase();
            font.contains(&query)
        })
        .map(|font| font.to_string())
        .collect()
}

pub fn get_definition(mut word: String) -> anyhow::Result<Definitions> {
    word.retain(|c| !r#"(),".;:'"#.contains(c));

    Ok(ureq::get(&format!(
        "https://api.dictionaryapi.dev/api/v2/entries/en/{}",
        word.trim().to_lowercase()
    ))
    .call()?
    .ok_status()?
    .into_json::<Vec<Definitions>>()?
    .remove(0))
}

fn get_file(
    user: &str,
    repo: &str,
    path: &str,
    token: &str,
) -> anyhow::Result<Option<FileResponse>> {
    let response = match ureq::get(&github(&format!(
        "/repos/{}/{}/contents/{}",
        user, repo, path
    )))
    .set("accept", "application/vnd.github+json")
    .set("Authorization", format!("Bearer {}", token).as_str())
    .call()
    {
        Ok(response) => response.into_json::<FileResponse>()?,
        Err(ureq::Error::Status(404, _)) => return Ok(None),
        e => return Err(anyhow!("Error getting file {e:?}")),
    };

    Ok(Some(response))
}

fn github(path: &str) -> String {
    format!("{}{}", API_URL, path)
}
