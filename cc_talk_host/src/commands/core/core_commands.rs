use cc_talk_core::{Category, Header, cc_talk::Manufacturer};

use super::{
    super::command::{BelongsTo, Command, ParseResponseError},
    CoreCommandSet,
};

pub struct SimplePollCommand;
impl Command for SimplePollCommand {
    type Response = ();

    fn header(&self) -> Header {
        Header::SimplePoll
    }

    fn data(&self) -> &[u8] {
        &[]
    }

    fn parse_response(
        &self,
        response_payload: &[u8],
    ) -> Result<Self::Response, ParseResponseError> {
        match response_payload.is_empty() {
            true => Ok(()),
            false => Err(ParseResponseError::DataLengthMismatch(
                0,
                response_payload.len(),
            )),
        }
    }
}
impl BelongsTo<CoreCommandSet> for SimplePollCommand {}

pub struct RequestManufacturerIdCommand;
impl Command for RequestManufacturerIdCommand {
    type Response = Manufacturer;

    fn header(&self) -> Header {
        Header::RequestManufacturerId
    }

    fn data(&self) -> &[u8] {
        &[]
    }

    fn parse_response(
        &self,
        response_payload: &[u8],
    ) -> Result<Self::Response, ParseResponseError> {
        let manufacturer_str = core::str::from_utf8(response_payload)
            .map_err(|_| ParseResponseError::ParseError("Invalid UTF-8 response"))?
            .trim();

        Manufacturer::from_name(manufacturer_str)
            .ok_or(ParseResponseError::ParseError("Unknown manufacturer"))
    }
}
impl BelongsTo<CoreCommandSet> for RequestManufacturerIdCommand {}

pub struct RequestEquipementCategoryIdCommand;
impl Command for RequestEquipementCategoryIdCommand {
    type Response = Category;

    fn header(&self) -> Header {
        Header::RequestEquipementCategoryId
    }

    fn data(&self) -> &[u8] {
        &[]
    }

    /// Parses the response payload as a category ID.
    fn parse_response(
        &self,
        response_payload: &[u8],
    ) -> Result<Self::Response, ParseResponseError> {
        let category_str = core::str::from_utf8(response_payload)
            .map_err(|_| ParseResponseError::ParseError("Invalid UTF-8 response"))?
            .trim();

        Ok(Category::from(category_str))
    }
}
impl BelongsTo<CoreCommandSet> for RequestEquipementCategoryIdCommand {}

pub struct RequestProductCodeCommand;
impl Command for RequestProductCodeCommand {
    type Response = ();

    fn header(&self) -> Header {
        Header::RequestProductCode
    }

    fn data(&self) -> &[u8] {
        &[]
    }

    /// The answer to this command is a string, currently the `parse_response` will only check if
    /// the response is valid UTF-8.
    ///
    /// The cast to a valid data type depending on the enviornment (std, heapless, etc.) is left to
    /// the user.
    fn parse_response(
        &self,
        response_payload: &[u8],
    ) -> Result<Self::Response, ParseResponseError> {
        if !response_payload.iter().all(|&b| b.is_ascii()) {
            return Err(ParseResponseError::ParseError("Invalid ASCII response"));
        }
        Ok(())
    }
}
impl BelongsTo<CoreCommandSet> for RequestProductCodeCommand {}

pub struct RequestBuildCodeCommand;
impl Command for RequestBuildCodeCommand {
    type Response = ();

    fn header(&self) -> Header {
        Header::RequestBuildCode
    }

    fn data(&self) -> &[u8] {
        &[]
    }

    /// The answer to this command is a string, currently the `parse_response` will only check if
    /// the response is valid UTF-8.
    ///
    /// The cast to a valid data type depending on the enviornment (std, heapless, etc.) is left to
    /// the user.
    fn parse_response(
        &self,
        response_payload: &[u8],
    ) -> Result<Self::Response, ParseResponseError> {
        if !response_payload.iter().all(|&b| b.is_ascii()) {
            return Err(ParseResponseError::ParseError("Invalid UTF-8 response"));
        }
        Ok(())
    }
}
impl BelongsTo<CoreCommandSet> for RequestBuildCodeCommand {}

#[deprecated(note = "This command is not implemented yet.")]
pub struct RequestEncryptionSupportCommand;
impl Command for RequestEncryptionSupportCommand {
    type Response = bool;

    fn header(&self) -> Header {
        Header::RequestEncryptionSupport
    }

    fn data(&self) -> &[u8] {
        &[170, 85, 0, 0, 85, 170]
    }

    /// The response is a single byte indicating whether encryption is supported.
    fn parse_response(&self, _: &[u8]) -> Result<Self::Response, ParseResponseError> {
        todo!("encryption support command not implemented yet")
    }
}
impl BelongsTo<CoreCommandSet> for RequestEncryptionSupportCommand {}

#[cfg(test)]
mod test {
    use cc_talk_core::Category;

    use super::*;

    #[test]
    fn simple_poll_command() {
        let cmd = SimplePollCommand;
        assert_eq!(cmd.header(), Header::SimplePoll);
        assert!(cmd.data().is_empty());
        assert!(cmd.parse_response(&[]).is_ok());
        assert!(cmd.parse_response(&[1, 2, 3]).is_err());
    }

    #[test]
    fn existing_manufacturer() {
        let cmd = RequestManufacturerIdCommand;
        assert_eq!(cmd.header(), Header::RequestManufacturerId);
        assert!(cmd.data().is_empty());

        let inotek = b"INK";
        let inotek_parsed = cmd.parse_response(inotek);
        assert!(inotek_parsed.is_ok());
        assert_eq!(inotek_parsed.unwrap(), Manufacturer::INOTEK);

        let jcm = b"Japan Cash Machine";
        let jcm_parsed = cmd.parse_response(jcm);
        assert!(jcm_parsed.is_ok());
        assert_eq!(jcm_parsed.unwrap(), Manufacturer::JapanCashMachine);
    }

    #[test]
    fn non_existing_manufacturer() {
        let cmd = RequestManufacturerIdCommand;
        let unknown_manufacturer = b"Unknown Manufacturer";
        let unknown_parsed = cmd.parse_response(unknown_manufacturer);
        assert!(unknown_parsed.is_err());
        assert_eq!(
            unknown_parsed.unwrap_err(),
            ParseResponseError::ParseError("Unknown manufacturer")
        );
    }

    #[test]
    fn category_id_command() {
        let cmd = RequestEquipementCategoryIdCommand;
        assert_eq!(cmd.header(), Header::RequestEquipementCategoryId);
        assert!(cmd.data().is_empty());

        let bill_validator = b"Bill Validator";
        let bill_validator_parsed = cmd.parse_response(bill_validator);
        assert!(bill_validator_parsed.is_ok());
        assert_eq!(bill_validator_parsed.unwrap(), Category::BillValidator);

        let invalid_category_str = b"Unknown Category";
        let invalid_category_parsed = cmd.parse_response(invalid_category_str);
        assert!(invalid_category_parsed.is_ok());
        assert_eq!(invalid_category_parsed.unwrap(), Category::Unknown);
    }

    #[test]
    fn product_code() {
        let cmd = RequestProductCodeCommand;
        assert_eq!(cmd.header(), Header::RequestProductCode);
        assert!(cmd.data().is_empty());

        let valid_product_code = b"Product123";
        let parsed_valid = cmd.parse_response(valid_product_code);
        assert!(parsed_valid.is_ok());

        let non_utf8_product_code = &[0xFF, 0xFE, 0xFD];
        let parsed_invalid = cmd.parse_response(non_utf8_product_code);
        assert!(parsed_invalid.is_err());
    }

    #[test]
    fn request_build_code() {
        let cmd = RequestBuildCodeCommand;
        assert_eq!(cmd.header(), Header::RequestBuildCode);
        assert!(cmd.data().is_empty());

        let valid_build_code = b"Build123";
        let parsed_valid = cmd.parse_response(valid_build_code);
        assert!(parsed_valid.is_ok());

        let invalid_build_code = &[0xFF, 0xFE, 0xFD];
        let parsed_invalid = cmd.parse_response(invalid_build_code);
        assert!(parsed_invalid.is_err());
    }

    #[test]
    #[should_panic]
    fn request_encryption_support() {
        let cmd = RequestEncryptionSupportCommand;
        assert_eq!(cmd.header(), Header::RequestEncryptionSupport);
        assert_eq!(cmd.data(), &[170, 85, 0, 0, 85, 170]);

        // This command is not implemented yet, so we just check that it compiles.
        let _ = cmd.parse_response(&[]);
    }
}
