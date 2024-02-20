// SPDX-License-Identifier: MIT OR Apache-2.0
// Copyright © 2022 Adrian <adrian.eddy at gmail>

use std::{io::{ Read, Seek, Result, Cursor }, sync::Arc};
use byteorder::{ ReadBytesExt, WriteBytesExt, BigEndian };

use crate::{desc_reader, writer};

// We need to:
// - Merge mdat boxes
// - Sum         moov/mvhd/duration
// - Sum         moov/trak/tkhd/duration
// - Sum         moov/trak/mdia/mdhd/duration
// - Sum         moov/trak/edts/elst/segment duration
// - Merge lists moov/trak/mdia/minf/stbl/stts
// - Merge lists moov/trak/mdia/minf/stbl/stsz
// - Merge lists moov/trak/mdia/minf/stbl/stss
// - Merge lists moov/trak/mdia/minf/stbl/stco and co64
// - Rewrite stco to co64

pub const fn fourcc(s: &str) -> u32 {
    let s = s.as_bytes();
    (s[3] as u32) | ((s[2] as u32) << 8) | ((s[1] as u32) << 16) | ((s[0] as u32) << 24)
}
pub const fn has_children(typ: u32, is_read: bool) -> bool {
    typ == fourcc("moov") || typ == fourcc("trak") || typ == fourcc("edts") ||
    typ == fourcc("mdia") || typ == fourcc("minf") || typ == fourcc("stbl") ||
    (typ == fourcc("stsd") && is_read)
}

pub fn read_box<R: Read + Seek>(reader: &mut R) -> Result<(u32, u64, u64, i64)> {
    let pos = reader.stream_position()?;
    let size = reader.read_u32::<BigEndian>()?;
    let typ = reader.read_u32::<BigEndian>()?;
    if size == 1 {
        let largesize = reader.read_u64::<BigEndian>()?;
        Ok((typ, pos, largesize, 16))
    } else {
        Ok((typ, pos, size as u64, 8))
    }
}

pub fn join_buffers(buffers: Vec<Vec<u8>>) -> Result<Vec<u8>> {
    let mut desc = desc_reader::Desc::default();
    desc.moov_tracks.resize(10, Default::default());

    let buffers: Vec<_> = buffers.into_iter()
        .map(Arc::new)
        .collect();

    for (i, buffer) in buffers.iter().enumerate() {

        let mut buf_rw = Cursor::new(buffer.as_ref());

        { // Find mdat first
            while let Ok((typ, _, size, header_size)) = read_box(&mut buf_rw) {
                let org_pos = buf_rw.stream_position()?;
                if typ == fourcc("mdat") {
                    desc.mdat_position.push((None, org_pos, size - header_size as u64));
                    desc.mdat_final_position = org_pos;
                    break;
                }
                buf_rw.seek(std::io::SeekFrom::Start(org_pos + size - header_size as u64))?;
            }
            buf_rw.seek(std::io::SeekFrom::Start(0))?;
        }

        desc_reader::read_desc(&mut buf_rw, &mut desc, 0, u64::MAX, i)?;

        if let Some(mdat) = desc.mdat_position.last_mut() {
            mdat.0 = Some(buffer.clone());
            desc.mdat_offset += mdat.2;
            for t in &mut desc.moov_tracks {
                t.sample_offset = t.stsz_count;
                t.chunk_offset = t.stco.len() as u32;
            }
        }
    }

    // Write it to the file
    let mut buf_0 = Cursor::new(buffers[0].as_ref());
    let mut buf_out = Cursor::new(Vec::<u8>::new());

    writer::rewrite_from_desc(&mut buf_0, &mut buf_out, &mut desc, 0, u64::MAX)?;

    // Patch final mdat positions
    for track in &desc.moov_tracks {
        buf_out.seek(std::io::SeekFrom::Start(track.co64_final_position))?;
        for x in &track.stco {
            buf_out.write_u64::<BigEndian>(*x + desc.mdat_final_position)?;
        }
    }

    Ok(buf_out.into_inner())
}
