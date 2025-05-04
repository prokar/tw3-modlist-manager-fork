#[cfg(not(debug_assertions))]
pub const MODLIST_DATABASE_PATH: &str = ".";
#[cfg(debug_assertions)]
pub const MODLIST_DATABASE_PATH: &str = r#"D:\SteamLibrary\steamapps\common\The Witcher 3\modlists"#;
#[cfg(not(debug_assertions))]
pub const WITCHER_GAME_ROOT: &str = r#"..\"#;
#[cfg(debug_assertions)]
pub const WITCHER_GAME_ROOT: &str = r"D:\SteamLibrary\steamapps\common\The Witcher 3";
#[cfg(not(debug_assertions))]
pub const SCRIPTMERGER_PATH: &str = r"..\scriptmerger";
#[cfg(debug_assertions)]
pub const SCRIPTMERGER_PATH: &str = r#"D:\SteamLibrary\steamapps\common\The Witcher 3\scriptmerger"#;

pub const TW3SCRIPTMERGER_PATH: &str = "tw3-script-merger.exe";

pub const SCRIPTMERGER_EXE_NAME: &str = "WitcherScriptMerger.exe";

pub const MODLIST_CONFIG_NAME: &str = "modlist.toml";

pub const MODLIST_MERGEINVENTORY_PATH: &str = "MergeInventory.xml";

pub const MODLIST_MERGEDBUNDLES_PATH: &str = "mergedbundles";

pub const SCRIPTMERGER_MERGEDFILES_FOLDERNAME: &str = "mod0000_MergedFiles";

pub const SCRIPTMERGER_MERGEDBUNDLES_PATH: &str = r#"Merged Bundle Content"#;

pub const WITCHER_SAVES: &str = r#"The Witcher 3"#;

pub const MODMANAGER_PATH: &str = r#"The Witcher 3 Mod Manager"#;