#![no_std] // 使用 no_std，表明不使用标准库

use gstd::{msg, prelude::*, ActorId};
use gstd::collections::{BTreeMap, BTreeSet};
use tpio::*;

// 全局状态静态变量
static mut STATE: Option<TradingPairState> = None;

#[no_mangle]
extern "C" fn init() {
    // 从消息中加载初始化参数
    let TmgInit { owner } = msg::load().expect("Unable to decode TmgInit");

    // 初始化合约状态
    let mut state = TradingPairState {
        owner,
        trading_pairs: BTreeMap::new(),
        counter: 0,
        authorized_editors: BTreeSet::new(),
        event_logs: vec![],
        event_counter: 0,
        editor_counter: 0,
    };

    // 将合约拥有者添加到授权编辑者列表中
    state.authorized_editors.insert(owner);
    state.editor_counter += 1;

    // 存储合约状态
    unsafe { STATE = Some(state) };

    // 回复初始化成功信息
    msg::reply("Initialization Successful", 0).expect("Failed to send reply");
}

#[no_mangle]
extern "C" fn handle() {
    // 从消息中加载操作
    let action: TradingPairAction = msg::load().expect("Unable to decode TradingPairAction");
    let caller = msg::source();
    let state = unsafe { STATE.as_mut().expect("State is not initialized") };

    match action {
        TradingPairAction::CreateTradingPair { symbol, notes } => {
            // 验证权限
            if !state.authorized_editors.contains(&caller) {
                panic!("Unauthorized");
            }
            // 生成新的交易对ID
            let id = format!("TradingPairID-{}", state.counter);
            let timestamp = gstd::exec::block_timestamp();
            // 创建新的交易对
            let trading_pair = TradingPair {
                id: id.clone(),
                symbol,
                notes,
                timestamp,
                last_modified_by: caller,
            };
            // 存储交易对信息
            state.trading_pairs.insert(id.clone(), trading_pair);
            state.counter += 1;
            // 创建并存储事件日志
            state.event_counter += 1;
            let event_log = EventLog {
                id: format!("EventLogID-{}", state.event_counter),
                timestamp,
                account: caller,
                action: "Create".to_string(),
                trading_pair_id: id.clone(),
                details: "Created new trading pair".to_string(),
            };
            state.event_logs.push(event_log);
            // 回复创建成功信息
            msg::reply(TradingPairReply::TradingPairCreated, 0).expect("Failed to send reply");
        },
        TradingPairAction::UpdateTradingPair { id, symbol, notes } => {
            // 验证权限
            if !state.authorized_editors.contains(&caller) {
                panic!("Unauthorized");
            }
            // 更新现有交易对信息
            if let Some(trading_pair) = state.trading_pairs.get_mut(&id) {
                trading_pair.symbol = symbol.clone();
                trading_pair.notes = notes.clone();
                trading_pair.timestamp = gstd::exec::block_timestamp();
                trading_pair.last_modified_by = caller;
                // 创建并存储事件日志
                state.event_counter += 1;
                let event_log = EventLog {
                    id: format!("EventLogID-{}", state.event_counter),
                    timestamp: trading_pair.timestamp,
                    account: caller,
                    action: "Update".to_string(),
                    trading_pair_id: id.clone(),
                    details: "Updated trading pair".to_string(),
                };
                state.event_logs.push(event_log);
                // 回复更新成功信息
                msg::reply(TradingPairReply::TradingPairUpdated, 0).expect("Failed to send reply");
            } else {
                // 回复交易对未找到信息
                msg::reply("TradingPair not found", 0).expect("Failed to send reply");
            }
        },
        TradingPairAction::DeleteTradingPair { id } => {
            // 验证权限
            if !state.authorized_editors.contains(&caller) {
                panic!("Unauthorized");
            }
            // 删除现有交易对信息
            if state.trading_pairs.remove(&id).is_some() {
                state.counter -= 1;
                // 创建并存储事件日志
                state.event_counter += 1;
                let event_log = EventLog {
                    id: format!("EventLogID-{}", state.event_counter),
                    timestamp: gstd::exec::block_timestamp(),
                    account: caller,
                    action: "Delete".to_string(),
                    trading_pair_id: id.clone(),
                    details: "Deleted trading pair".to_string(),
                };
                state.event_logs.push(event_log);
                // 回复删除成功信息
                msg::reply(TradingPairReply::TradingPairDeleted, 0).expect("Failed to send reply");
            } else {
                // 回复交易对未找到信息
                msg::reply("TradingPair not found", 0).expect("Failed to send reply");
            }
        },
        TradingPairAction::AddEditor { editor } => {
            // 验证是否为合约拥有者
            if caller != state.owner {
                panic!("Only owner can add editors");
            }
            // 添加编辑权限地址
            if state.authorized_editors.insert(editor) {
                state.editor_counter += 1;
                // 回复添加成功信息
                msg::reply(TradingPairReply::EditorAdded, 0).expect("Failed to send reply");
            } else {
                // 回复编辑者已存在信息
                msg::reply("Editor already exists", 0).expect("Failed to send reply");
            }
        },
        TradingPairAction::RemoveEditor { editor } => {
            // 验证是否为合约拥有者
            if caller != state.owner {
                panic!("Only owner can remove editors");
            }
            // 移除编辑权限地址
            if state.authorized_editors.remove(&editor) {
                state.editor_counter -= 1;
                // 回复移除成功信息
                msg::reply(TradingPairReply::EditorRemoved, 0).expect("Failed to send reply");
            } else {
                // 回复编辑者未找到信息
                msg::reply("Editor not found", 0).expect("Failed to send reply");
            }
        },
        TradingPairAction::QueryTradingPair { id } => {
            // 查询特定交易对
            if let Some(trading_pair) = state.trading_pairs.get(&id) {
                msg::reply(TradingPairReply::TradingPairInfo(format!("{:?}", trading_pair)), 0).expect("Failed to send reply");
            } else {
                // 回复交易对未找到信息
                msg::reply("TradingPair not found", 0).expect("Failed to send reply");
            }
        },
        TradingPairAction::QueryAllTradingPairs => {
            // 查询所有交易对
            let all_trading_pairs = format!("{:?}", state.trading_pairs);
            msg::reply(TradingPairReply::AllTradingPairs(all_trading_pairs), 0).expect("Failed to send reply");
        },
        TradingPairAction::QueryTradingPairCount => {
            // 查询交易对数量
            msg::reply(TradingPairReply::TradingPairCount(format!("{}", state.counter)), 0).expect("Failed to send reply");
        },
        TradingPairAction::QueryEventLog { id } => {
            // 查询特定事件日志
            if let Some(event_log) = state.event_logs.iter().find(|log| log.id == id) {
                msg::reply(TradingPairReply::EventLogInfo(format!("{:?}", event_log)), 0).expect("Failed to send reply");
            } else {
                // 回复事件日志未找到信息
                msg::reply("EventLog not found", 0).expect("Failed to send reply");
            }
        },
        TradingPairAction::QueryAllEventLogs => {
            // 查询所有事件日志
            let all_event_logs = format!("{:?}", state.event_logs);
            msg::reply(TradingPairReply::AllEventLogs(all_event_logs), 0).expect("Failed to send reply");
        },
        TradingPairAction::QueryEventCount => {
            // 查询事件日志数量
            msg::reply(TradingPairReply::EventCount(format!("{}", state.event_counter)), 0).expect("Failed to send reply");
        },
        TradingPairAction::QueryEditors => {
            // 查询所有编辑者
            let all_editors = format!("{:?}", state.authorized_editors);
            msg::reply(TradingPairReply::AllEditors(all_editors), 0).expect("Failed to send reply");
        },
        TradingPairAction::QueryEditorCount => {
            // 查询编辑者数量
            msg::reply(TradingPairReply::EditorCount(format!("{}", state.editor_counter)), 0).expect("Failed to send reply");
        },
    }
}

#[no_mangle]
extern "C" fn state() {
    // 从消息中加载操作
    let action: TradingPairAction = msg::load().expect("Unable to decode TradingPairAction");
    let state = unsafe { STATE.as_mut().expect("State is not initialized") };

    match action {
        TradingPairAction::QueryTradingPair { id } => {
            // 查询特定交易对
            if let Some(trading_pair) = state.trading_pairs.get(&id) {
                msg::reply(TradingPairReply::TradingPairInfo(format!("{:?}", trading_pair)), 0).expect("Failed to send reply");
            } else {
                // 回复交易对未找到信息
                msg::reply("TradingPair not found", 0).expect("Failed to send reply");
            }
        },
        TradingPairAction::QueryAllTradingPairs => {
            // 查询所有交易对
            let all_trading_pairs = format!("{:?}", state.trading_pairs);
            msg::reply(TradingPairReply::AllTradingPairs(all_trading_pairs), 0).expect("Failed to send reply");
        },
        TradingPairAction::QueryTradingPairCount => {
            // 查询交易对数量
            msg::reply(TradingPairReply::TradingPairCount(format!("{}", state.counter)), 0).expect("Failed to send reply");
        },
        TradingPairAction::QueryEventLog { id } => {
            // 查询特定事件日志
            if let Some(event_log) = state.event_logs.iter().find(|log| log.id == id) {
                msg::reply(TradingPairReply::EventLogInfo(format!("{:?}", event_log)), 0).expect("Failed to send reply");
            } else {
                // 回复事件日志未找到信息
                msg::reply("EventLog not found", 0).expect("Failed to send reply");
            }
        },
        TradingPairAction::QueryAllEventLogs => {
            // 查询所有事件日志
            let all_event_logs = format!("{:?}", state.event_logs);
            msg::reply(TradingPairReply::AllEventLogs(all_event_logs), 0).expect("Failed to send reply");
        },
        TradingPairAction::QueryEventCount => {
            // 查询事件日志数量
            msg::reply(TradingPairReply::EventCount(format!("{}", state.event_counter)), 0).expect("Failed to send reply");
        },
        TradingPairAction::QueryEditors => {
            // 查询所有编辑者
            let all_editors = format!("{:?}", state.authorized_editors);
            msg::reply(TradingPairReply::AllEditors(all_editors), 0).expect("Failed to send reply");
        },
        TradingPairAction::QueryEditorCount => {
            // 查询编辑者数量
            msg::reply(TradingPairReply::EditorCount(format!("{}", state.editor_counter)), 0).expect("Failed to send reply");
        },
        _ => {
            // 回复未知查询操作信息
            msg::reply("Unknown query action", 0).expect("Failed to send reply");
        },
    }
}
