use std::cell::RefCell;

/// Request context for Inertia - stored in thread-local storage
#[derive(Clone, Default)]
pub struct InertiaContext {
    pub path: String,
    pub is_inertia: bool,
    pub version: Option<String>,
}

thread_local! {
    static CONTEXT: RefCell<Option<InertiaContext>> = RefCell::new(None);
}

impl InertiaContext {
    /// Set the current request context (called by server before handler)
    pub fn set(ctx: InertiaContext) {
        CONTEXT.with(|c| {
            *c.borrow_mut() = Some(ctx);
        });
    }

    /// Get the current request context
    pub fn get() -> Option<InertiaContext> {
        CONTEXT.with(|c| c.borrow().clone())
    }

    /// Clear the context (called after handler completes)
    pub fn clear() {
        CONTEXT.with(|c| {
            *c.borrow_mut() = None;
        });
    }

    /// Get current path or empty string
    pub fn current_path() -> String {
        Self::get().map(|c| c.path).unwrap_or_default()
    }

    /// Check if current request is an Inertia XHR request
    pub fn is_inertia_request() -> bool {
        Self::get().map(|c| c.is_inertia).unwrap_or(false)
    }
}
