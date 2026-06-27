use axum::http::header;

use crate::context::{
    AppContext, build_dual_token_headers_for_context, local_service_app_context,
    local_service_app_context_from_env, resolve_app_context, split_scope,
};

pub trait DualTokenRequestBuilderExt {
    fn with_dual_token_context<I, S>(
        self,
        tenant_id: &str,
        user_id: &str,
        actor_kind: &str,
        device_id: Option<&str>,
        permission_scope: I,
    ) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>;

    fn with_dual_token_tenant<S>(self, tenant_id: S) -> Self
    where
        S: AsRef<str>;

    fn with_dual_token_organization<S>(self, organization_id: S) -> Self
    where
        S: AsRef<str>;

    fn with_dual_token_user<S>(self, user_id: S) -> Self
    where
        S: AsRef<str>;

    fn with_dual_token_actor<S>(self, actor_id: S) -> Self
    where
        S: AsRef<str>;

    fn with_dual_token_actor_kind<S>(self, actor_kind: S) -> Self
    where
        S: AsRef<str>;

    fn with_dual_token_session<S>(self, session_id: S) -> Self
    where
        S: AsRef<str>;

    fn with_dual_token_device<S>(self, device_id: S) -> Self
    where
        S: AsRef<str>;

    fn with_dual_token_app<S>(self, app_id: S) -> Self
    where
        S: AsRef<str>;

    fn with_dual_token_permission_scope<S>(self, permission_scope: S) -> Self
    where
        S: AsRef<str>;
}

impl DualTokenRequestBuilderExt for axum::http::request::Builder {
    fn with_dual_token_context<I, S>(
        self,
        tenant_id: &str,
        user_id: &str,
        actor_kind: &str,
        device_id: Option<&str>,
        permission_scope: I,
    ) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        let context =
            local_service_app_context(tenant_id, user_id, actor_kind, device_id, permission_scope);
        let headers =
            build_dual_token_headers_for_context(&context, context.permission_scope.iter());
        headers
            .iter()
            .fold(self, |builder, (name, value)| builder.header(name, value))
    }

    fn with_dual_token_tenant<S>(self, tenant_id: S) -> Self
    where
        S: AsRef<str>,
    {
        let tenant_id = tenant_id.as_ref().to_owned();
        with_updated_local_dual_token_context(self, move |context| {
            context.tenant_id = tenant_id;
        })
    }

    fn with_dual_token_organization<S>(self, organization_id: S) -> Self
    where
        S: AsRef<str>,
    {
        let organization_id = organization_id.as_ref().to_owned();
        with_updated_local_dual_token_context(self, move |context| {
            context.organization_id = organization_id;
        })
    }

    fn with_dual_token_user<S>(self, user_id: S) -> Self
    where
        S: AsRef<str>,
    {
        let user_id = user_id.as_ref().to_owned();
        with_updated_local_dual_token_context(self, move |context| {
            context.user_id = user_id.clone();
            context.actor_id = user_id;
        })
    }

    fn with_dual_token_actor<S>(self, actor_id: S) -> Self
    where
        S: AsRef<str>,
    {
        let actor_id = actor_id.as_ref().to_owned();
        with_updated_local_dual_token_context(self, move |context| {
            context.actor_id = actor_id;
        })
    }

    fn with_dual_token_actor_kind<S>(self, actor_kind: S) -> Self
    where
        S: AsRef<str>,
    {
        let actor_kind = actor_kind.as_ref().to_owned();
        with_updated_local_dual_token_context(self, move |context| {
            context.actor_kind = actor_kind;
        })
    }

    fn with_dual_token_session<S>(self, session_id: S) -> Self
    where
        S: AsRef<str>,
    {
        let session_id = session_id.as_ref().to_owned();
        with_updated_local_dual_token_context(self, move |context| {
            context.session_id = Some(session_id);
        })
    }

    fn with_dual_token_device<S>(self, device_id: S) -> Self
    where
        S: AsRef<str>,
    {
        let device_id = device_id.as_ref().to_owned();
        with_updated_local_dual_token_context(self, move |context| {
            context.device_id = Some(device_id);
        })
    }

    fn with_dual_token_app<S>(self, app_id: S) -> Self
    where
        S: AsRef<str>,
    {
        let app_id = app_id.as_ref().to_owned();
        with_updated_local_dual_token_context(self, move |context| {
            context.app_id = Some(app_id);
        })
    }

    fn with_dual_token_permission_scope<S>(self, permission_scope: S) -> Self
    where
        S: AsRef<str>,
    {
        let permission_scope = split_scope(permission_scope.as_ref());
        with_updated_local_dual_token_context(self, move |context| {
            context.permission_scope = permission_scope;
        })
    }
}

fn with_updated_local_dual_token_context<F>(
    mut builder: axum::http::request::Builder,
    update: F,
) -> axum::http::request::Builder
where
    F: FnOnce(&mut AppContext),
{
    let mut context = builder
        .headers_ref()
        .and_then(|headers| resolve_app_context(headers).ok())
        .unwrap_or_else(local_service_app_context_from_env);
    update(&mut context);
    let headers = build_dual_token_headers_for_context(&context, context.permission_scope.iter());
    if let Some(target_headers) = builder.headers_mut() {
        target_headers.remove(header::AUTHORIZATION);
        target_headers.remove("Access-Token");
        for (name, value) in headers.iter() {
            target_headers.insert(name, value.clone());
        }
    }
    builder
}
