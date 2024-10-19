// api 模块
mod system; // 系统模块
mod test; // 测试模块

//  路由模块
mod route;

//  重新导出
pub use route::api;
pub use route::set_auth_middleware;
