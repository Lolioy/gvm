use anyhow::{anyhow, Result};
use tauri::{App, AppHandle, CustomMenuItem, Manager, SystemTrayEvent, SystemTrayHandle, SystemTrayMenu, SystemTrayMenuItem, SystemTraySubmenu};

use crate::version::*;

pub const MENU_ID_QUIT: &str = "quit";
pub const MENU_ID_CURRENT_VERSION: &str = "current_version";

pub fn setup(app: &mut App) -> Result<()> {
    let tray_handle = app.app_handle().tray_handle();
    tray_handle.set_menu(
        SystemTrayMenu::new()
            .add_submenu(SystemTraySubmenu::new("本地版本", get_menu_local_versions()?))
            .add_submenu(SystemTraySubmenu::new("更多版本", get_menu_more_versions()?))
            .add_native_item(SystemTrayMenuItem::Separator)
            .add_item(get_menu_current_version()?)
            .add_native_item(SystemTrayMenuItem::Separator)
            .add_item(get_menu_quit())
    )?;
    change_tooltip(&tray_handle, "")?;
    Ok(())
}

fn get_menu_quit() -> CustomMenuItem {
    CustomMenuItem::new(MENU_ID_QUIT, "退出")
}

fn get_menu_current_version() -> Result<CustomMenuItem> {
    let current_version = get_current_version()?.unwrap_or("None".to_string());
    Ok(CustomMenuItem::new(MENU_ID_CURRENT_VERSION, current_version).disabled())
}

fn get_menu_local_versions() -> Result<SystemTrayMenu> {
    let current_version = &get_current_version()?;
    let mut menu = SystemTrayMenu::new();
    let versions: Vec<CustomMenuItem> = get_local_versions()
        .iter()
        .map(|v| CustomMenuItem::new(gen_version_menu_id(LOCAL_VERSION_PREFIX, v.as_str()), v.as_str()))
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

fn get_menu_more_versions() -> Result<SystemTrayMenu> {
    let current_version = &get_current_version()?;
    let mut menu = SystemTrayMenu::new();
    let versions: Vec<CustomMenuItem> = get_more_versions()
        .iter()
        .map(|v| CustomMenuItem::new(gen_version_menu_id(MORE_VERSION_PREFIX, v.as_str()), v.as_str()))
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

fn gen_version_menu_id(prefix: &str, version: &str) -> String {
    format!("{prefix}{VERSION_TAG}{version}")
}

fn get_version_from_menu_id(menu_id: &str) -> Option<String> {
    if let Some(idx) = menu_id.find(VERSION_TAG) {
        let version = &menu_id[idx + VERSION_TAG.len()..];
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
    // 更新菜单: 本地版本
    for v in get_local_versions() {
        let menu_item = tray_handle.get_item(gen_version_menu_id(LOCAL_VERSION_PREFIX, v.as_str()).as_str());
        menu_item.set_enabled(v.as_str() != current_version)?;
        menu_item.set_selected(v.as_str() == current_version)?;
    }

    // 更新菜单: 本地版本
    for v in get_more_versions() {
        let menu_item = tray_handle.get_item(gen_version_menu_id(MORE_VERSION_PREFIX, v.as_str()).as_str());
        menu_item.set_enabled(v.as_str() != current_version)?;
        menu_item.set_selected(v.as_str() == current_version)?;
    }

    // 更新菜单: 当前版本
    tray_handle.get_item(MENU_ID_CURRENT_VERSION).set_title(current_version)?;

    // 更新: tooltip
    tray_handle.set_tooltip(&format!("Current Version {current_version}"))?;

    Ok(())
}

fn change_tooltip(tray_handle: &SystemTrayHandle, tooltip: &str) -> Result<()> {
    let tooltip = if tooltip.is_empty() { get_current_version()?.unwrap_or("None".to_string()) } else { tooltip.to_string() };
    // 更新: tooltip
    tray_handle.set_tooltip(&format!("当前版本: {tooltip}"))?;
    Ok(())
}

pub fn on_menu_event(app_handle: &AppHandle, event: SystemTrayEvent) {
    match event {
        SystemTrayEvent::MenuItemClick { id, .. } => {
            if id == MENU_ID_QUIT {
                std::process::exit(0);
            } else if id.contains(VERSION_TAG) {
                if let Err(e) = change_version_menu(app_handle, id.as_str()) {
                    eprintln!("change version failed: {}", e);
                }
            }
        }
        _ => {}
    };
}