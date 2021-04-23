import { List } from '@material-ui/core';
import React from 'react';
import { EmptyListComponent } from './EmptyListComponent';
import * as API from './graphql/api';
import { IdNameListItemLinks } from './IdNameListItemLinks';
import { LoadingComponent } from './LoadingComponent';
import { TitleComponent } from './TitleComponent';
import { useGraphQLData } from './useGraphQLData';

export const AlbumsComponent = () => {
  const { data } = useGraphQLData(API.albums());

  return data ? (
    <>
      <TitleComponent title="Albums"></TitleComponent>
      {data.albums && data.albums.length ? (
        <List>
          <IdNameListItemLinks
            items={data.albums}
            to={(id) => `/albums/${id}`}
          />
        </List>
      ) : (
        <EmptyListComponent />
      )}
    </>
  ) : (
    <LoadingComponent />
  );
};
