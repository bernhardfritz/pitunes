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
import { Genre } from './models';
import { SearchComponent } from './SearchComponent';
import { TitleComponent } from './TitleComponent';
import { useGraphQLData } from './useGraphQLData';

export const GenresComponent = () => {
  const { data, refresh } = useGraphQLData(API.genres());

  const [editGenre, setEditGenre] = useState<Genre | null>(null);
  const openEditGenreDialog = Boolean(editGenre);

  const [deleteGenre, setDeleteGenre] = useState<Genre | null>(null);
  const openDeleteGenreDialog = Boolean(deleteGenre);

  const handleSubmitEditGenreDialog = async (event: any) => {
    event.preventDefault();

    if (editGenre === null) {
      return;
    }

    await fetcher(
      API.updateAlbum(editGenre.id, event.target.elements['name'].value)
    );
    setEditGenre(null);
    refresh();
  };

  const handleConfirmDeleteGenreDialog = async () => {
    if (deleteGenre === null) {
      return;
    }

    await fetcher(API.deleteGenre(deleteGenre.id));
    setDeleteGenre(null);
    refresh();
  };

  const [pattern, setPattern] = useState('');
  const [genreFuse, setGenreFuse] = useState<Fuse<Genre>>();
  const handleSearch = (pattern: string) => setPattern(pattern);
  useEffect(() => {
    if (data) {
      if (data.genres) {
        setGenreFuse(new Fuse(data.genres, { keys: ['name'] }));
      }
    }
  }, [data]);
  const genres = data?.genres ?? [];
  const filteredGenres =
    genreFuse !== undefined && pattern.length > 0
      ? genreFuse.search(pattern).map((result) => result.item)
      : genres;

  return data ? (
    <>
      <TitleComponent title="Genres"></TitleComponent>
      {genres.length > 0 ? (
        <>
          <SearchComponent onSearch={handleSearch}></SearchComponent>
          <List>
            {filteredGenres.map((genre: Genre) => (
              <ListItemLink
                key={genre.id}
                to={`/genres/${genre.id}`}
                primary={genre.name}
                menu={
                  <MenuComponent
                    items={[
                      {
                        key: 'edit',
                        name: 'Edit',
                        onClick: () => setEditGenre(genre),
                      },
                      {
                        key: 'delete',
                        name: 'Delete',
                        onClick: () => setDeleteGenre(genre),
                      },
                    ]}
                  ></MenuComponent>
                }
              />
            ))}
          </List>
          <FormDialogComponent
            open={openEditGenreDialog}
            onClose={() => setEditGenre(null)}
            onSubmit={handleSubmitEditGenreDialog}
            title="Edit genre"
            submit="Edit"
          >
            <TextField
              type="text"
              id="name"
              label="Name"
              defaultValue={editGenre?.name}
              autoFocus
            />
          </FormDialogComponent>
          <ConfirmationDialogComponent
            open={openDeleteGenreDialog}
            onClose={() => setDeleteGenre(null)}
            onConfirm={handleConfirmDeleteGenreDialog}
            title="Delete genre"
            confirm="Delete"
          >
            {deleteGenre?.name}
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
