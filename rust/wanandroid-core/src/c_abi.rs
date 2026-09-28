use crate::ffi::{BindingError, CoreHandle, CoreSnapshot};
use crate::repository::{AuthRepository, HomeRepository, RepositoryError, WanAndroidRepository};
use crate::reqwest_client::ReqwestHttpClient;
use crate::search::SearchRepository;
use crate::session::MemorySessionStore;
use std::ffi::{CStr, CString};
use std::os::raw::c_char;
use std::time::Duration;

pub type NativeRepository = WanAndroidRepository<ReqwestHttpClient, MemorySessionStore>;
pub type NativeCoreHandle = CoreHandle<NativeRepository>;

#[repr(C)]
pub struct NativeStringResult {
    pub value: *mut c_char,
    pub error: *mut c_char,
}

impl NativeStringResult {
    pub fn success(value: String) -> Self {
        Self {
            value: into_c_string(value),
            error: std::ptr::null_mut(),
        }
    }

    pub fn failure(error: impl std::fmt::Display) -> Self {
        Self {
            value: std::ptr::null_mut(),
            error: into_c_string(error.to_string()),
        }
    }
}

fn into_c_string(value: String) -> *mut c_char {
    CString::new(value)
        .unwrap_or_else(|_| CString::new("native string contains NUL").expect("static string"))
        .into_raw()
}

/// Releases a string returned from any `wanandroid_core_*` result.
///
/// # Safety
/// `value` must be null or a pointer returned by this library that has not already been freed.
#[no_mangle]
pub unsafe extern "C" fn wanandroid_string_free(value: *mut c_char) {
    if !value.is_null() {
        drop(CString::from_raw(value));
    }
}

#[no_mangle]
pub extern "C" fn wanandroid_core_create(timeout_ms: u64) -> *mut NativeCoreHandle {
    let client = match ReqwestHttpClient::wanandroid(Duration::from_millis(timeout_ms.max(1))) {
        Ok(client) => client,
        Err(_) => return std::ptr::null_mut(),
    };
    let repository = WanAndroidRepository::with_session(
        std::sync::Arc::new(client),
        std::sync::Arc::new(MemorySessionStore::default()),
    );
    Box::into_raw(Box::new(CoreHandle::new(repository)))
}

/// Releases an opaque handle created by `wanandroid_core_create`.
///
/// # Safety
/// `handle` must be null or a live pointer returned by `wanandroid_core_create`, and must be
/// destroyed at most once after all callers stop using it.
#[no_mangle]
pub unsafe extern "C" fn wanandroid_core_destroy(handle: *mut NativeCoreHandle) {
    if !handle.is_null() {
        drop(Box::from_raw(handle));
    }
}

/// Serializes the current snapshot for an opaque core handle.
///
/// # Safety
/// `handle` must be null or a live pointer returned by `wanandroid_core_create`.
#[no_mangle]
pub unsafe extern "C" fn wanandroid_core_snapshot(
    handle: *const NativeCoreHandle,
) -> NativeStringResult {
    if handle.is_null() {
        return NativeStringResult::failure("core handle is null");
    }
    match (*handle).snapshot_json() {
        Ok(snapshot) => NativeStringResult::success(snapshot),
        Err(error) => NativeStringResult::failure(error),
    }
}

/// Dispatches a UTF-8 JSON action and returns the resulting JSON snapshot.
///
/// # Safety
/// `handle` must be null or a live core pointer. `action_json` must be null or a valid,
/// NUL-terminated UTF-8 C string that remains alive for the duration of this call.
#[no_mangle]
pub unsafe extern "C" fn wanandroid_core_dispatch(
    handle: *const NativeCoreHandle,
    action_json: *const c_char,
) -> NativeStringResult {
    if handle.is_null() {
        return NativeStringResult::failure("core handle is null");
    }
    if action_json.is_null() {
        return NativeStringResult::failure("action JSON pointer is null");
    }
    let action_json = match CStr::from_ptr(action_json).to_str() {
        Ok(value) => value,
        Err(error) => {
            return NativeStringResult::failure(BindingError::InvalidAction(error.to_string()))
        }
    };
    match (*handle).dispatch_json(action_json) {
        Ok(snapshot) => NativeStringResult::success(snapshot),
        Err(error) => NativeStringResult::failure(error),
    }
}

/// Generic helper retained for host-side adapters that already own a repository.
pub fn dispatch_json<R>(handle: &CoreHandle<R>, action_json: &str) -> Result<String, BindingError>
where
    R: Clone
        + HomeRepository
        + AuthRepository
        + SearchRepository
        + crate::project::ProjectRepository
        + crate::collection::CollectionRepository,
{
    handle.dispatch_json(action_json)
}

#[allow(dead_code)]
fn _native_contract_types_are_send_sync()
where
    NativeRepository: Send + Sync,
    CoreSnapshot: Send + Sync,
    RepositoryError: Send + Sync,
{
}

#[cfg(test)]
mod tests {
    use super::NativeStringResult;

    #[test]
    fn native_string_result_has_exactly_one_payload_pointer() {
        let success = NativeStringResult::success("{}".into());
        assert!(!success.value.is_null());
        assert!(success.error.is_null());
        unsafe { super::wanandroid_string_free(success.value) };

        let failure = NativeStringResult::failure("bad action");
        assert!(failure.value.is_null());
        assert!(!failure.error.is_null());
        unsafe { super::wanandroid_string_free(failure.error) };
    }
}
