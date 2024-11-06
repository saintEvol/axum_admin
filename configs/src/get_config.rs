use std::{io::Read};

use once_cell::sync::Lazy;

use super::cfgs::Configs;
use parking_lot::Mutex;


/// 外部代码设置的配置，库将使用此配置作为整个库的配置
static SETTING_CFG: Mutex<Option<Configs>> = Mutex::new(None);

// const CFG_FILE: &str = "config/config.toml";
//  只要是配置文件中的配置项，都可以通过这个结构体来获取，
// 只要读取一次值后保存到内存，一直可供使用
pub static CFG: Lazy<Configs> = Lazy::new(self::Configs::init);

impl Configs {

    /// 设置配置，必须调用并且只能调用一次
    pub fn set_config(config: Configs) {
        let mut guard = SETTING_CFG.lock();
        if guard.is_some() {
            panic!("必须设置且仅能设置一次配置")
        }
        guard.replace(config);

    }

    /// 从外部设置的配置中获取配置
    fn init() -> Self {

        let mut setting_config = SETTING_CFG.lock();
        match setting_config.take() {
            None => {
                panic!("启动框架前，请先设置好配置数据!")
            }
            Some(config) => {
                config
            }
        }


        // let mut file = match File::open(CFG_FILE) {
        //     Ok(f) => f,
        //     Err(e) => panic!("不存在配置文件：{}，错误信息：{}", CFG_FILE, e),
        // };
        // let mut cfg_contents = String::new();
        // match file.read_to_string(&mut cfg_contents) {
        //     Ok(s) => s,
        //     Err(e) => panic!("读取配置文件失败，错误信息：{}", e),
        // };
        // toml::from_str(&cfg_contents).expect("解析配置文件错误")
    }
}
