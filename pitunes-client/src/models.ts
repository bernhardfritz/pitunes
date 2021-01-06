export type IdName = {
  id: string;
  name: string;
};

export type Album = IdName;

export type Artist = IdName;

export type Genre = IdName;

export type Playlist = IdName;

export type Track = IdName & {
  duration: number;
  album?: Album;
  artist?: Artist;
  genre?: Artist;
  trackNumber?: number;
};
