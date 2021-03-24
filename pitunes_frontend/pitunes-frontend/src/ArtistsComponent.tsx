// eslint-disable-next-line import/no-webpack-loader-syntax
import ArtistsQuery from '!!raw-loader!./graphql/ArtistsQuery.graphql';
import { List } from '@material-ui/core';
import React from 'react';
import { AppContext } from './App';
import { EmptyListComponent } from './EmptyListComponent';
import { GraphQLResource } from './GraphQLResource';
import { IdNameListItemLinks } from './IdNameListItemLinks';

export const ArtistsComponent = () => (
  <AppContext.Consumer>
    {({ fetcher }) => (
      <GraphQLResource
        fetcher={fetcher}
        fetcherParams={{
          query: ArtistsQuery,
          operationName: 'ArtistsQuery',
        }}
      >
        {(data: any) =>
          data.artists ? (
            <List>
              <IdNameListItemLinks
                items={data.artists}
                to={(id) => `/artists/${id}`}
              />
            </List>
          ) : (
            <EmptyListComponent />
          )
        }
      </GraphQLResource>
    )}
  </AppContext.Consumer>
);
