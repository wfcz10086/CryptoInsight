#![no_std]

use codec::{Decode, Encode}; // 引入 codec 库用于序列化和反序列化
use gmeta::{In, InOut, Metadata, Out}; // 引入 gmeta 库中的相关类型
use scale_info::TypeInfo; // 引入 scale_info 库用于类型信息
use gstd::{prelude::*, ActorId}; // 引入 gstd 库中的预置模块和 ActorId 类型
use gstd::collections::{BTreeMap, BTreeSet}; // 引入 gstd 库中的 BTreeMap 和 BTreeSet 集合类型

pub type TradingPairId = u64; // 定义 TradingPairId 类型为 u64
pub type EventLogId = u64; // 定义 EventLogId 类型为 u64

/// 定义 ProgramMetadata 结构体
pub struct ProgramMetadata;

impl Metadata for ProgramMetadata {
    type Init = In<TmgInit>; // 初始化类型
    type Handle = InOut<TradingPairAction, TradingPairReply>; // 处理类型
    type Reply = (); // 回复类型
    type Others = (); // 其他类型
    type Signal = (); // 信号类型
    type State = Out<TradingPairState>; // 状态类型
}

/// 初始化结构体
#[derive(Debug, Clone, Encode, Decode, TypeInfo)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub struct TmgInit {
    pub name: String, // 初始化时的名称
    pub owner: ActorId, // 初始化时的拥有者
}

/// 交易对操作的枚举类型
#[derive(Encode, Decode, TypeInfo, Debug, PartialEq, Eq)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub enum TradingPairAction {
    CreateTradingPair(String, String), // 创建交易对（包含交易对符号和备注）
    UpdateTradingPair(TradingPairId, String, String), // 更新交易对（包含交易对ID、交易对符号和备注）
    DeleteTradingPair(TradingPairId), // 删除交易对
    QueryTradingPair(TradingPairId), // 查询特定交易对
    QueryAllTradingPairs, // 查询所有交易对
    QueryTradingPairCount, // 查询交易对数量
    QueryEventLogs, // 查询事件日志
    QueryEventCount, // 查询事件总数
    AddEditor(ActorId), // 增加合法编辑者
    RemoveEditor(ActorId), // 移除合法编辑者
}

/// 交易对操作的响应类型
#[derive(Encode, Debug, PartialEq, Eq, Decode, TypeInfo)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub enum TradingPairReply {
    TradingPairCreated, // 交易对创建成功
    TradingPairUpdated, // 交易对更新成功
    TradingPairDeleted, // 交易对删除成功
    TradingPairInfo(Option<TradingPair>), // 交易对信息
    AllTradingPairs(Vec<TradingPair>), // 所有交易对
    TradingPairCount(u64), // 交易对数量
    EventLogs(Vec<EventLog>), // 事件日志
    EventCount(u64), // 事件总数
    EditorAdded, // 合法编辑者添加成功
    EditorRemoved, // 合法编辑者移除成功
}

/// 交易对的结构体
#[derive(Debug, Clone, Encode, Decode, TypeInfo, PartialEq, Eq)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub struct TradingPair {
    pub id: TradingPairId, // 交易对 ID
    pub symbol: String, // 交易对符号
    pub notes: String, // 备注
    pub timestamp: u64, // 最后变更时间戳
    pub last_modified_by: ActorId, // 最后变更账号
}

/// 事件日志的结构体
#[derive(Debug, Clone, Encode, Decode, TypeInfo, PartialEq, Eq)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub struct EventLog {
    pub id: EventLogId, // 事件ID，自增
    pub timestamp: u64, // 操作时间戳
    pub account: ActorId, // 操作账号
    pub action: String, // 操作类型（新增、修改、删除）
    pub trading_pair_id: TradingPairId, // 交易对ID
    pub details: String, // 操作详情
}

/// 交易对状态的结构体
#[derive(Encode, Decode, TypeInfo)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub struct TradingPairState {
    pub owner: ActorId, // 合约拥有者的账号
    pub trading_pairs: BTreeMap<TradingPairId, TradingPair>, // 交易对集合
    pub authorized_editors: BTreeSet<ActorId>, // 合法编辑者列表
    pub counter: u64, // 累加器，初始值为 0
    pub event_logs: Vec<EventLog>, // 事件日志
    pub event_counter: u64, // 事件ID自增计数器，初始值为 0
}

impl TradingPairState {
    pub fn new(owner: ActorId) -> Self {
        TradingPairState {
            owner, // 初始化合约拥有者
            trading_pairs: BTreeMap::new(), // 初始化为空集合
            authorized_editors: BTreeSet::new(), // 初始化为空集合
            counter: 0, // 初始计数器值为 0
            event_logs: Vec::new(), // 初始化为空向量
            event_counter: 0, // 初始事件计数器值为 0
        }
    }
}
