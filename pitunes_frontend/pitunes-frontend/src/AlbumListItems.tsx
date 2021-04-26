import { TextField } from '@material-ui/core';
import React, { useState } from 'react';
import { ConfirmationDialogComponent } from './ConfirmationDialogComponent';
import { FormDialogComponent } from './FormDialogComponent';
import * as API from './graphql/api';
import { fetcher } from './graphql/fetcher';
import { ListItemLink } from './ListItemLink';
import { MenuComponent } from './MenuComponent';
import { Album } from './models';

type AlbumListItemsProps = {
  albums: Album[];
  refresh?: () => void;
};

export const AlbumListItems = ({ albums, refresh }: AlbumListItemsProps) => {
  const [editAlbum, setEditAlbum] = useState<Album | null>(null);
  const openEditAlbumDialog = Boolean(editAlbum);

  const [deleteAlbum, setDeleteAlbum] = useState<Album | null>(null);
  const openDeleteAlbumDialog = Boolean(deleteAlbum);

  const handleSubmitEditAlbumDialog = async (event: any) => {
    event.preventDefault();

    if (editAlbum === null) {
      return;
    }

    await fetcher(
      API.updateAlbum(editAlbum.id, event.target.elements['name'].value)
    );
    setEditAlbum(null);

    if (refresh !== undefined) {
      refresh();
    }
  };

  const handleConfirmDeleteAlbumDialog = async () => {
    if (deleteAlbum === null) {
      return;
    }

    await fetcher(API.deleteAlbum(deleteAlbum.id));
    setDeleteAlbum(null);

    if (refresh !== undefined) {
      refresh();
    }
  };

  return (
    <>
      {albums && albums.length > 0 && (
        <>
          {albums.map((album: Album) => (
            <ListItemLink
              key={album.id}
              to={`/albums/${album.id}`}
              primary={album.name}
              menu={
                <MenuComponent
                  items={[
                    {
                      name: 'Edit',
                      onClick: () => setEditAlbum(album),
                    },
                    {
                      name: 'Delete',
                      onClick: () => setDeleteAlbum(album),
                    },
                  ]}
                ></MenuComponent>
              }
            />
          ))}
          <FormDialogComponent
            open={openEditAlbumDialog}
            onClose={() => setEditAlbum(null)}
            onSubmit={handleSubmitEditAlbumDialog}
            title="Edit album"
            submit="Edit"
          >
            <TextField
              type="text"
              id="name"
              label="Name"
              defaultValue={editAlbum?.name}
              autoFocus
            />
          </FormDialogComponent>
          <ConfirmationDialogComponent
            open={openDeleteAlbumDialog}
            onClose={() => setDeleteAlbum(null)}
            onConfirm={handleConfirmDeleteAlbumDialog}
            title="Delete album"
            confirm="Delete"
          >
            {deleteAlbum?.name}
          </ConfirmationDialogComponent>
        </>
      )}
    </>
  );
};
