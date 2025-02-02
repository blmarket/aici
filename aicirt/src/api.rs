use aici_abi::{StorageCmd, TokenId};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use crate::HashMap;

pub type ModuleInstId = usize;

#[derive(Serialize, Deserialize)]
pub struct AiciPostPreProcessReq {
    // Executed first
    pub post_ops: Vec<AiciPostOp>,
    // Executed second
    pub pre_ops: Vec<AiciPreOp>,
    // Executed third
    pub freed: Vec<ModuleInstId>,
}

#[derive(Serialize, Deserialize)]
pub struct AiciPostPreProcessResp {
    pub post_seqs: HashMap<ModuleInstId, SequenceResult<AiciPostProcessResultInner>>,
    pub pre_seqs: HashMap<ModuleInstId, SequenceResult<AiciPreProcessResultInner>>,
}

#[derive(Serialize, Deserialize)]
pub struct AiciPreProcessResultInner {
    pub suspend: bool,
    pub num_forks: usize,
    pub ff_tokens: Vec<TokenId>,
}

#[derive(Serialize, Deserialize)]
pub struct AiciMidProcessReq {
    pub ops: Vec<AiciMidOp>,
}

#[derive(Serialize, Deserialize)]
pub struct AiciMidProcessResp {
    pub seqs: HashMap<ModuleInstId, SequenceResult<AiciMidProcessResultInner>>,
    pub num_seqs: usize,
}

#[derive(Serialize, Deserialize)]
pub struct AiciMidProcessResultInner {
    pub ff_tokens: Vec<TokenId>,
    pub backtrack: u32,
}

#[derive(Serialize, Deserialize)]
pub struct AiciPostProcessResultInner {
    pub stop: bool,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct AiciPreOp {
    // This assigns id to the module currently instantiated with req_id
    pub id: ModuleInstId,
    pub req_id: String,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct AiciMidOp {
    pub id: ModuleInstId,
    pub clone_id: Option<ModuleInstId>,
}

#[derive(Serialize, Deserialize)]
pub struct AiciPostOp {
    pub id: ModuleInstId,
    pub tokens: Vec<Token>,
    #[serde(default)]
    pub backtrack: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SequenceResult<T = ()> {
    pub error: String,
    pub result: Option<T>,
    // StorageCmd::ReadVar are not recorded
    pub storage: Vec<StorageCmd>,
    pub logs: String,
    pub micros: u64,
}

impl<T> SequenceResult<T> {
    pub fn from_error(error: String) -> SequenceResult<T> {
        SequenceResult {
            logs: error.clone(),
            error,
            result: None,
            storage: vec![],
            micros: 0,
        }
    }
    pub fn clone_with<S>(&self, result: Option<S>) -> SequenceResult<S> {
        SequenceResult {
            error: self.error.clone(),
            result,
            storage: self.storage.clone(),
            logs: self.logs.clone(),
            micros: self.micros,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct MkModuleReq {
    pub binary: String,
}

#[derive(Serialize, Deserialize)]
pub struct MkModuleResp {
    pub module_id: String,
    pub wasm_size: usize,
    pub compiled_size: usize,
    pub time: u64,
}

#[derive(Serialize, Deserialize)]
pub struct SetTagsReq {
    pub module_id: String,
    pub tags: Vec<String>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct TagInfo {
    pub tag: String,
    pub module_id: String,
    pub updated_at: u64, // unix time
    pub updated_by: String,
    pub wasm_size: u64,
    pub compiled_size: u64,
}

#[derive(Serialize, Deserialize)]
pub struct GetTagsResp {
    pub tags: Vec<TagInfo>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct InstantiateReq {
    pub req_id: String,
    // [TokenId] or str
    pub prompt: Value,
    pub module_id: String, // or tag name
    #[serde(default)]
    pub module_arg: Value,
}

pub type Token = TokenId;

#[derive(Serialize, Deserialize, Debug)]
pub struct TokensResp {
    pub vocab_size: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AuthInfo {
    pub user: String,
    pub is_admin: bool,
}

impl AuthInfo {
    pub fn local_user() -> Self {
        AuthInfo {
            user: "local".to_string(),
            is_admin: false,
        }
    }
}
