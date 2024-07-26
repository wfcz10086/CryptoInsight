#![no_std]

use codec::{Decode, Encode}; // 引入 codec 库用于序列化和反序列化
use gmeta::{In, InOut, Metadata, Out}; // 引入 gmeta 库中的相关类型
use scale_info::TypeInfo; // 引入 scale_info 库用于类型信息
use gstd::{prelude::*, ActorId}; // 引入 gstd 库中的预置模块和 ActorId 类型
use gstd::collections::BTreeMap; // 引入 gstd 库中的 BTreeMap 集合类型

pub type TradingPairId = u64; // 定义 TradingPairId 类型为 u64

pub struct ProgramMetadata; // 定义 ProgramMetadata 结构体

impl Metadata for ProgramMetadata {
    type Init = In<TmgInit>; // 初始化类型
    type Handle = InOut<TradingPairAction, TradingPairReply>; // 处理类型
    type Reply = (); // 回复类型
    type Others = (); // 其他类型
    type Signal = (); // 信号类型
    type State = Out<TradingPairState>; // 状态类型
}

#[derive(Debug, Clone, Encode, Decode, TypeInfo)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub struct TmgInit {
    pub name: String, // 初始化时的名称
}

#[derive(Encode, Decode, TypeInfo, Debug, PartialEq, Eq)] // 添加 PartialEq 和 Eq 派生
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub enum TradingPairAction {
    CreateTradingPair(TradingPairId, String), // 创建交易对
    UpdateTradingPair(TradingPairId, String), // 更新交易对
    DeleteTradingPair(TradingPairId), // 删除交易对
    QueryTradingPair(TradingPairId), // 查询交易对
    QueryAllTradingPairs, // 查询所有交易对
    QueryTradingPairCount, // 查询交易对数量
}

#[derive(Encode, Debug, PartialEq, Eq, Decode, TypeInfo)] // 添加 PartialEq 和 Eq 派生
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub enum TradingPairReply {
    TradingPairCreated, // 交易对创建成功
    TradingPairUpdated, // 交易对更新成功
    TradingPairDeleted, // 交易对删除成功
    TradingPairInfo(Option<TradingPair>), // 交易对信息
    AllTradingPairs(Vec<TradingPair>), // 所有交易对
    TradingPairCount(u64), // 交易对数量
}

#[derive(Debug, Clone, Encode, Decode, TypeInfo, PartialEq, Eq)] // 添加 PartialEq 和 Eq 派生
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub struct TradingPair {
    pub id: TradingPairId, // 交易对 ID
    pub description: String, // 交易对描述
}

#[derive(Default, Encode, Decode, TypeInfo)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub struct TradingPairState {
    pub owner: ActorId, // 拥有者的 ActorId
    pub trading_pairs: BTreeMap<TradingPairId, TradingPair>, // 交易对集合
}
