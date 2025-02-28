//! ZCL Header
#![allow(dead_code, unreachable_pub)]
use core::{fmt, mem};

use crate::{common::parse::PackBytes, impl_pack_bytes};
use core::fmt::Debug;

#[derive(Debug)]
pub struct ZclHeader {
    /// See Section 2.4.1.1
    pub frame_control: FrameControl,
    /// Set only if [`FrameControl::manufacturer_specific`] is `true`.
    /// See Section 2.4.1.2
    pub manufacturer_code: Option<u16>,
    /// See Section 2.4.1.3
    pub sequence_number: u8,
    /// See Section 2.4.1.4
    pub command_identifier: CommandIdentifier,
}

/// 2.4.1.4  Values can be found in Table 2-3
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(missing_docs)]
pub enum CommandIdentifier {
    ReadAttributes = 0x00,
    ReadAttributesResponse = 0x01,
    WriteAttributes = 0x02,
    WriteAttributesUndivided = 0x03,
    WriteAttributesResponse = 0x04,
    WriteAttributesNoResponse = 0x05,
    ConfigureReporting = 0x06,
    ConfigureReportingResponse = 0x07,
    ReadReportingConfiguration = 0x08,
    ReadReportingConfigurationResponse = 0x09,
    ReportAttributes = 0x0a,
    DefaultResponse = 0x0b,
    DiscoverAttributes = 0x0c,
    DiscoverAttributesResponse = 0x0d,
}

impl PackBytes for CommandIdentifier {
    fn unpack_from_iter(src: impl IntoIterator<Item = u8>) -> Option<Self> {
        let b = src.into_iter().next()?;
        if b <= 0x0d {
            // SAFETY: any byte with value <= 0x0d is a valid CommandFrameIdentifier
            Some(unsafe { mem::transmute::<u8, Self>(b) })
        } else {
            Some(Self::ReadAttributes)
        }
    }
}

impl_pack_bytes! {
    /// 64-bit network address
    #[derive(Clone, Copy, PartialEq, Eq)]
    pub struct ManufacturerCode(pub u16);
}

impl PackBytes for ZclHeader {
    fn unpack_from_iter(src: impl IntoIterator<Item = u8>) -> Option<Self> {
        let mut src = src.into_iter();
        let frame_control = FrameControl::unpack_from_iter(&mut src)?;
        Some(Self {
            frame_control,
            manufacturer_code: frame_control
                .manufacturer_specific()
                .then(|| PackBytes::unpack_from_iter(&mut src))
                .flatten(),
            sequence_number: src.next()?,
            command_identifier: CommandIdentifier::unpack_from_iter(src.next())?,
        })
    }
}

impl_pack_bytes! {
    /// See Section 2.4.1.1 Frame Control
    #[derive(Clone, Copy, PartialEq, Eq)]
    pub struct FrameControl(pub u8);
}

impl FrameControl {
    /// See Section 2.4.1.1.1
    ///
    /// Returns `true` if command is global
    pub fn frame_type(self) -> bool {
        ((self.0 >> 1) & 0b11) != 0
    }

    /// The Manufacturer Specific field specifies whether this command refers to a
    /// manufacturer specific extension.
    ///
    /// If this value is set to 1, the manufacturer code field SHALL be present in
    /// the ``ZCLframe``.
    ///
    /// See Section 2.4.1.1.2
    pub fn manufacturer_specific(self) -> bool {
        ((self.0 >> 2) & 0b1) != 0
    }

    /// The direction specifies the client/server direction for this command.
    /// If set to 1, the command is being sent from the server side of a
    /// cluster to the client side of a cluster.
    ///
    /// See Section 2.4.1.1.3
    pub fn direction(self) -> bool {
        ((self.0 >> 3) & 0xb1) != 0
    }

    /// See Section 2.4.1.1.4
    pub fn disable_default_response(self) -> bool {
        ((self.0 >> 4) & 0xb1) != 0
    }
}

impl Debug for FrameControl {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("FrameControl")
            .field("frame_type", &self.frame_type())
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use crate::common::parse::PackBytes;

    use super::*;

    #[test]
    fn unpack_frame_control() {
        // given
        let input = [0x18];

        // when
        let frame_control = FrameControl::unpack_from_slice(&input).unwrap();

        // then
        assert!(!frame_control.frame_type());
        assert!(!frame_control.manufacturer_specific());
        assert!(frame_control.direction());
        assert!(frame_control.disable_default_response());
    }

    #[test]
    fn unpack_header_without_manufacturer_code() {
        // given
        let input = [0x18, 0x01, 0x0a, 0x00, 0x00, 0x29, 0x8a, 0x0b];

        // when
        let header = ZclHeader::unpack_from_slice(&input).unwrap();

        // then
        assert!(!header.frame_control.frame_type());
        assert_eq!(header.manufacturer_code, None);
        assert_eq!(header.sequence_number, 1);
        assert_eq!(
            header.command_identifier,
            CommandIdentifier::ReportAttributes
        );
    }
}
