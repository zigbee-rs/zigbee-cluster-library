//! General ZCL Frame
#![allow(missing_docs)]

use byte::ctx::{self};
use heapless::Vec;

use crate::{
    common::data_types::ZclDataType,
    header::ZclHeader,
    impl_byte, payload::ZclFramePayload,
};

impl_byte! {
    /// ZCL Frame
    ///
    /// See Section 2.4.1
    #[derive(Debug)]
    pub struct ZclFrame<'a> {
        pub header: ZclHeader,
        #[ctx = &header]
        pub payload: ZclFramePayload<'a>,
    }
}

#[derive(Debug)]
pub enum GeneralCommand<'a> {
    ReadAttributesCommand(Vec<ReadAttribute, 16>),
    // ReportAttributesCommand(Vec<AttributeReport<'a>, 16>),
    // ...
}

impl_byte! {
    #[derive(Debug,PartialEq)]
    pub struct ReadAttribute {
        pub attribute_id: u16,
    }
}

// impl_byte! {
//     #[derive(Debug,PartialEq)]
//     pub struct AttributeReport<'a> {
//         pub attribute_id: u16,
//         pub data_type: ZclDataType<'a>,
//         #[ctx = ctx::Bytes::Len(data_type.length())]
//         pub value: &'a [u8],
//     }
// }

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
        // let (frame, _) = ZclFrame::try_read(input, &ZclHeader).expect("Failed to read ZclFrame");

        // then
        // assert!(frame.header.frame_control.is_manufacturer_specific());
        // assert_eq!(frame.header.sequence_number, 1);
        // assert!(matches!(frame.payload, ZclFramePayload::GeneralCommand(_)));

        // let expected = &[0x00, 0x00, 0x29, 0x3f, 0x0a];
        // assert!(matches!(frame, ZclFrame::GeneralCommand(_)));
        // if let ZclFramePayload::GeneralCommand(cmd) = frame.payload {
        //     if let GeneralCommand::ReportAttributesCommand(report) = cmd {
        //         assert_eq!(report.len(), 1);
        //     }  else {
        //         panic!("Report Attributes Command expected!");
        //     }
        // } else {
        //     panic!("GeneralCommand expecyed!");
        // }
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
        // assert!(matches!(frame, ZclFrame::ClusterSpecificCommand(_)));
        // if let ZclFrame::ClusterSpecificCommand(general_command) = frame {
        //     assert!(!general_command
        //         .header
        //         .frame_control
        //         .is_manufacturer_specific());
        //     assert_eq!(general_command.payload, expected);
        // } else {
        //     panic!("ClusterSpecificCommand expecyed!");
        // }
    }
}
