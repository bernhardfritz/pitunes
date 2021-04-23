import { List } from '@material-ui/core';
import React from 'react';
import { EmptyListComponent } from './EmptyListComponent';
import * as API from './graphql/api';
import { IdNameListItemLinks } from './IdNameListItemLinks';
import { LoadingComponent } from './LoadingComponent';
import { TitleComponent } from './TitleComponent';
import { useGraphQLData } from './useGraphQLData';

export const GenresComponent = () => {
  const { data } = useGraphQLData(API.genres());

  return data ? (
    <>
      <TitleComponent title="Genres"></TitleComponent>
      {data.genres && data.genres.length > 0 ? (
        <List>
          <IdNameListItemLinks
            items={data.genres}
            to={(id) => `/genres/${id}`}
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
