//! General ZCL Frame
#![allow(missing_docs)]

use byte::{ctx, BytesExt, TryRead, TryWrite};
use heapless::Vec;

use crate::{
    common::data_types::ZclDataType,
    header::{command_identifier::CommandIdentifier, frame_control::FrameType, ZclHeader},
    impl_byte,
};

impl_byte! {
    /// ZCL Frame
    ///
    /// See Section 2.4.1
    #[derive(Debug)]
    pub struct ZclFrame<'a> {
        pub header: ZclHeader,
        pub payload: ZclFramePayload<'a>,
    }
}

pub enum ZclFramePayload<'a> {
    GeneralCommand(GeneralCommand<'a>),
    ClusterSpecificCommand(&'a [u8]),
    Reserved,
}

impl<'a> TryRead<'a, &ZclHeader> for ZclFramePayload<'a> {
    fn try_read(bytes: &'a [u8], header: &ZclHeader) -> Result<(Self, usize), ::byte::Error> {
        let offset = &mut 0;
        let payload = match header.frame_control.frame_type() {
            FrameType::GlobalCommand => {
                let cmd = match header.command_identifier {
                    // ReadAttributes => todo!(),
                    // ReadAttributesResponse => todo!(),
                    // WriteAttributes => todo!(),
                    // WriteAttributesUndivided => todo!(),
                    // WriteAttributesResponse => todo!(),
                    // WriteAttributesNoResponse => todo!(),
                    // ConfigureReporting => todo!(),
                    // ConfigureReportingResponse => todo!(),
                    // ReadReportingConfiguration => todo!(),
                    // ReadReportingConfigurationResponse => todo!(),
                    CommandIdentifier::ReportAttributes => {
                        let mut attribute_reports: Vec<AttributeReport<'_>, 16> = Vec::new();
                        while let Ok(attribute_report) = bytes.read_with(offset, ()) {
                            attribute_reports.push(attribute_report).unwrap();
                        }
                        GeneralCommand::ReportAttributesCommand(attribute_reports)
                    }
                    // DefaultResponse => todo!(),
                    // DiscoverAttributes => todo!(),
                    // DiscoverAttributesResponse => todo!(),
                    // ReadAttributesStructured => todo!(),
                    // WriteAttributesStructured => todo!(),
                    // WriteAttributesStructuredResponse => todo!(),
                    // DiscoverCommandsReceived => todo!(),
                    // DiscoverCommandsReceivedResponse => todo!(),
                    // DiscoverCommandsGenerated => todo!(),
                    // DiscoverCommandsGeneratedResponse => todo!(),
                    // DiscoverAttributesExtended => todo!(),
                    // DiscoverAttributesExtendedResponse => todo!(),
                    // Reserved => todo!(),
                    _ => todo!(),
                };
                ZclFramePayload::GeneralCommand(cmd)
            }
            FrameType::ClusterCommand => todo!(),
            FrameType::Reserved => todo!(),
        };

        Ok((payload, *offset))
    }
}
impl TryWrite<&ZclHeader> for ZclFramePayload<'_> {
    fn try_write(self, bytes: &mut [u8], header: &ZclHeader) -> Result<usize, ::byte::Error> {
        unimplemented!()
    }
}

pub enum GeneralCommand<'a> {
    ReadAttributesCommand(Vec<ReadAttribute, 16>),
    ReportAttributesCommand(Vec<AttributeReport<'a>, 16>),
    // ...
}

impl_byte! {
    #[derive(Debug,PartialEq)]
    pub struct ReadAttribute {
        pub attribute_id: u16,
    }
}

impl_byte! {
    #[derive(Debug,PartialEq)]
    pub struct AttributeReport<'a> {
        pub attribute_id: u16,
        pub data_type: ZclDataType<'a>,
        #[ctx = ctx::Bytes::Len(data_type.length())]
        pub value: &'a [u8],
    }
}

#[allow(missing_docs)]
pub struct ClusterSpecificCommand<'a> {
    /// ZCL Header
    pub header: ZclHeader,
    /// ZCL Payload
    pub payload: &'a [u8],
}

#[cfg(test)]
mod tests {
    use byte::TryRead;

    use super::*;

    #[test]
    fn parse_attribute_report() {
        // given
        let input: &[u8] = &[
            0x00, 0x00, // identifier
            0x29, 0xab, 0x03,
        ];

        // when
        let (report, _) =
            AttributeReport::try_read(input, ()).expect("Failed to read AttributeReport in test");

        // then
        assert_eq!(report.attribute_id, 0u16);
        assert_eq!(
            report.data_type,
            ZclDataType::SignedInt(crate::common::data_types::SignedN::Int16(5))
        );
        assert_eq!(report.value, &[]);
    }

    #[allow(clippy::panic)]
    #[test]
    fn zcl_general_command() {
        // given
        let input: &[u8] = &[
            0x18, // frame control
            0x01, // sequence number
            0x0a, // command
            0x00, 0x00, 0x29, 0x3f, 0x0a, // payload
        ];

        // when
        let (frame, _) = ZclFrame::try_read(input, ()).expect("Failed to read ZclFrame");

        // then
        // let expected = &[0x00, 0x00, 0x29, 0x3f, 0x0a];
        assert!(matches!(frame, ZclFrame::GeneralCommand(_)));
        if let ZclFrame::GeneralCommand(general_command) = frame {
            assert!(!general_command
                .header
                .frame_control
                .is_manufacturer_specific());
            // assert_eq!(general_command.payload, expected);
        } else {
            panic!("GeneralCommand expecyed!");
        }
    }

    #[allow(clippy::panic)]
    #[test]
    fn cluster_specific_command() {
        // given
        let input: &[u8] = &[
            0x19, // frame control
            0x01, // sequence number
            0x0a, // command
            0x00, 0x00, 0x29, 0x3f, 0x0a, // payload
        ];

        // when
        let (frame, _) = ZclFrame::try_read(input, ()).expect("Failed to read ZclFrame");

        // then
        let expected = &[0x00, 0x00, 0x29, 0x3f, 0x0a];
        assert!(matches!(frame, ZclFrame::ClusterSpecificCommand(_)));
        if let ZclFrame::ClusterSpecificCommand(general_command) = frame {
            assert!(!general_command
                .header
                .frame_control
                .is_manufacturer_specific());
            assert_eq!(general_command.payload, expected);
        } else {
            panic!("ClusterSpecificCommand expecyed!");
        }
    }
}
