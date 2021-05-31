import {
  createStyles,
  Divider,
  IconButton,
  makeStyles,
  Menu,
  MenuItem,
  Theme,
} from '@material-ui/core';
import MoreVertIcon from '@material-ui/icons/MoreVert';
import React, { useState } from 'react';
import { NestedMenuItem } from './NestedMenuItem';
import { orNbsp } from './orNbsp';

const useStyles = makeStyles((theme: Theme) =>
  createStyles({
    divider: {
      marginTop: 6,
      marginBottom: 6,
    },
  })
);

type MenuItemType = {
  key?: string;
  name: string;
  onClick: () => void;
};

type NestedMenuItemType = {
  key?: string;
  name: string;
  items: (MenuItemType | DividerType)[];
};

type DividerType = {
  key?: string;
};

const isMenuItemType = (
  item: MenuItemType | NestedMenuItemType | DividerType
): item is MenuItemType => (item as any).onClick;

const isNestedMenuItemType = (
  item: MenuItemType | NestedMenuItemType | DividerType
): item is NestedMenuItemType => (item as any).items;

type MenuComponentProps = {
  items: (MenuItemType | NestedMenuItemType | DividerType)[];
};

export const MenuComponent = ({ items }: MenuComponentProps) => {
  const classes = useStyles();
  const [anchorEl, setAnchorEl] = useState<HTMLElement | null>(null);
  const open = Boolean(anchorEl);

  const handleClick = (event: React.MouseEvent<HTMLElement>) => {
    setAnchorEl(event.currentTarget);
  };

  const handleClose = () => {
    setAnchorEl(null);
  };

  return (
    <>
      <IconButton edge="end" onClick={handleClick}>
        <MoreVertIcon />
      </IconButton>
      <Menu anchorEl={anchorEl} open={open} onClose={handleClose}>
        {items.map((item: MenuItemType | NestedMenuItemType | DividerType) =>
          isMenuItemType(item) ? (
            <MenuItem
              key={item.key}
              onClick={() => {
                handleClose();
                item.onClick();
              }}
            >
              {orNbsp(item.name)}
            </MenuItem>
          ) : isNestedMenuItemType(item) ? (
            <NestedMenuItem
              key={item.key}
              label={item.name}
              parentMenuOpen={open}
              left
            >
              {item.items.map((nestedItem: MenuItemType | DividerType) =>
                isMenuItemType(nestedItem) ? (
                  <MenuItem
                    key={nestedItem.key}
                    onClick={() => {
                      handleClose();
                      nestedItem.onClick();
                    }}
                  >
                    {orNbsp(nestedItem.name)}
                  </MenuItem>
                ) : (
                  <Divider key={nestedItem.key} variant="middle" className={classes.divider} />
                )
              )}
            </NestedMenuItem>
          ) : (
            <Divider key={item.key} variant="middle" className={classes.divider} />
          )
        )}
      </Menu>
    </>
  );
};
