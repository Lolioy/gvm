use anyhow::{anyhow, Result};
use tauri::{App, AppHandle, CustomMenuItem, Manager, SystemTrayEvent, SystemTrayHandle, SystemTrayMenu, SystemTrayMenuItem, SystemTraySubmenu};

use crate::version::*;

// 退出
pub const MENU_ID_QUIT: &str = "quit";
pub const MENU_TITLE_QUIT: &str = "退出";
// 当前版本
pub const MENU_ID_CURRENT_VERSION: &str = "current_version";
// 版本列表
pub const MENU_ID_VERSION_PREFIX: &str = "version_";
pub const MENU_TITLE_CURRENT_VERSION: &str = "版本列表";

pub fn setup(app: &mut App) -> Result<()> {
    let tray_handle = app.app_handle().tray_handle();
    tray_handle.set_menu(
        SystemTrayMenu::new()
            .add_submenu(SystemTraySubmenu::new(MENU_TITLE_CURRENT_VERSION, get_version_list_menu()?))
            .add_native_item(SystemTrayMenuItem::Separator)
            .add_item(CustomMenuItem::new(MENU_ID_CURRENT_VERSION,
                                          get_current_version()?.unwrap_or("None".to_string())).disabled())
            .add_native_item(SystemTrayMenuItem::Separator)
            .add_item(CustomMenuItem::new(MENU_ID_QUIT, MENU_TITLE_QUIT))
    )?;
    change_tooltip(&tray_handle, "")?;
    Ok(())
}

fn get_version_list_menu() -> Result<SystemTrayMenu> {
    let current_version = &get_current_version()?;
    let mut menu = SystemTrayMenu::new();
    let versions: Vec<CustomMenuItem> = get_version_list()
        .iter()
        .map(|v| CustomMenuItem::new(gen_version_menu_id(v.as_str()), v.as_str()))
        .collect();
    for mut cm in versions {
        if let Some(current_version) = current_version {
            if cm.id_str.contains(current_version) {
                cm = cm.selected().disabled();
            }
        }
        menu = menu.add_item(cm);
    }
    Ok(menu)
}

fn gen_version_menu_id(version: &str) -> String {
    format!("{MENU_ID_VERSION_PREFIX}{version}")
}

fn get_version_from_menu_id(menu_id: &str) -> Option<String> {
    if let Some(idx) = menu_id.find(MENU_ID_VERSION_PREFIX) {
        let version = &menu_id[idx + MENU_ID_VERSION_PREFIX.len()..];
        return Some(version.to_string());
    }
    None
}

fn change_version_menu(app_handle: &AppHandle, menu_id: &str) -> Result<()> {
    let current_version = &match get_version_from_menu_id(menu_id) {
        None => {
            return Err(anyhow!("parse `{menu_id}` to version failed"));
        }
        Some(version) => {
            // 保存版本
            set_current_version(version.as_str())?;
            version
        }
    };
    let tray_handle = &app_handle.tray_handle();
    // 更新菜单: 版本列表
    for v in get_version_list() {
        let menu_item = tray_handle.get_item(gen_version_menu_id(v.as_str()).as_str());
        menu_item.set_enabled(v.as_str() != current_version)?;
        menu_item.set_selected(v.as_str() == current_version)?;
    }
    // 更新菜单: 当前版本
    tray_handle.get_item(MENU_ID_CURRENT_VERSION).set_title(current_version)?;
    // 更新: tooltip
    change_tooltip(&tray_handle, current_version)?;
    Ok(())
}

fn change_tooltip(tray_handle: &SystemTrayHandle, tooltip: &str) -> Result<()> {
    let tooltip = if tooltip.is_empty() { get_current_version()?.unwrap_or("".to_string()) } else { tooltip.to_string() };
    // 更新: tooltip
    tray_handle.set_tooltip(&format!("当前版本: {tooltip}"))?;
    Ok(())
}

pub fn on_menu_event(app_handle: &AppHandle, event: SystemTrayEvent) {
    match event {
        SystemTrayEvent::MenuItemClick { id, .. } => {
            if id == MENU_ID_QUIT {
                std::process::exit(0);
            } else if id.contains(MENU_ID_VERSION_PREFIX) {
                if let Err(e) = change_version_menu(app_handle, id.as_str()) {
                    eprintln!("change version failed: {}", e);
                }
            }
        }
        _ => {}
    };
}