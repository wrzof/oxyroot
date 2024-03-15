use crate::factory_all_for_register_impl;
use crate::rbytes::rbuffer::RBuffer;
use crate::rbytes::Unmarshaler;
use crate::root::traits::Object;
use crate::rvers;

use crate::rcolors::Color;

pub(crate) struct AttMarker {
    color: Color,
    style: i16,
    width: f32,
}

impl Default for AttMarker {
    fn default() -> Self {
        AttMarker {
            color: Color::Int(1),
            style: 1,
            width: 1.0,
        }
    }
}

impl Unmarshaler for AttMarker {
    fn unmarshal(&mut self, r: &mut RBuffer) -> crate::rbytes::Result<()> {
        let hdr = r.read_header(self.class())?;

        if hdr.vers > rvers::ATT_MARKER {
            return Err(crate::rbytes::Error::VersionTooHigh {
                class: self.class().into(),
                version_read: hdr.vers,
                max_expected: rvers::ATT_MARKER,
            });
        }

        self.color = Color::from_i16(r.read_i16()?);

        self.style = r.read_i16()?;
        self.width = r.read_f32()?;
        r.check_header(&hdr)?;
        Ok(())
    }
}

factory_all_for_register_impl!(AttMarker, "TAttMarker");
