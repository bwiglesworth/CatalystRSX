#[cfg(test)]
mod tests {
    #[test]
    fn test_put_method_errors() {
        let response = handle_put_request(invalid_data);
        assert!(matches!(response.error(), HttpError::InvalidMethod));
    }

    #[test]
    fn test_delete_method_errors() {
        let response = handle_delete_request(nonexistent_id);
        assert!(matches!(response.error(), HttpError::ResourceNotFound));
    }

    #[test]
    fn test_patch_method_errors() {
        let response = handle_patch_request(partial_data);
        assert!(matches!(response.error(), HttpError::ValidationFailed));
    }
}
