//! Types that contain invocation metadata.

use std::cell::RefCell;
use std::collections::HashMap;
use std::sync::Arc;

/// Information about the calling application.
#[derive(Debug)]
pub struct ClientApplication {
    pub(crate) installation_id: String,
    pub(crate) app_title: String,
    pub(crate) app_version_code: String,
    pub(crate) app_package_name: String,
}

/// Information about the client application passed by the calling application.
#[derive(Debug)]
pub struct ClientContext {
    pub(crate) client: ClientApplication,
    pub(crate) env: HashMap<String, String>,
    pub(crate) custom: HashMap<String, String>,
}

/// Information about the cognito identity used by the calling application.
#[derive(Debug)]
pub struct CognitoIdentity {
    pub(crate) cognito_identity_id: Option<String>,
    pub(crate) cognito_identity_pool_id: Option<String>,
}

impl CognitoIdentity {
    /// Cognito identity ID.
    pub fn id(&self) -> Option<&str> {
        self.cognito_identity_id.as_ref().map(|s| s.as_ref())
    }

    /// Cognito identity pool ID.
    pub fn pool_id(&self) -> Option<&str> {
        self.cognito_identity_pool_id.as_ref().map(|s| s.as_ref())
    }
}

#[derive(Debug)]
pub(crate) struct LambdaContext {
    pub(crate) aws_request_id: String,
    pub(crate) invoked_function_arn: String,
    pub(crate) identity: CognitoIdentity,
    pub(crate) client_context: Option<ClientContext>,
}

task_local! {
    static CTX: RefCell<Option<Context>> = RefCell::new(None)
}

/// Metadata that is passed to the function on invocation.
#[derive(Clone, Debug)]
pub struct Context {
    inner: Arc<LambdaContext>,
}

impl Context {
    /// Retrieve the current context.
    ///
    /// ## Panics
    ///
    /// This function will panic when called outside of a lambda runtime task.
    pub fn current() -> Context {
        let opt_ctx = CTX.with(|ctx_cell| ctx_cell.borrow().clone());
        if let Some(ctx) = opt_ctx {
            return ctx;
        } else {
            panic!("Context::current() called outside of a lambda runtime task");
        }
    }

    /// AWS Request ID.
    pub fn aws_request_id(&self) -> &str {
        &self.inner.aws_request_id
    }

    /// ARN of the invoked function.
    pub fn invoked_function_arn(&self) -> &str {
        &self.inner.invoked_function_arn
    }

    /// Information about the cognito identity used by the calling application.
    pub fn identity(&self) -> &CognitoIdentity {
        &self.inner.identity
    }

    pub(crate) fn set_current(lctx: LambdaContext) {
        CTX.with(|ctx_cell| {
            *ctx_cell.borrow_mut() = Some(Context {
                inner: Arc::new(lctx),
            });
        });
    }
}

#[test]
#[should_panic]
fn context_current_panics_outside_of_task() {
    Context::current();
}
