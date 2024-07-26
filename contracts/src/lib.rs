#![no_std]

use gstd::{msg, prelude::*, collections::BTreeMap}; // 引入gstd库中的必要模块
use tpio::*; // 引入 tpio 库中的所有内容

static mut STATE: Option<TradingPairState> = None; // 使用静态可变变量来存储状态

#[no_mangle]
extern "C" fn init() {
    // 初始化函数，用于设置合约的初始状态
    let _init_data: TmgInit = msg::load().expect("Unable to decode TmgInit"); // 从消息中加载初始化数据

    let state = TradingPairState {
        owner: msg::source(), // 设置拥有者为消息的发送者
        trading_pairs: BTreeMap::new(), // 初始化一个空的交易对集合
    };

    unsafe {
        STATE = Some(state); // 存储初始状态
    }
}

#[no_mangle]
extern "C" fn handle() {
    // 处理函数，用于处理不同的交易对操作
    let action: TradingPairAction = msg::load().expect("Unable to decode TradingPairAction"); // 从消息中加载交易对操作

    let state = unsafe {
        STATE.as_mut().expect("State is not initialized") // 获取当前状态
    };

    match action {
        TradingPairAction::CreateTradingPair(id, description) => {
            // 处理创建交易对操作
            let trading_pair = TradingPair { id, description }; // 创建新的交易对
            state.trading_pairs.insert(id, trading_pair); // 将新交易对插入集合
            msg::reply(TradingPairReply::TradingPairCreated, 0).expect("Unable to reply"); // 回复交易对创建成功消息
        }
        TradingPairAction::UpdateTradingPair(id, description) => {
            // 处理更新交易对操作
            if let Some(trading_pair) = state.trading_pairs.get_mut(&id) {
                // 如果交易对存在，则更新描述
                trading_pair.description = description;
                msg::reply(TradingPairReply::TradingPairUpdated, 0).expect("Unable to reply"); // 回复交易对更新成功消息
            } else {
                // 如果交易对不存在，回复交易对信息为空
                msg::reply(TradingPairReply::TradingPairInfo(None), 0).expect("Unable to reply");
            }
        }
        TradingPairAction::DeleteTradingPair(id) => {
            // 处理删除交易对操作
            state.trading_pairs.remove(&id); // 从集合中移除交易对
            msg::reply(TradingPairReply::TradingPairDeleted, 0).expect("Unable to reply"); // 回复交易对删除成功消息
        }
        TradingPairAction::QueryTradingPair(id) => {
            // 处理查询单个交易对操作
            let trading_pair = state.trading_pairs.get(&id).cloned(); // 获取交易对信息
            msg::reply(TradingPairReply::TradingPairInfo(trading_pair), 0).expect("Unable to reply"); // 回复交易对信息
        }
        TradingPairAction::QueryAllTradingPairs => {
            // 处理查询所有交易对操作
            let all_trading_pairs: Vec<_> = state.trading_pairs.values().cloned().collect(); // 获取所有交易对
            msg::reply(TradingPairReply::AllTradingPairs(all_trading_pairs), 0).expect("Unable to reply"); // 回复所有交易对信息
        }
        TradingPairAction::QueryTradingPairCount => {
            // 处理查询交易对数量操作
            let count = state.trading_pairs.len() as u64; // 获取交易对数量
            msg::reply(TradingPairReply::TradingPairCount(count), 0).expect("Unable to reply"); // 回复交易对数量
        }
    }
}

#[no_mangle]
extern "C" fn state() {
    // 状态查询函数，用于获取当前状态
    let state = unsafe {
        STATE.as_ref().expect("State is not initialized") // 获取当前状态
    };
    msg::reply(state, 0).expect("Unable to reply"); // 回复当前状态信息
}
