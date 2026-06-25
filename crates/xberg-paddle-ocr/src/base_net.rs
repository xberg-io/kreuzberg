use ort::session::{
    Session,
    builder::{GraphOptimizationLevel, SessionBuilder},
};

use crate::ocr_error::OcrError;

pub trait BaseNet {
    fn new() -> Self;

    fn get_session_builder(
        &self,
        num_thread: usize,
        builder_fn: Option<fn(SessionBuilder) -> Result<SessionBuilder, ort::Error>>,
    ) -> Result<SessionBuilder, OcrError> {
        let builder = Session::builder()?;
        let builder = match builder_fn {
            Some(custom) => custom(builder)?,
            None => builder
                .with_optimization_level(GraphOptimizationLevel::All)
                .map_err(|e| OcrError::Ort(ort::Error::new(e.message())))?
                .with_intra_threads(num_thread)
                .map_err(|e| OcrError::Ort(ort::Error::new(e.message())))?
                .with_inter_threads(1)
                .map_err(|e| OcrError::Ort(ort::Error::new(e.message())))?,
        };

        Ok(builder)
    }

    fn set_input_names(&mut self, input_names: Vec<String>);
    fn set_session(&mut self, session: Option<Session>);

    fn init(&mut self, session: Session) {
        let input_names: Vec<String> = session.inputs().iter().map(|input| input.name().to_string()).collect();

        self.set_input_names(input_names);
        self.set_session(Some(session));
    }

    fn init_model(
        &mut self,
        path: &str,
        num_thread: usize,
        builder_fn: Option<fn(SessionBuilder) -> Result<SessionBuilder, ort::Error>>,
    ) -> Result<(), OcrError> {
        // Wrap ORT session creation in catch_unwind to prevent mutex poisoning
        // on platforms where ORT initialization can panic (notably Windows).
        let path_owned = path.to_string();
        let session = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut builder = self.get_session_builder(num_thread, builder_fn)?;
            builder.commit_from_file(&path_owned).map_err(OcrError::from)
        }))
        .map_err(|_| OcrError::Ort(ort::Error::new("ORT session initialization panicked")))??;
        self.init(session);

        Ok(())
    }

    fn init_model_from_memory(
        &mut self,
        model_bytes: &[u8],
        num_thread: usize,
        builder_fn: Option<fn(SessionBuilder) -> Result<SessionBuilder, ort::Error>>,
    ) -> Result<(), OcrError> {
        // Wrap ORT session creation in catch_unwind to prevent mutex poisoning
        // on platforms where ORT initialization can panic (notably Windows).
        let session = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut builder = self.get_session_builder(num_thread, builder_fn)?;
            builder.commit_from_memory(model_bytes).map_err(OcrError::from)
        }))
        .map_err(|_| OcrError::Ort(ort::Error::new("ORT session initialization panicked")))??;

        self.init(session);

        Ok(())
    }
}
