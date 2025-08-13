use std::{ffi::OsStr, path::PathBuf, time::Duration};

use lofty::file::{AudioFile, TaggedFileExt};
use walkdir::WalkDir;

#[allow(unused)]
#[derive(Default, Clone, Debug, PartialEq, Eq)]
pub struct Track {
    pub path: PathBuf,
    pub title: Option<String>,
    pub album: Option<String>,
    pub album_artist: Option<String>,
    pub artist: Option<String>,
    pub duration: Option<Duration>,
    pub disc: Option<String>,
    pub disc_total: Option<String>,
    pub track: Option<String>,
    pub track_total: Option<String>,
    pub cover: Option<Vec<u8>>,
}

impl Track {
    pub fn read_front_cover(&self) -> Result<Option<Vec<u8>>, lofty::error::LoftyError> {
        let path = self.path.as_path();

        Ok(lofty::read_from_path(path)?.primary_tag().and_then(|tag| {
            tag.get_picture_type(lofty::picture::PictureType::CoverFront)
                .or_else(|| tag.pictures().first())
                .map(|pic| pic.data().to_owned())
        }))
    }
}

/// Scans the given path for music files and reads their metadata.
///
/// This function recursively traverses directories, collecting `TrackInfo` for supported
/// music file types (`.flac`, `.wav`, `.mp3`).
///
/// # Arguments
///
/// * `path` - The starting path to scan. This can be a file or a directory.
///
/// # Returns
///
/// A `Result` which is:
/// - `Vec<PathBuf>` containing a list of `PathBuf` for all music files found.
pub fn scan_tracks(path: &std::path::Path) -> Vec<PathBuf> {
    let walker = WalkDir::new(path)
        .follow_links(true)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|entry| {
            entry.file_type().is_file()
                && matches!(
                    entry.path().extension().and_then(OsStr::to_str),
                    Some("flac" | "mp3" | "wav")
                )
        });
    walker.map(|entry| entry.path().to_owned()).collect()
}

/// Reads metadata from a music file.
///
/// This function attempts to read metadata from file
/// using the `lofty` crate.
///
/// # Arguments
///
/// * `path` - The path to the music file.
///
/// # Returns
///
/// A `Result<Option<Track>, lofty::error::LoftyError>`:
/// - `Ok(Track)` if the file can be read by lofty.
/// - `Err(lofty::error::LoftyError)` if an error occurred while reading the music file
///   or its tags
pub fn read_track_metadata(
    path: &std::path::Path,
) -> std::result::Result<Track, lofty::error::LoftyError> {
    let tagged = lofty::read_from_path(path)?;

    Ok(tagged.primary_tag().map_or_else(
        || Track {
            path: path.to_owned(),
            ..Default::default()
        },
        |tag| Track {
            path: path.to_owned(),
            title: tag
                .get_string(&lofty::tag::ItemKey::TrackTitle)
                .map(String::from),
            album: tag
                .get_string(&lofty::tag::ItemKey::AlbumTitle)
                .map(String::from),
            album_artist: tag
                .get_string(&lofty::tag::ItemKey::AlbumArtist)
                .map(String::from),
            artist: tag
                .get_string(&lofty::tag::ItemKey::TrackArtist)
                .map(String::from),
            disc: tag
                .get_string(&lofty::tag::ItemKey::DiscNumber)
                .map(String::from),
            disc_total: tag
                .get_string(&lofty::tag::ItemKey::DiscTotal)
                .map(String::from),
            track: tag
                .get_string(&lofty::tag::ItemKey::TrackNumber)
                .map(String::from),
            track_total: tag
                .get_string(&lofty::tag::ItemKey::TrackTotal)
                .map(String::from),
            duration: Some(tagged.properties().duration()),
            cover: None,
        },
    ))
}
