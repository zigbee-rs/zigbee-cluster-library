//! General ZCL Frame
#![allow(missing_docs)]

use byte::{BytesExt, TryRead, TryWrite};

use crate::{common::data_types::ZclDataType, header::ZclHeader, impl_byte};

/// ZCL Frame
///
/// See Section 2.4.1
#[allow(missing_docs)]
pub enum ZclFrame<'a> {
    GeneralCommand(GeneralCommand<'a>),
    ClusterSpecificCommand(ClusterSpecificCommand<'a>),
    Reserved(ZclHeader),
}

pub enum GeneralCommand<'a> {
    ReportAttributesCommand(ReportAttributesCommand<'a>),
}

impl_byte! {
    pub struct ReportAttributesCommand<'a> {
        /// ZCL Header
        pub header: ZclHeader,
        /// ZCL Payload
        pub payload: &'a [AttributeReport<'a>],
    }
}

impl_byte! {
    #[derive(Debug,PartialEq)]
    pub struct AttributeReport<'a> {
        pub attribute_id: u16,
        pub data_type: ZclDataType<'a>,
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

impl<'a> TryRead<'a, ()> for ZclFrame<'a> {
    fn try_read(bytes: &'a [u8], _: ()) -> byte::Result<(Self, usize)> {
        let offset = &mut 0;

        let header: ZclHeader = bytes.read_with(offset, ())?;
        let frame = match header.frame_control.frame_type() {
            crate::header::frame_control::FrameType::GlobalCommand => {
                let payload = match header.command_identifier {
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
                    ReportAttributes => GeneralCommand::ReportAttributesCommand::try_read(offset, ()),
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
                    _ => bytes.read_with(offset, ())?
                };
                // let payload = bytes.read_with(offset, ctx::Bytes::Len(bytes.len() - *offset))?;

                Self::GeneralCommand(GeneralCommand { header, payload })
            }
            crate::header::frame_control::FrameType::ClusterCommand => {
                let payload = bytes.read_with(offset, bytes::ctx::Bytes::Len(bytes.len() - *offset))?;

                Self::ClusterSpecificCommand(ClusterSpecificCommand { header, payload })
            }
            crate::header::frame_control::FrameType::Reserved => Self::Reserved(header),
        };

        Ok((frame, *offset))
    }
}

impl TryWrite for ZclFrame<'_> {
    fn try_write(self, bytes: &mut [u8], _: ()) -> byte::Result<usize> {
        let offset = &mut 0;
        match self {
            ZclFrame::GeneralCommand(general_command) => {
                bytes.write_with(offset, general_command.header, ())?;
                // bytes.write_with(offset, general_command.payload, ())?;

                Ok(*offset)
            }
            ZclFrame::ClusterSpecificCommand(cluster_specific_command) => {
                bytes.write_with(offset, cluster_specific_command.header, ())?;
                bytes.write_with(offset, cluster_specific_command.payload, ())?;

                Ok(*offset)
            }
            ZclFrame::Reserved(zcl_header) => {
                bytes.write_with(offset, zcl_header, ())?;

                Ok(*offset)
            }
        }
    }
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
            0x29, 
            0xab, 0x03
        ];

        // when 
        let (report, _) = AttributeReport::try_read(input, ()).expect("Failed to read AttributeReport in test");

        // then
        assert_eq!(report.attribute_id, 0u16);
        assert_eq!(report.data_type, ZclDataType::SignedInt(crate::common::data_types::SignedN::Int16(5)));
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

