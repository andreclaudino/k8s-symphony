use schemars::JsonSchema;
use serde::{Serialize, Deserialize};

#[derive(Deserialize, Serialize, Clone, Debug, JsonSchema)]
#[serde(rename_all="camelCase")]
pub enum FlowSource {

}