#![no_std] // 使用 no_std，表明不使用标准库

use tpio::*;
use parity_scale_codec::{Decode, Encode}; // 使用 parity_scale_codec 库用于序列化和反序列化
use gmeta::{In, InOut, Metadata, Out}; // 引入 gmeta 库中的相关类型
use scale_info::TypeInfo; // 引入 scale_info 库用于类型信息
use gstd::{msg, prelude::*, ActorId}; // 引入 gstd 库中的预置模块和 ActorId 类型
use gstd::collections::{BTreeMap, BTreeSet}; // 引入 gstd 库中的 BTreeMap 和 BTreeSet 集合类型

// 定义静态变量 STATE 来存储合约状态
static mut STATE: Option<TradingPairState> = None;

// 初始化函数，使用 async 初始化
#[gstd::async_init]
async fn init() {
    // 从初始化消息中加载初始化参数
    let tmg_init: TmgInit = msg::load().expect("Unable to load TmgInit");
    // 创建新的状态实例，并设置合约拥有者
    let state = TradingPairState::new(tmg_init.owner);
    unsafe {
        // 将状态存储在静态变量中
        STATE = Some(state);
    }
    // 回复初始化成功信息
    msg::reply("Initialized", 0).expect("Failed to reply in init");
}

// 消息处理函数
#[no_mangle]
extern "C" fn handle() {
    // 加载传入的操作
    let action: TradingPairAction = msg::load().expect("Unable to load TradingPairAction");

    // 获取当前状态
    let state = unsafe { STATE.as_mut().expect("State is not initialized") };

    // 获取消息的发送者地址
    let caller = msg::source();

    // 检查是否为合约拥有者进行权限管理操作
    match action {
        TradingPairAction::AddEditor(editor) | TradingPairAction::RemoveEditor(editor) => {
            if caller != state.owner {
                // 回复无权限信息
                msg::reply("Unauthorized", 0).expect("Failed to reply unauthorized");
                return;
            }
        },
        _ => {
            // 检查是否为授权编辑者进行交易对操作
            if !state.is_owner_or_editor(caller) {
                // 回复无权限信息
                msg::reply("Unauthorized", 0).expect("Failed to reply unauthorized");
                return;
            }
        }
    }

    // 处理不同的操作类型
    match action {
        TradingPairAction::CreateTradingPair(symbol, notes) => {
            // 自增计数器
            state.counter += 1;
            // 创建新的交易对
            let trading_pair = TradingPair {
                id: state.counter,
                symbol: symbol.clone(),
                notes: notes.clone(),
                timestamp: gstd::exec::block_timestamp(),
                last_modified_by: caller,
            };
            // 存储交易对信息
            state.trading_pairs.insert(state.counter, trading_pair);

            // 创建事件日志
            let event_log = EventLog {
                id: state.event_counter,
                timestamp: gstd::exec::block_timestamp(),
                account: caller,
                action: "Create".into(),
                trading_pair_id: state.counter,
                details: format!("Created trading pair with symbol: {}", symbol),
            };
            // 存储事件日志
            state.event_logs.push(event_log);
            // 自增事件计数器
            state.event_counter += 1;

            // 回复创建成功信息
            msg::reply(TradingPairReply::TradingPairCreated, 0).expect("Failed to reply TradingPairCreated");
        },
        TradingPairAction::UpdateTradingPair(id, symbol, notes) => {
            // 获取并更新交易对信息
            if let Some(trading_pair) = state.trading_pairs.get_mut(&id) {
                trading_pair.symbol = symbol.clone();
                trading_pair.notes = notes.clone();
                trading_pair.timestamp = gstd::exec::block_timestamp();
                trading_pair.last_modified_by = caller;

                // 创建事件日志
                let event_log = EventLog {
                    id: state.event_counter,
                    timestamp: gstd::exec::block_timestamp(),
                    account: caller,
                    action: "Update".into(),
                    trading_pair_id: id,
                    details: format!("Updated trading pair with id: {}", id),
                };
                // 存储事件日志
                state.event_logs.push(event_log);
                // 自增事件计数器
                state.event_counter += 1;

                // 回复更新成功信息
                msg::reply(TradingPairReply::TradingPairUpdated, 0).expect("Failed to reply TradingPairUpdated");
            } else {
                // 回复交易对未找到信息
                msg::reply("TradingPairNotFound", 0).expect("Failed to reply TradingPairNotFound");
            }
        },
        TradingPairAction::DeleteTradingPair(id) => {
            // 删除交易对信息
            if state.trading_pairs.remove(&id).is_some() {
                // 自减计数器
                state.counter -= 1;

                // 创建事件日志
                let event_log = EventLog {
                    id: state.event_counter,
                    timestamp: gstd::exec::block_timestamp(),
                    account: caller,
                    action: "Delete".into(),
                    trading_pair_id: id,
                    details: format!("Deleted trading pair with id: {}", id),
                };
                // 存储事件日志
                state.event_logs.push(event_log);
                // 自增事件计数器
                state.event_counter += 1;

                // 回复删除成功信息
                msg::reply(TradingPairReply::TradingPairDeleted, 0).expect("Failed to reply TradingPairDeleted");
            } else {
                // 回复交易对未找到信息
                msg::reply("TradingPairNotFound", 0).expect("Failed to reply TradingPairNotFound");
            }
        },
        TradingPairAction::QueryTradingPair(id) => {
            // 查询特定交易对信息
            let trading_pair = state.trading_pairs.get(&id).cloned();
            // 回复查询结果
            msg::reply(TradingPairReply::TradingPairInfo(trading_pair), 0).expect("Failed to reply TradingPairInfo");
        },
        TradingPairAction::QueryAllTradingPairs => {
            // 查询所有交易对信息
            let all_trading_pairs = state.trading_pairs.values().cloned().collect::<Vec<_>>();
            // 回复查询结果
            msg::reply(TradingPairReply::AllTradingPairs(all_trading_pairs), 0).expect("Failed to reply AllTradingPairs");
        },
        TradingPairAction::QueryTradingPairCount => {
            // 查询交易对数量
            let count = state.trading_pairs.len() as u64;
            // 回复查询结果
            msg::reply(TradingPairReply::TradingPairCount(count), 0).expect("Failed to reply TradingPairCount");
        },
        TradingPairAction::QueryEventLogs => {
            // 查询事件日志
            let event_logs = state.event_logs.clone();
            // 回复查询结果
            msg::reply(TradingPairReply::EventLogs(event_logs), 0).expect("Failed to reply EventLogs");
        },
        TradingPairAction::QueryEventCount => {
            // 查询事件日志数量
            let count = state.event_logs.len() as u64;
            // 回复查询结果
            msg::reply(TradingPairReply::EventCount(count), 0).expect("Failed to reply EventCount");
        },
        TradingPairAction::AddEditor(editor) => {
            // 添加编辑权限地址
            state.authorized_editors.insert(editor);
            // 回复添加成功信息
            msg::reply(TradingPairReply::EditorAdded, 0).expect("Failed to reply EditorAdded");
        },
        TradingPairAction::RemoveEditor(editor) => {
            // 移除编辑权限地址
            state.authorized_editors.remove(&editor);
            // 回复移除成功信息
            msg::reply(TradingPairReply::EditorRemoved, 0).expect("Failed to reply EditorRemoved");
        },
    }
}

// 状态查询函数
#[no_mangle]
extern "C" fn state() {
    // 获取当前状态
    let state = unsafe { STATE.as_ref().expect("State is not initialized") };
    // 回复当前状态
    msg::reply(state, 0).expect("Failed to reply with state");
}
