#![no_std] // 使用 no_std，表明不使用标准库
use gstd::{prelude::*, ActorId};
use gstd::collections::{BTreeMap, BTreeSet};
use gmeta::{In, InOut, Metadata, Out};

// 合约初始化时传入的参数
#[derive(Debug, Clone, Encode, Decode, TypeInfo)]
pub struct TmgInit {
    pub owner: ActorId,
}

// 存储合约的整体状态
#[derive(Debug, Clone, Encode, Decode, TypeInfo)]
pub struct TradingPairState {
    pub owner: ActorId,
    pub trading_pairs: BTreeMap<String, TradingPair>,
    pub counter: u64,
    pub authorized_editors: BTreeSet<ActorId>,
    pub event_logs: Vec<EventLog>,
    pub event_counter: u64,
    pub editor_counter: u64,
}

// 表示一个交易对的详细信息
#[derive(Debug, Clone, Encode, Decode, TypeInfo)]
pub struct TradingPair {
    pub id: String,
    pub symbol: String,
    pub notes: String,
    pub timestamp: u64,
    pub last_modified_by: ActorId,
}

// 记录每次交易对操作的事件日志
#[derive(Debug, Clone, Encode, Decode, TypeInfo)]
pub struct EventLog {
    pub id: String,
    pub timestamp: u64,
    pub account: ActorId,
    pub action: String,
    pub trading_pair_id: String,
    pub details: String,
}

// 定义所有可能的交易对操作
#[derive(Debug, Encode, Decode, TypeInfo)]
pub enum TradingPairAction {
    CreateTradingPair { symbol: String, notes: String },
    UpdateTradingPair { id: String, symbol: String, notes: String },
    DeleteTradingPair { id: String },
    QueryTradingPair { id: String },
    QueryAllTradingPairs,
    QueryTradingPairCount,
    QueryEventLog { id: String },
    QueryAllEventLogs,
    QueryEventCount,
    QueryEditors,
    QueryEditorCount,
    AddEditor { editor: ActorId },
    RemoveEditor { editor: ActorId },
}

// 定义所有可能的回复消息
#[derive(Debug, Encode, Decode, TypeInfo)]
pub enum TradingPairReply {
    TradingPairCreated,
    TradingPairUpdated,
    TradingPairDeleted,
    TradingPairInfo(String),
    AllTradingPairs(String),
    TradingPairCount(String),
    EventLogInfo(String),
    AllEventLogs(String),
    EventCount(String),
    AllEditors(String),
    EditorCount(String),
    EditorAdded,
    EditorRemoved,
}

// 定义合约的元数据
pub struct TradingPairMetadata;

impl Metadata for TradingPairMetadata {
    type Init = In<TmgInit>;
    type Handle = InOut<TradingPairAction, TradingPairReply>;
    type State = Out<TradingPairState>;
    type Signal = ();
    type Reply = ();
    type Others = ();
}
