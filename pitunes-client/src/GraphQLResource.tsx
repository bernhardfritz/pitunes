import { FetcherParams } from 'graphiql/dist/components/GraphiQL';
import { FunctionComponent, useEffect, useState } from 'react';
import { Fetcher } from './fetcher';

type GraphQLResourceProps = {
  fetcher: Fetcher;
  fetcherParams: FetcherParams;
};

type GraphQLResourceState = {
  data: any;
};

export const GraphQLResource: FunctionComponent<GraphQLResourceProps> = ({
  fetcher,
  fetcherParams,
  children,
}) => {
  const [state, setState] = useState<GraphQLResourceState>({ data: null });
  useEffect(() => {
    (async () => {
      const { data } = await fetcher(fetcherParams);
      setState({ data });
    })();
  }, []);
  return state.data ? (children as any)(state.data) : null;
};
