import Fuse from 'fuse.js';
import React, { useEffect, useState } from 'react';

type FilterableItemsProps<T> = {
  items: T[];
  fuseOptions: Fuse.IFuseOptions<T>;
  pattern: string;
  render: (item: T, index: number) => JSX.Element;
};

export const FilterableItems = <T,>({
  items,
  fuseOptions,
  pattern,
  render,
}: FilterableItemsProps<T>) => {
  const [fuse, setFuse] = useState<Fuse<T>>();
  useEffect(() => {
    setFuse(new Fuse(items, fuseOptions));
  }, [items, fuseOptions]);
  return (
    <>
      {fuse !== undefined && pattern.length > 0
        ? fuse
            .search(pattern)
            .map((result) => render(result.item, result.refIndex))
        : items.map(render)}
    </>
  );
};
