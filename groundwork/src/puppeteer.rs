use anyhow::{anyhow, Context, Result};
use atrium_api::types::string::{Did, Handle, Nsid};
use bsky_sdk::agent::config::Config;
use bsky_sdk::BskyAgent;
use tracing::{debug, info};

type Session =
    atrium_api::types::Object<atrium_api::com::atproto::server::create_session::OutputData>;
pub type Commit =
    atrium_api::types::Object<atrium_api::com::atproto::sync::get_latest_commit::OutputData>;

/// tell the local bsky repo to do stuff, as an authenticated user
pub struct Puppeteer {
    local_session: Option<Session>,
    local_agent: BskyAgent,
    bsky_session: Option<Session>,
    bsky_agent: BskyAgent,
}

impl Puppeteer {
    pub async fn new(addr: &str) -> Result<Self> {
        let local_agent = BskyAgent::builder()
            .config(Config {
                endpoint: addr.to_string(),
                ..Default::default()
            })
            .build()
            .await?;
        let bsky_agent = BskyAgent::builder().build().await?;
        Ok(Self {
            local_session: None,
            local_agent,
            bsky_session: None,
            bsky_agent,
        })
    }

    pub async fn login_bsky(&mut self, identifier: &str, password: &str) -> Result<()> {
        let session = self.bsky_agent.login(identifier, password).await?;
        debug!("bsky session created: {:?}", session);
        self.bsky_session = Some(session);
        Ok(())
    }

    pub async fn login_local(&mut self, identifier: &str, password: &str) -> Result<()> {
        let session = self.local_agent.login(identifier, password).await?;
        debug!("local session created: {:?}", session);
        self.local_session = Some(session);
        Ok(())
    }

    pub fn bsky_session(&self) -> Option<&Session> {
        self.bsky_session.as_ref()
    }

    pub fn session(&self) -> Option<&Session> {
        self.local_session.as_ref()
    }

    pub fn agent(&self) -> &BskyAgent {
        &self.local_agent
    }

    // com.atproto.sync.getLatestCommit
    pub async fn get_latest_commit(&self, did: Did) -> Result<Commit> {
        let commit = self
            .local_agent
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

    pub async fn fetch_repo(&self, did: Did) -> Result<Vec<u8>> {
        self.bsky_agent
            .api
            .com
            .atproto
            .sync
            .get_repo(
                atrium_api::com::atproto::sync::get_repo::ParametersData { did, since: None }
                    .into(),
            )
            .await
            .context("fetching repo from bluesky")
    }

    pub async fn import_bsky_to_local(&self, did: Did) -> Result<()> {
        let repo_car = self.fetch_repo(did).await?;

        println!("repo_car: {:?} bytes", repo_car.len());

        self.local_agent
            .api
            .com
            .atproto
            .repo
            .import_repo(repo_car)
            .await
            .context("importing car to local pds")?;

        let result = self
            .local_agent
            .api
            .com
            .atproto
            .sync
            .list_repos(
                atrium_api::com::atproto::sync::list_repos::ParametersData {
                    cursor: None,
                    limit: None,
                }
                .into(),
            )
            .await?;
        debug!("Repo imported: {:?}", result);

        Ok(())
    }

    pub async fn create_local_account(&self) -> Result<()> {
        let acct = self
            .local_agent
            .api
            .com
            .atproto
            .server
            .create_account(
                atrium_api::com::atproto::server::create_account::InputData {
                    email: Some("dummy@n0.computer".into()),
                    password: Some("password".into()),
                    handle: Handle::new("dummy.n0.social".into()).unwrap(),
                    did: None,
                    invite_code: None,
                    plc_op: None,
                    recovery_key: None,
                    verification_code: None,
                    verification_phone: None,
                }
                .into(),
            )
            .await?;
        info!("Dummy account created: {:?}", acct);
        Ok(())
    }
    pub async fn post(&self) -> Result<()> {
        let did = self
            .local_session
            .clone()
            .ok_or_else(|| anyhow!("no local session"))?
            .data
            .did;
        self.local_agent
            .api
            .com
            .atproto
            .repo
            .create_record(
                atrium_api::com::atproto::repo::create_record::InputData {
                    repo: did.into(),
                    collection: Nsid::new("app.bsky.feed.post".into()).unwrap(),
                    record: atrium_api::types::Unknown::Null,
                    rkey: None,
                    swap_commit: None,
                    validate: None,
                }
                .into(),
            )
            .await?;
        Ok(())
    }
}
