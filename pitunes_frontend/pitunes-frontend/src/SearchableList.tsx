import { List } from '@material-ui/core';
import Fuse from 'fuse.js';
import React, { useState } from 'react';
import { FilterableItems } from './FilterableItems';
import { SearchComponent } from './SearchComponent';

type SearchableListProps<T> = {
  items: T[];
  fuseOptions: Fuse.IFuseOptions<T>;
  render: (item: T) => JSX.Element;
};

export const SearchableList = <T,>({
  items,
  fuseOptions,
  render,
}: SearchableListProps<T>) => {
  const [pattern, setPattern] = useState('');
  const handleSearch = (pattern: string) => setPattern(pattern);
  return (
    <>
      <SearchComponent onSearch={handleSearch}></SearchComponent>
      <List>
        <FilterableItems
          items={items}
          fuseOptions={fuseOptions}
          pattern={pattern}
          render={render}
        ></FilterableItems>
      </List>
    </>
  );
};
