import { FetcherParams } from 'graphiql/dist/components/GraphiQL';
import { FunctionComponent } from 'react';
import { Fetcher } from './fetcher';
import { useGraphQLData } from './useGraphQLData';

type GraphQLResourceProps = {
  fetcher: Fetcher;
  fetcherParams: FetcherParams;
};

export const GraphQLResource: FunctionComponent<GraphQLResourceProps> = ({
  fetcher,
  fetcherParams,
  children,
}) => {
  const data = useGraphQLData(fetcher, fetcherParams);

  return data ? (children as any)(data) : null;
};
