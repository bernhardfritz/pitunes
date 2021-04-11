import { FetcherParams } from 'graphiql/dist/components/GraphiQL';
import { FunctionComponent } from 'react';
import { useGraphQLData } from './useGraphQLData';

type GraphQLResourceProps = {
  fetcherParams: FetcherParams;
};

export const GraphQLResource: FunctionComponent<GraphQLResourceProps> = ({
  fetcherParams,
  children,
}) => {
  const { data, refresh } = useGraphQLData(fetcherParams);

  return data ? (children as any)({ data, refresh }) : null;
};
