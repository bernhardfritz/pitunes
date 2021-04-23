import { FetcherParams } from 'graphiql/dist/components/GraphiQL';

type FetcherResult = { data: any };

type Fetcher = (graphQLParams: FetcherParams) => Promise<FetcherResult>;

export const fetcher: Fetcher = (graphQLParams: FetcherParams) =>
  fetch('/api/graphql', {
    method: 'post',
    headers: {
      'Content-Type': 'application/json',
    },
    body: JSON.stringify(graphQLParams),
  }).then((response) => response.json());
