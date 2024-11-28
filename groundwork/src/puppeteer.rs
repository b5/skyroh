use anyhow::Result;
use atrium_xrpc_client::reqwest::ReqwestClient;
use bsky_sdk::BskyAgent;
use tracing::debug;

type Session =
    atrium_api::types::Object<atrium_api::com::atproto::server::create_session::OutputData>;
pub type Commit =
    atrium_api::types::Object<atrium_api::com::atproto::sync::get_latest_commit::OutputData>;

/// tell the local bsky repo to do stuff, as an authenticated user
pub struct Puppeteer {
    session: Session,
    agent: BskyAgent,
}

impl Puppeteer {
    pub async fn new(addr: &str, username: &str, password: &str) -> Result<Self> {
        let client = ReqwestClient::new(addr);
        let agent = BskyAgent::builder().client(client).build().await?;
        let session = agent.login(username, password).await?;
        debug!("session created: {:?}", session);
        Ok(Self { agent, session })
    }

    // com.atproto.sync.getLatestCommit
    pub async fn get_latest_commit(&self) -> Result<Commit> {
        let did = self.session.did.clone();
        let commit = self
            .agent
            .api
            .com
            .atproto
            .sync
            .get_latest_commit(
                atrium_api::com::atproto::sync::get_latest_commit::ParametersData { did }.into(),
            )
            .await?;
        debug!("Latest commit: {:?}", commit);

        Ok(commit)
    }

    pub async fn fetch_repo(&self) -> Result<()> {
        let did = self.session.did.clone();
        let repo = self
            .agent
            .api
            .com
            .atproto
            .sync
            .get_repo(
                atrium_api::com::atproto::sync::get_repo::ParametersData { did, since: None }
                    .into(),
            )
            .await?;
        debug!("Repo fetched: {:?}", repo);

        Ok(())
    }
}
