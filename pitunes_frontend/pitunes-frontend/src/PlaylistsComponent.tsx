import { TextField } from '@material-ui/core';
import React, { useState } from 'react';
import { ConfirmationDialogComponent } from './ConfirmationDialogComponent';
import { EmptyListComponent } from './EmptyListComponent';
import { FormDialogComponent } from './FormDialogComponent';
import * as API from './graphql/api';
import { fetcher } from './graphql/fetcher';
import { ListItemLink } from './ListItemLink';
import { LoadingComponent } from './LoadingComponent';
import { MenuComponent } from './MenuComponent';
import { Playlist } from './models';
import { SearchableList } from './SearchableList';
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

  return data ? (
    <>
      <TitleComponent title="Playlists"></TitleComponent>
      {data.playlists && data.playlists.length > 0 ? (
        <>
          <SearchableList
            items={data.playlists}
            fuseOptions={{ keys: ['name'] }}
            render={(playlist: Playlist) => (
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
            )}
          ></SearchableList>
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
