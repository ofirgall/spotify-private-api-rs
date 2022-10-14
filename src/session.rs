use crate::api;
use crate::Result;

#[derive(Debug)]
pub struct Session {
    user_id: String,

    http_client: reqwest::Client,

    access_token: String,
    client_token: String,
}

impl Session {
    /// Creates a new session for spotify private api
    ///
    pub async fn new(dc: &str, key: &str, user_id: &str) -> Result<Self> {
        let http_client = reqwest::Client::new();
        let access_token_resp = api::session::get_access_token(&http_client, dc, key).await?;
        let client_token =
            api::session::get_client_token(&http_client, &access_token_resp.client_id).await?;

        Ok(Self {
            user_id: user_id.to_string(),
            http_client,
            access_token: access_token_resp.access_token,
            client_token,
        })
    }

    pub async fn get_root_list(&self) -> Result<api::folders::RootList> {
        let res = self.http_client
            .get(format!("https://spclient.wg.spotify.com/playlist/v2/user/{}/rootlist?decorate=revision%2Clength%2Cattributes%2Ctimestamp%2Cowner", self.user_id))
            .header("Accept", "application/json")
            .header("app-platform", "WebPlayer")
            .header("authorization", format!("Bearer {}", self.access_token))
            .header("client-token", &self.client_token)
            .send()
            .await?
            .text()
            .await?;

        Ok(serde_json::from_str(&res)?)
    }

    pub async fn send_changes(&self, changes: &api::folders::Changes) -> Result<()> {
        self.http_client
            .post(format!(
                "https://spclient.wg.spotify.com/playlist/v2/user/{}/rootlist/changes",
                self.user_id
            ))
            .header("Accept", "application/json")
            .header("app-platform", "WebPlayer")
            .header("authorization", format!("Bearer {}", self.access_token))
            .header("client-token", &self.client_token)
            .header("content-type", "application/json;charset=UTF-8") // TODO: parse names in this charsets
            .body(serde_json::to_string(changes)?)
            .send()
            .await?
            .error_for_status()?;

        Ok(())
    }
}

// TODO: Write a system test framework to work with a real spotify connection
#[cfg(test)]
mod tests {
    use crate::session::Session;

    async fn session_from_env() -> Session {
        let dc = std::env::var("SPOTIFY_DC").expect("failed to get SPOTIFY_DC from ENV");
        let key = std::env::var("SPOTIFY_KEY").expect("failed to get SPOTIFY_KEY from ENV");
        let user_id =
            std::env::var("SPOTIFY_USER_ID").expect("failed to get SPOTIFY_USER_ID from ENV");

        Session::new(&dc, &key, &user_id)
            .await
            .expect("Failed to create session")
    }

    #[cfg_attr(not(feature = "system-tests"), ignore)]
    #[tokio::test]
    async fn test_new_session() {
        let root_list = session_from_env()
            .await
            .get_root_list()
            .await
            .expect("failed to get root list");
        println!("{:?}", root_list); // TODO: validate root list
    }

    #[cfg_attr(not(feature = "system-tests"), ignore)]
    #[tokio::test]
    async fn test_create_folder() {
        let s = session_from_env().await;

        let root_list = s.get_root_list().await.expect("failed to get root list");
        let changes = root_list
            .new_request()
            .add("TestFolder", &root_list.generate_folder_uri(), 0, 2)
            .build();

        s.send_changes(&changes)
            .await
            .expect("failed to send changes");
    }
}
