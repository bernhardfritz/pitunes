// eslint-disable-next-line import/no-webpack-loader-syntax
import GenresQuery from '!!raw-loader!./graphql/GenresQuery.graphql';
import { List } from '@material-ui/core';
import React from 'react';
import { AppContext } from './App';
import { EmptyListComponent } from './EmptyListComponent';
import { GraphQLResource } from './GraphQLResource';
import { IdNameListItemLinks } from './IdNameListItemLinks';

export const GenresComponent = () => (
  <AppContext.Consumer>
    {({ fetcher }) => (
      <GraphQLResource
        fetcher={fetcher}
        fetcherParams={{
          query: GenresQuery,
          operationName: 'GenresQuery',
        }}
      >
        {(data: any) =>
          data.genres ? (
            <List>
              <IdNameListItemLinks
                items={data.genres}
                to={(id) => `/genres/${id}`}
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
