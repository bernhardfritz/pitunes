import React from 'react';
import { ListItemLink } from './ListItemLink';
import { IdName } from './models';

type IdNameListItemLinksProps = { items: IdName[]; to: (id: string) => string };

export const IdNameListItemLinks = ({
  items,
  to,
}: IdNameListItemLinksProps) => (
  <>
    {items.map((item) => (
      <ListItemLink key={item.id} to={to(item.id)} primary={item.name} />
    ))}
  </>
);
