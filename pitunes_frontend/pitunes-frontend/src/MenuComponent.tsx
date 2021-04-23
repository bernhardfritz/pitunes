import { IconButton, Menu, MenuItem } from '@material-ui/core';
import MoreVertIcon from '@material-ui/icons/MoreVert';
import React, { useState } from 'react';
import { NestedMenuItem } from './NestedMenuItem';

type MenuItemType = {
  name: string;
  onClick: () => void;
};

type NestedMenuItemType = {
  name: string;
  items: MenuItemType[];
};

const isNestedMenuItemType = (
  item: MenuItemType | NestedMenuItemType
): item is NestedMenuItemType => (item as any).items;

type MenuComponentProps = {
  items: (MenuItemType | NestedMenuItemType)[];
};

export const MenuComponent = ({ items }: MenuComponentProps) => {
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
        {items.map((item: MenuItemType | NestedMenuItemType) =>
          isNestedMenuItemType(item) ? (
            <NestedMenuItem label={item.name} parentMenuOpen={open} left>
              {item.items.map((nestedItem: MenuItemType) => (
                <MenuItem
                  onClick={() => {
                    handleClose();
                    nestedItem.onClick();
                  }}
                >
                  {nestedItem.name}
                </MenuItem>
              ))}
            </NestedMenuItem>
          ) : (
            <MenuItem
              onClick={() => {
                handleClose();
                item.onClick();
              }}
            >
              {item.name}
            </MenuItem>
          )
        )}
      </Menu>
    </>
  );
};
