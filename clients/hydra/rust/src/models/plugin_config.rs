/*
 * ORY Hydra
 *
 * Welcome to the ORY Hydra HTTP API documentation. You will find documentation for all HTTP APIs here.
 *
 * The version of the OpenAPI document: v1.10.5
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PluginConfig {
    #[serde(rename = "Args")]
    pub args: Box<crate::models::PluginConfigArgs>,
    /// description
    #[serde(rename = "Description")]
    pub description: String,
    /// Docker Version used to create the plugin
    #[serde(rename = "DockerVersion", skip_serializing_if = "Option::is_none")]
    pub docker_version: Option<String>,
    /// documentation
    #[serde(rename = "Documentation")]
    pub documentation: String,
    /// entrypoint
    #[serde(rename = "Entrypoint")]
    pub entrypoint: Vec<String>,
    /// env
    #[serde(rename = "Env")]
    pub env: Vec<crate::models::PluginEnv>,
    #[serde(rename = "Interface")]
    pub interface: Box<crate::models::PluginConfigInterface>,
    /// ipc host
    #[serde(rename = "IpcHost")]
    pub ipc_host: bool,
    #[serde(rename = "Linux")]
    pub linux: Box<crate::models::PluginConfigLinux>,
    /// mounts
    #[serde(rename = "Mounts")]
    pub mounts: Vec<crate::models::PluginMount>,
    #[serde(rename = "Network")]
    pub network: Box<crate::models::PluginConfigNetwork>,
    /// pid host
    #[serde(rename = "PidHost")]
    pub pid_host: bool,
    /// propagated mount
    #[serde(rename = "PropagatedMount")]
    pub propagated_mount: String,
    #[serde(rename = "User", skip_serializing_if = "Option::is_none")]
    pub user: Option<Box<crate::models::PluginConfigUser>>,
    /// work dir
    #[serde(rename = "WorkDir")]
    pub work_dir: String,
    #[serde(rename = "rootfs", skip_serializing_if = "Option::is_none")]
    pub rootfs: Option<Box<crate::models::PluginConfigRootfs>>,
}

impl PluginConfig {
    pub fn new(args: crate::models::PluginConfigArgs, description: String, documentation: String, entrypoint: Vec<String>, env: Vec<crate::models::PluginEnv>, interface: crate::models::PluginConfigInterface, ipc_host: bool, linux: crate::models::PluginConfigLinux, mounts: Vec<crate::models::PluginMount>, network: crate::models::PluginConfigNetwork, pid_host: bool, propagated_mount: String, work_dir: String) -> PluginConfig {
        PluginConfig {
            args: Box::new(args),
            description,
            docker_version: None,
            documentation,
            entrypoint,
            env,
            interface: Box::new(interface),
            ipc_host,
            linux: Box::new(linux),
            mounts,
            network: Box::new(network),
            pid_host,
            propagated_mount,
            user: None,
            work_dir,
            rootfs: None,
        }
    }
}


