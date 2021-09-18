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
import { Artist } from './models';
import { SearchComponent } from './SearchComponent';
import { TitleComponent } from './TitleComponent';
import { useGraphQLData } from './useGraphQLData';

export const ArtistsComponent = () => {
  const { data, refresh } = useGraphQLData(API.artists());

  const [editArtist, setEditArtist] = useState<Artist | null>(null);
  const openEditArtistDialog = Boolean(editArtist);

  const [deleteArtist, setDeleteArtist] = useState<Artist | null>(null);
  const openDeleteArtistDialog = Boolean(deleteArtist);

  const handleSubmitEditArtistDialog = async (event: any) => {
    event.preventDefault();

    if (editArtist === null) {
      return;
    }

    await fetcher(
      API.updateArtist(editArtist.id, event.target.elements['name'].value)
    );
    setEditArtist(null);
    refresh();
  };

  const handleConfirmDeleteArtistDialog = async () => {
    if (deleteArtist === null) {
      return;
    }

    await fetcher(API.deleteArtist(deleteArtist.id));
    setDeleteArtist(null);
    refresh();
  };

  const [pattern, setPattern] = useState('');
  const [artistFuse, setArtistFuse] = useState<Fuse<Artist>>();
  const handleSearch = (pattern: string) => setPattern(pattern);
  useEffect(() => {
    if (data) {
      if (data.artists) {
        setArtistFuse(new Fuse(data.artists, { keys: ['name'] }));
      }
    }
  }, [data]);
  const artists = data?.artists ?? [];
  const filteredArtists =
    artistFuse !== undefined && pattern.length > 0
      ? artistFuse.search(pattern).map((result) => result.item)
      : artists;

  return data ? (
    <>
      <TitleComponent title="Artists"></TitleComponent>
      {artists.length > 0 ? (
        <>
          <SearchComponent onSearch={handleSearch}></SearchComponent>
          <List>
            {filteredArtists.map((artist: Artist) => (
              <ListItemLink
                key={artist.id}
                to={`/artists/${artist.id}`}
                primary={artist.name}
                menu={
                  <MenuComponent
                    items={[
                      {
                        key: 'edit',
                        name: 'Edit',
                        onClick: () => setEditArtist(artist),
                      },
                      {
                        key: 'delete',
                        name: 'Delete',
                        onClick: () => setDeleteArtist(artist),
                      },
                    ]}
                  ></MenuComponent>
                }
              />
            ))}
          </List>
          <FormDialogComponent
            open={openEditArtistDialog}
            onClose={() => setEditArtist(null)}
            onSubmit={handleSubmitEditArtistDialog}
            title="Edit artist"
            submit="Edit"
          >
            <TextField
              type="text"
              id="name"
              label="Name"
              defaultValue={editArtist?.name}
              autoFocus
            />
          </FormDialogComponent>
          <ConfirmationDialogComponent
            open={openDeleteArtistDialog}
            onClose={() => setDeleteArtist(null)}
            onConfirm={handleConfirmDeleteArtistDialog}
            title="Delete artist"
            confirm="Delete"
          >
            {deleteArtist?.name}
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
