import { List, TextField } from '@material-ui/core';
import Fuse from 'fuse.js';
import React, { useEffect, useState } from 'react';
import { ConfirmationDialogComponent } from './ConfirmationDialogComponent';
import { EmptyListComponent } from './EmptyListComponent';
import { FormDialogComponent } from './FormDialogComponent';
import * as API from './graphql/api';
import { fetcher } from './graphql/fetcher';
import { ListItemLink } from './ListItemLink';
import { LoadingComponent } from './LoadingComponent';
import { MenuComponent } from './MenuComponent';
import { Playlist } from './models';
import { SearchComponent } from './SearchComponent';
import { TitleComponent } from './TitleComponent';
import { useGraphQLData } from './useGraphQLData';

export const PlaylistsComponent = () => {
  const { data, refresh } = useGraphQLData(API.playlists());

  const [editPlaylist, setEditPlaylist] = useState<Playlist | null>(null);
  const openEditPlaylistDialog = Boolean(editPlaylist);

  const [deletePlaylist, setDeletePlaylist] = useState<Playlist | null>(null);
  const openDeletePlaylistDialog = Boolean(deletePlaylist);

  const handleSubmitEditPlaylistDialog = async (event: any) => {
    event.preventDefault();

    if (editPlaylist === null) {
      return;
    }

    await fetcher(
      API.updatePlaylist(editPlaylist.id, event.target.elements['name'].value)
    );
    setEditPlaylist(null);
    refresh();
  };

  const handleConfirmDeletePlaylistDialog = async () => {
    if (deletePlaylist === null) {
      return;
    }

    await fetcher(API.deletePlaylist(deletePlaylist.id));
    setDeletePlaylist(null);
    refresh();
  };

  const [pattern, setPattern] = useState('');
  const [playlistFuse, setPlaylistFuse] = useState<Fuse<Playlist>>();
  const handleSearch = (pattern: string) => setPattern(pattern);
  useEffect(() => {
    if (data) {
      if (data.playlists) {
        setPlaylistFuse(new Fuse(data.playlists, { keys: ['name'] }));
      }
    }
  }, [data]);
  const playlists = data?.playlists ?? [];
  const filteredPlaylists =
    playlistFuse !== undefined && pattern.length > 0
      ? playlistFuse.search(pattern).map((result) => result.item)
      : playlists;

  return data ? (
    <>
      <TitleComponent title="Playlists"></TitleComponent>
      {playlists.length > 0 ? (
        <>
          <SearchComponent onSearch={handleSearch}></SearchComponent>
          <List>
            {filteredPlaylists.map((playlist: Playlist) => (
              <ListItemLink
                key={playlist.id}
                to={`/playlists/${playlist.id}`}
                primary={playlist.name}
                menu={
                  <MenuComponent
                    items={[
                      {
                        key: 'edit',
                        name: 'Edit',
                        onClick: () => setEditPlaylist(playlist),
                      },
                      {
                        key: 'delete',
                        name: 'Delete',
                        onClick: () => setDeletePlaylist(playlist),
                      },
                    ]}
                  ></MenuComponent>
                }
              />
            ))}
          </List>
          <FormDialogComponent
            open={openEditPlaylistDialog}
            onClose={() => setEditPlaylist(null)}
            onSubmit={handleSubmitEditPlaylistDialog}
            title="Edit playlist"
            submit="Edit"
          >
            <TextField
              type="text"
              id="name"
              label="Name"
              defaultValue={editPlaylist?.name}
              autoFocus
            />
          </FormDialogComponent>
          <ConfirmationDialogComponent
            open={openDeletePlaylistDialog}
            onClose={() => setDeletePlaylist(null)}
            onConfirm={handleConfirmDeletePlaylistDialog}
            title="Delete playlist"
            confirm="Delete"
          >
            {deletePlaylist?.name}
          </ConfirmationDialogComponent>
        </>
      ) : (
        <EmptyListComponent />
      )}
    </>
  ) : (
    <LoadingComponent />
  );
};
