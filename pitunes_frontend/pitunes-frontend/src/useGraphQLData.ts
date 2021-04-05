import { FetcherParams } from 'graphiql/dist/components/GraphiQL';
import { useEffect, useState } from 'react';
import { Fetcher } from './fetcher';

export const useGraphQLData = (
  fetcher: Fetcher,
  fetcherParams: FetcherParams
) => {
  const [data, setData] = useState<any>(null);
  const refresh = () => {
    (async () => {
      const { data } = await fetcher(fetcherParams);
      setData(data);
    })();
  };
  useEffect(refresh, []);

  return { data, refresh };
};

export interface GraphQLData {
  data: any;
  refresh: () => void;
}
