//! This file is inspired from https://github.com/pfzetto/axum-oidc

use testcontainers::{
    ContainerAsync,
    core::{CmdWaitFor, ExecCommand, Image, WaitFor},
    runners::AsyncRunner,
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
        vec![
            WaitFor::message_on_stdout("Listening on:"),
            WaitFor::message_on_stdout(
                "Running the server in development mode. DO NOT use this configuration in production.",
            ),
        ]
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

impl Default for Realm {
    fn default() -> Self {
        Self {
            name: "realm_test".to_string(),
            clients: vec![Client::default()],
            users: vec![
                User {
                    username: "admin".to_string(),
                    email: "admin@example.com".to_string(),
                    ..Default::default()
                },
                User {
                    username: "user_1".to_string(),
                    email: "user_1@example.com".to_string(),
                    ..Default::default()
                },
                User {
                    username: "user_2".to_string(),
                    email: "user_2@example.com".to_string(),
                    ..Default::default()
                },
            ],
        }
    }
}

#[derive(Clone)]
pub struct Client {
    pub client_id: String,
    pub client_secret: Option<String>,
}

impl Default for Client {
    fn default() -> Self {
        Self {
            client_id: "0".to_owned(),
            client_secret: Some("coucou".to_owned()),
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

impl Default for User {
    fn default() -> Self {
        Self {
            username: "John Doe".to_string(),
            email: "john.doe@example.com".to_string(),
            firstname: "John".to_string(),
            lastname: "Doe".to_string(),
            password: "password".to_string(),
        }
    }
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

        keycloak
            .container
            .exec(
                ExecCommand::new([
                    "/opt/keycloak/bin/kcadm.sh",
                    "config",
                    "credentials",
                    "--server",
                    "http://localhost:8080",
                    "--realm",
                    "master",
                    "--user",
                    "admin",
                    "--password",
                    "admin",
                ])
                .with_cmd_ready_condition(CmdWaitFor::exit_code(0)),
            )
            .await
            .unwrap();

        for realm in keycloak.realms.iter() {
            if realm.name != "master" {
                keycloak.create_realm(&realm.name).await;
            }
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
                keycloak.create_user(user, &realm.name).await;
            }
        }

        Ok(keycloak)
    }

    pub fn url(&self) -> &str {
        &self.url
    }

    async fn create_realm(&self, name: &str) {
        self.container
            .exec(
                ExecCommand::new([
                    "/opt/keycloak/bin/kcadm.sh",
                    "create",
                    "realms",
                    "-s",
                    &format!("realm={name}"),
                    "-s",
                    "enabled=true",
                ])
                .with_cmd_ready_condition(CmdWaitFor::exit_code(0)),
            )
            .await
            .unwrap();
    }

    async fn create_client(&self, client_id: &str, client_secret: Option<&str>, realm: &str) {
        if let Some(client_secret) = client_secret {
            self.container
                .exec(
                    ExecCommand::new([
                        "/opt/keycloak/bin/kcadm.sh",
                        "create",
                        "clients",
                        "-r",
                        &realm,
                        "-s",
                        &format!("clientId={client_id}"),
                        "-s",
                        &format!("secret={client_secret}"),
                        "-s",
                        "redirectUris=[\"*\"]",
                    ])
                    .with_cmd_ready_condition(CmdWaitFor::exit_code(0)),
                )
                .await
                .unwrap();
        } else {
            self.container
                .exec(
                    ExecCommand::new([
                        "/opt/keycloak/bin/kcadm.sh",
                        "create",
                        "clients",
                        "-r",
                        &realm,
                        "-s",
                        &format!("clientId={client_id}"),
                        "-s",
                        "redirectUris=[\"*\"]",
                    ])
                    .with_cmd_ready_condition(CmdWaitFor::exit_code(0)),
                )
                .await
                .unwrap();
        }
    }

    pub async fn create_user(&self, user: &User, realm: &str) -> String {
        let stderr = self
            .container
            .exec(
                ExecCommand::new([
                    "/opt/keycloak/bin/kcadm.sh",
                    "create",
                    "users",
                    "-r",
                    &realm,
                    "-s",
                    &format!("username={}", user.username),
                    "-s",
                    "enabled=true",
                    "-s",
                    "emailVerified=true",
                    "-s",
                    &format!("email={}", user.email),
                    "-s",
                    &format!("firstName={}", user.firstname),
                    "-s",
                    &format!("lastName={}", user.lastname),
                ])
                .with_cmd_ready_condition(CmdWaitFor::exit_code(0)),
            )
            .await
            .unwrap()
            .stderr_to_vec()
            .await
            .unwrap();

        let stderr = String::from_utf8_lossy(&stderr);
        let id = stderr.split('\'').nth(1).unwrap().to_string();

        self.container
            .exec(
                ExecCommand::new([
                    "/opt/keycloak/bin/kcadm.sh",
                    "set-password",
                    "-r",
                    &realm,
                    "--username",
                    &user.username,
                    "--new-password",
                    &user.password,
                ])
                .with_cmd_ready_condition(CmdWaitFor::exit_code(0)),
            )
            .await
            .unwrap();
        id
    }
}
