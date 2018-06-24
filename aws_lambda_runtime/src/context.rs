//! Types that contain invocation metadata.

use std::collections::HashMap;
use std::sync::Arc;

/// Information about the client application that invoked the lambda function.
#[derive(Debug)]
pub struct Client {
    pub(crate) installation_id: String,
    pub(crate) app_title: String,
    pub(crate) app_version_code: String,
    pub(crate) app_package_name: String,
}

impl Client {
    /// Installation id of the application.
    pub fn installation_id(&self) -> &str {
        &self.installation_id
    }

    /// Title of the application.
    pub fn app_title(&self) -> &str {
        &self.app_title
    }

    /// Version of the application.
    pub fn app_version_code(&self) -> &str {
        &self.app_version_code
    }

    /// Package name of the application.
    pub fn app_package_name(&self) -> &str {
        &self.app_package_name
    }
}

/// Client-specific information passed by the calling application.
#[derive(Debug)]
pub struct ClientContext {
    pub(crate) client: Client,
    pub(crate) env: HashMap<String, String>,
    pub(crate) custom: HashMap<String, String>,
}

impl ClientContext {
    /// Client information provided by the mobile SDK.
    pub fn client(&self) -> &Client {
        &self.client
    }

    /// Get custom values set by the client application.
    pub fn get_custom(&self, key: &str) -> Option<&str> {
        self.custom.get(key).map(|s| s.as_ref())
    }

    /// Get environment information provided by mobile SDK.
    pub fn get_environment(&self, key: &str) -> Option<&str> {
        self.env.get(key).map(|s| s.as_ref())
    }
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

scoped_thread_local!(static CTX: Context);

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
        if CTX.is_set() {
            CTX.with(|ctx| ctx.clone())
        } else {
            panic!("Context::current() called outside of a lambda runtime task");
        }
    }

    /// AWS request ID associated with the request.
    pub fn aws_request_id(&self) -> &str {
        &self.inner.aws_request_id
    }

    /// ARN of the function being invoked.
    pub fn invoked_function_arn(&self) -> &str {
        &self.inner.invoked_function_arn
    }

    /// Gets information about the Amazon Cognito identity provider when invoked
    /// through the AWS Mobile SDK.
    pub fn identity(&self) -> &CognitoIdentity {
        &self.inner.identity
    }

    /// Information about the client application and device when invoked
    /// through the AWS Mobile SDK.
    pub fn client_context(&self) -> Option<&ClientContext> {
        self.inner.client_context.as_ref()
    }

    pub(crate) fn new(lctx: LambdaContext) -> Context {
        Context {
            inner: Arc::new(lctx),
        }
    }

    pub(crate) fn with<F, R>(&self, f: F) -> R
    where
        F: FnOnce() -> R,
    {
        CTX.set(self, f)
    }
}

#[test]
#[should_panic]
fn context_current_panics_outside_of_task() {
    Context::current();
}
