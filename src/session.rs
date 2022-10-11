use crate::api;
use serde_json::Result;

#[derive(Debug)]
pub struct Session {
    user_id: String,

    http_client: reqwest::Client,

    access_token: String,
    client_id: String,
    client_token: String,
}

impl Session {
    pub async fn new(dc: &str, key: &str, user_id: &str) -> Result<Self> {
        let http_client = reqwest::Client::new();
        let access_token_resp = api::session::get_access_token(&http_client, dc, key).await?;
        let client_token =
            api::session::get_client_token(&http_client, &access_token_resp.client_id).await?;

        Ok(Self {
            user_id: user_id.to_string(),
            http_client,
            access_token: access_token_resp.access_token,
            client_id: access_token_resp.client_id,
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
            .await.unwrap().text().await.unwrap();
        // println!("Res: {}", res);

        serde_json::from_str(&res)
    }
}

#[cfg(test)]
mod tests {
    use crate::session::Session;

    #[tokio::test]
    async fn test_new_session() {
        let dc = std::env::var("SPOTIFY_DC").unwrap();
        let key = std::env::var("SPOTIFY_KEY").unwrap();
        let user_id = std::env::var("SPOTIFY_USER_ID").unwrap();

        let s = Session::new(&dc, &key, &user_id).await.unwrap();
        println!("{:?}", s);
        let root_list = s.get_root_list().await.unwrap();
        println!("{:?}", root_list);
    }
}
