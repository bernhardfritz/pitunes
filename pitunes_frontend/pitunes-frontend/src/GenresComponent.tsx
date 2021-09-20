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
import { Genre } from './models';
import { SearchableList } from './SearchableList';
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

  return data ? (
    <>
      <TitleComponent title="Genres"></TitleComponent>
      {data.genres && data.genres.length > 0 ? (
        <>
          <SearchableList
            items={data.genres}
            fuseOptions={{ keys: ['name'] }}
            render={(genre: Genre) => (
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
            )}
          ></SearchableList>
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
