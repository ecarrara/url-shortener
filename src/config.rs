use serde::Deserialize;

#[derive(Deserialize, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
pub struct Config {
    /// Node id.
    pub node_id: u16,
    /// Server listen address.
    pub listen: String,
    /// Storage configuration.
    pub storage: StorageConfig,
}

#[derive(Deserialize, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum StorageConfig {
    Mongo(MongoConfig),
}

#[derive(Deserialize, Debug, PartialEq)]
pub struct MongoConfig {
    /// MongoDB database URI.
    pub uri: String,
    /// MongoDB database name.
    pub database: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserialie_config() {
        let content = r#"
        {
          "node_id":  1,
          "listen":  "0.0.0.0:3000",
          "storage": {
            "mongo": {
                "uri":  "mongodb://localhost:27017",
                "database": "test1"
            }
          }
        }
        "#;

        let config: Config = serde_json::from_str(content).expect("failed to parse config");

        assert_eq!(
            config,
            Config {
                node_id: 1,
                listen: "0.0.0.0:3000",
                storage: StorageConfig::Mongo(MongoConfig {
                    uri: "mongodb://localhost:27017".to_string(),
                    database: "test1".to_string(),
                })
            }
        );
    }
}
