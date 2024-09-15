//! This file is inspired from https://github.com/pfzetto/axum-oidc

use testcontainers::{
    core::Image,
    core::{ExecCommand, WaitFor},
    runners::AsyncRunner,
    ContainerAsync,
};

#[derive(Debug, Default, Clone)]
struct KeycloakImage;

const NAME: &str = "quay.io/keycloak/keycloak";
const TAG: &str = "25.0";

impl Image for KeycloakImage {
    fn name(&self) -> &str {
        NAME
    }

    fn tag(&self) -> &str {
        TAG
    }

    fn ready_conditions(&self) -> Vec<WaitFor> {
        vec![WaitFor::message_on_stdout("Listening on:")]
    }

    fn env_vars(
        &self,
    ) -> impl IntoIterator<
        Item = (
            impl Into<std::borrow::Cow<'_, str>>,
            impl Into<std::borrow::Cow<'_, str>>,
        ),
    > {
        [
            ("KEYCLOAK_ADMIN", "admin"),
            ("KEYCLOAK_ADMIN_PASSWORD", "admin"),
        ]
    }

    fn cmd(&self) -> impl IntoIterator<Item = impl Into<std::borrow::Cow<'_, str>>> {
        ["start-dev"]
    }
}

pub struct Keycloak {
    container: ContainerAsync<KeycloakImage>,
    realms: Vec<Realm>,
    url: String,
}

#[derive(Clone)]
pub struct Realm {
    pub name: String,
    pub clients: Vec<Client>,
    pub users: Vec<User>,
}

#[derive(Clone)]
pub struct Client {
    pub client_id: String,
    pub client_secret: Option<String>,
    pub redirect_uris: Vec<String>,
}

impl Default for Client {
    fn default() -> Self {
        Self {
            client_id: "0".to_owned(),
            client_secret: None,
            redirect_uris: ["*".to_owned()].to_vec(),
        }
    }
}

#[derive(Clone)]
pub struct User {
    pub username: String,
    pub email: String,
    pub firstname: String,
    pub lastname: String,
    pub password: String,
}

impl Keycloak {
    pub async fn start(
        realms: Vec<Realm>,
    ) -> Result<Keycloak, Box<dyn std::error::Error + 'static>> {
        let container = KeycloakImage.start().await?;

        let keycloak = Self {
            url: format!(
                "http://localhost:{}",
                container.get_host_port_ipv4(8080).await?,
            ),
            container,
            realms,
        };

        for realm in keycloak.realms.iter() {
            keycloak.create_realm(&realm.name).await;
            for client in realm.clients.iter() {
                keycloak
                    .create_client(
                        &client.client_id,
                        client.client_secret.as_deref(),
                        &realm.name,
                    )
                    .await;
            }
            for user in realm.users.iter() {
                keycloak
                    .create_user(
                        &user.username,
                        &user.email,
                        &user.firstname,
                        &user.lastname,
                        &user.password,
                        &realm.name,
                    )
                    .await;
            }
        }

        Ok(keycloak)
    }

    pub fn url(&self) -> &str {
        &self.url
    }

    async fn create_realm(&self, name: &str) {
        self.execute(format!(
            "/opt/keycloak/bin/kcadm.sh create realms -s realm={name} -s enabled=true"
        ))
        .await;
    }

    async fn create_client(&self, client_id: &str, client_secret: Option<&str>, realm: &str) {
        if let Some(client_secret) = client_secret {
            self.execute(format!(
                r#"/opt/keycloak/bin/kcadm.sh create clients -r {realm} -f - << EOF
            {{
                "clientId": "{client_id}",
                "secret": "{client_secret}",
                "redirectUris": ["*"]
            }}
            EOF
            "#
            ))
            .await;
        } else {
            self.execute(format!(
                r#"/opt/keycloak/bin/kcadm.sh create clients -r {realm} -f - << EOF
            {{
                "clientId": "{client_id}",
                "redirectUris": ["*"]
            }}
            EOF
            "#
            ))
            .await;
        }
    }

    async fn create_user(
        &self,
        username: &str,
        email: &str,
        firstname: &str,
        lastname: &str,
        password: &str,
        realm: &str,
    ) {
        let id = self.execute(format!("/opt/keycloak/bin/kcadm.sh create users -r {realm} -s username={username} -s enabled=true -s emailVerified=true -s email={email} -s firstName={firstname} -s lastName={lastname}"))
    .await;
        self.execute(format!("/opt/keycloak/bin/kcadm.sh set-password -r {realm} --username {username} --new-password {password}"))
        .await;
        id
    }

    async fn execute(&self, cmd: String) {
        let _ = self.container.exec(ExecCommand::new([cmd])).await;
    }
}
