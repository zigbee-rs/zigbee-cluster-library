//! General ZCL Frame

use byte::{ctx, BytesExt, TryRead, TryWrite};

use crate::header::ZclHeader;

/// ZCL Frame
///
/// See Section 2.4.1
#[allow(missing_docs)]
pub enum ZclFrame<'a> {
    GeneralCommand(GeneralCommand<'a>),
    ClusterSpecificCommand(ClusterSpecificCommand<'a>),
    Reserved(ZclHeader),
}

#[allow(missing_docs)]
pub struct GeneralCommand<'a> {
    /// ZCL Header
    pub header: ZclHeader,
    /// ZCL Payload
    pub payload: &'a [u8],
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
                let payload = bytes.read_with(offset, ctx::Bytes::Len(bytes.len() - *offset))?;

                Self::GeneralCommand(GeneralCommand { header, payload })
            }
            crate::header::frame_control::FrameType::ClusterCommand => {
                let payload = bytes.read_with(offset, ctx::Bytes::Len(bytes.len() - *offset))?;

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
                bytes.write(offset, general_command.payload)?;

                Ok(*offset)
            }
            ZclFrame::ClusterSpecificCommand(cluster_specific_command) => {
                bytes.write_with(offset, cluster_specific_command.header, ())?;
                bytes.write(offset, cluster_specific_command.payload)?;

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

    use super::ZclFrame;

    #[allow(clippy::panic)]
    #[test]
    fn zcl_command() {
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
        let expected = &[0x00, 0x00, 0x29, 0x3f, 0x0a];
        assert!(matches!(frame, ZclFrame::GeneralCommand(_)));
        if let ZclFrame::GeneralCommand(general_command) = frame {
            assert!(!general_command
                .header
                .frame_control
                .is_manufacturer_specific());
            assert_eq!(general_command.payload, expected);
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
