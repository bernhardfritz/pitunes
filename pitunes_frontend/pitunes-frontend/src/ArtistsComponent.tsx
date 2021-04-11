import { List } from '@material-ui/core';
import React from 'react';
import { EmptyListComponent } from './EmptyListComponent';
import { artists } from './graphql/api';
import { IdNameListItemLinks } from './IdNameListItemLinks';
import { LoadingComponent } from './LoadingComponent';
import { TitleComponent } from './TitleComponent';
import { useGraphQLData } from './useGraphQLData';

export const ArtistsComponent = () => {
  const { data } = useGraphQLData(artists());

  return data ? (
    <>
      <TitleComponent title="Artists"></TitleComponent>
      {data.artists && data.artists.length > 0 ? (
        <List>
          <IdNameListItemLinks
            items={data.artists}
            to={(id) => `/artists/${id}`}
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
