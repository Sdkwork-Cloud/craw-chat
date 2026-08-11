//! Principal Directory Contract - IM principal 目录访问控制契约
//!
//! 目录回答「该 tenant 下的这个 principal 是否可用 IM」：网关完成 IAM
//! token 认证后，会话服务在进入业务处理前用目录做二次校验。静态目录
//! （catalog 白名单）与动态目录（PostgreSQL 认证即注册）都是本契约的实现。

/// Principal 目录校验错误。
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum PrincipalDirectoryError {
    PrincipalNotFound {
        tenant_id: String,
        principal_id: String,
        principal_kind: String,
    },
    PrincipalDisabled {
        tenant_id: String,
        principal_id: String,
        principal_kind: String,
    },
    Unavailable(String),
}

/// Principal 目录：校验调用方是否为租户内已知且 active 的 principal。
pub trait PrincipalDirectory: Send + Sync {
    fn ensure_active_principal(
        &self,
        tenant_id: &str,
        principal_id: &str,
        principal_kind: &str,
    ) -> Result<(), PrincipalDirectoryError>;
}
