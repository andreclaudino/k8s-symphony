use kube::CustomResource;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use super::{FlowSource, SymphonyInput, SymphonyOutput, SymphonyState};


/// This represents the spec for a Symphony Job.
/// The spect contains:
///     * the flow source (flowSource)
///     * the input parameters definition (input)
///     * the output parameters definition (output)
#[derive(Deserialize, Serialize, CustomResource, Clone, Debug, JsonSchema)]
#[kube(group="neurono.ml", version="v1", kind="Symphony", namespaced)]
#[kube(status="SymphonyStatus")]
#[serde(rename_all="camelCase")]
pub struct SymphonySpec {
    pub flow_source: FlowSource,
    pub input: SymphonyInput,
    pub output: SymphonyOutput,
}

#[derive(Deserialize, Serialize, Clone, Debug, JsonSchema)]
pub struct SymphonyStatus {
    pub state: SymphonyState
}