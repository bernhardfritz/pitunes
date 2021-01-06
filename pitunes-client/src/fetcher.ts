import { FetcherParams } from 'graphiql/dist/components/GraphiQL';

export type FetcherResult = { data: any };
export type Fetcher = (graphQLParams: FetcherParams) => Promise<FetcherResult>;
