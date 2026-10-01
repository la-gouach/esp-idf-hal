//! Digital Signature driver
//!
//! To use, put this in your `Cargo.toml`:
//! ```
//! [[package.metadata.esp-idf-sys.extra_components]]
//! remote_component = { name = "espressif/esp_secure_cert_mgr", version = "2.5.0" }
//! ```

use crate::{
    sys::{
        esp_ds_data_ctx, esp_secure_cert_free_ds_ctx, esp_secure_cert_get_ds_ctx, EspError,
        ESP_FAIL,
    },
};
use core::{fmt, marker::PhantomData};

pub struct DsDriver<'d> {
    ctx: *mut esp_ds_data_ctx,
    _p: PhantomData<&'d mut ()>,
}

impl<'d> DsDriver<'d> {
    pub fn new(
        _ds: DS<'d>,
) -> Result<Self, EspError> {
        let ctx = unsafe { esp_secure_cert_get_ds_ctx() };
        if ctx.is_null() {
            Err(EspError::from_infallible::<ESP_FAIL>())
        } else {
            Ok(Self {
                ctx,
                _p: PhantomData,
            })
        }
    }

    pub unsafe fn as_ptr(&self) -> *mut esp_ds_data_ctx {
        self.ctx
    }
}

impl Drop for DsDriver<'_> {
    fn drop(&mut self) {
        unsafe { esp_secure_cert_free_ds_ctx(self.ctx) }
    }
}

unsafe impl Send for DsDriver<'_> {}
unsafe impl Sync for DsDriver<'_> {}

impl fmt::Debug for DsDriver<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("DsDriver").finish_non_exhaustive()
    }
}

crate::impl_peripheral!(DS);
