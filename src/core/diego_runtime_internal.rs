pub(crate) struct DiegoRuntimeInternal {
    pub check_opengl_errors: bool,
}

impl DiegoRuntimeInternal {
    pub(crate) fn new() -> Self {
        Self {
            check_opengl_errors: false,
        }
    }
}
