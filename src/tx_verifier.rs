use ckb_types::core::TransactionView;
use ckb_verification::TransactionError;

/// An offline transaction checker that helps detect incorrect transaction structures.
pub struct OutputsDataVerifier<'a> {
    transaction: &'a TransactionView,
}

impl<'a> OutputsDataVerifier<'a> {
    /// Returns a new OutputsDataVerifier.
    pub fn new(transaction: &'a TransactionView) -> Self {
        Self { transaction }
    }

    /// Check if count of cell output and count of output data are equal.
    pub fn verify(&self) -> Result<(), TransactionError> {
        let outputs_len = self.transaction.outputs().len();
        let outputs_data_len = self.transaction.outputs_data().len();

        if outputs_len != outputs_data_len {
            return Err(TransactionError::OutputsDataLengthMismatch {
                outputs_data_len,
                outputs_len,
            });
        }
        Ok(())
    }
}
