INSERT INTO tracks(path, title, artist, album, album_artist, track, track_total, disc, disc_total, duration)
VALUES (:path, :title, :artist, :album, :album_artist, :track, :track_total, :disc, :disc_total, :duration)
ON CONFLICT(path) DO UPDATE SET
  title = excluded.title,
  artist = excluded.artist,
  album = excluded.album,
  album_artist = excluded.album_artist,
  track = excluded.track,
  track_total = excluded.track_total,
  disc = excluded.disc,
  disc_total = excluded.disc_total,
  duration = excluded.duration
RETURNING id;
