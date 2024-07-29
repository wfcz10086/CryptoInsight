#![no_std]

use gmeta::{In, InOut, Metadata, Out};
use parity_scale_codec::{Decode, Encode};
use scale_info::TypeInfo;
use gstd::prelude::*;
use gstd::collections::{BTreeMap, BTreeSet};
use gstd::ActorId;

pub type TradingPairId = u64;
pub type EventLogId = u64;

pub struct TradingPairMetadata;

impl Metadata for TradingPairMetadata {
    type Init = In<TmgInit>;
    type Handle = InOut<TradingPairAction, TradingPairReply>;
    type State = Out<TradingPairState>;
    type Signal = ();
    type Reply = ();
    type Others = ();
}

#[derive(Debug, Clone, Encode, Decode, TypeInfo)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub struct TmgInit {
    pub name: String,
    pub owner: ActorId,
}

#[derive(Encode, Decode, TypeInfo, Debug, PartialEq, Eq)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub enum TradingPairAction {
    CreateTradingPair(String, String),
    UpdateTradingPair(TradingPairId, String, String),
    DeleteTradingPair(TradingPairId),
    QueryTradingPair(TradingPairId),
    QueryAllTradingPairs,
    QueryTradingPairCount,
    QueryEventLogs,
    QueryEventCount,
    AddEditor(ActorId),
    RemoveEditor(ActorId),
}

#[derive(Encode, Debug, PartialEq, Eq, Decode, TypeInfo)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub enum TradingPairReply {
    TradingPairCreated,
    TradingPairUpdated,
    TradingPairDeleted,
    TradingPairInfo(Option<TradingPair>),
    AllTradingPairs(Vec<TradingPair>),
    TradingPairCount(u64),
    EventLogs(Vec<EventLog>),
    EventCount(u64),
    EditorAdded,
    EditorRemoved,
}

#[derive(Debug, Clone, Encode, Decode, TypeInfo, PartialEq, Eq)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub struct TradingPair {
    pub id: TradingPairId,
    pub symbol: String,
    pub notes: String,
    pub timestamp: u64,
    pub last_modified_by: ActorId,
}

#[derive(Debug, Clone, Encode, Decode, TypeInfo, PartialEq, Eq)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub struct EventLog {
    pub id: EventLogId,
    pub timestamp: u64,
    pub account: ActorId,
    pub action: String,
    pub trading_pair_id: TradingPairId,
    pub details: String,
}

#[derive(Encode, Decode, TypeInfo)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub struct TradingPairState {
    pub owner: ActorId,
    pub trading_pairs: BTreeMap<TradingPairId, TradingPair>,
    pub authorized_editors: BTreeSet<ActorId>,
    pub counter: u64,
    pub event_logs: Vec<EventLog>,
    pub event_counter: u64,
    pub editor_counter: u64,
}

impl TradingPairState {
    pub fn new(owner: ActorId) -> Self {
        TradingPairState {
            owner,
            trading_pairs: BTreeMap::new(),
            authorized_editors: BTreeSet::new(),
            counter: 0,
            event_logs: Vec::new(),
            event_counter: 0,
            editor_counter: 0,
        }
    }

    pub fn is_owner_or_editor(&self, account: ActorId) -> bool {
        self.owner == account || self.authorized_editors.contains(&account)
    }
}
