use ms_pdb::Pdb;
use std::io::Write;
use std::path::PathBuf;

#[derive(clap::Parser)]
pub struct Options {
    /// The PDB to read.
    source_pdb: PathBuf,

    /// The PDB to write.
    dest_pdb: PathBuf,
}

pub fn copy_command(options: &Options) -> anyhow::Result<()> {
    let src = Pdb::open(&options.source_pdb)?;
    let mut dst = ms_pdb::msf::Msf::create(&options.dest_pdb, Default::default())?;

    for stream_index in 1..src.num_streams() {
        if src.is_stream_valid(stream_index) {
            let stream_data = src.read_stream_to_vec(stream_index)?;
            let mut s = dst.write_stream(stream_index)?;
            s.write_all(&stream_data)?;
        }
    }

    dst.commit()?;

    Ok(())
}
