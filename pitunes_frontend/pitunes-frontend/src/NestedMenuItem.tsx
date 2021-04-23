import { createStyles, makeStyles, Theme } from '@material-ui/core';
import Menu from '@material-ui/core/Menu';
import MenuItem from '@material-ui/core/MenuItem';
import ChevronRightIcon from '@material-ui/icons/ChevronRight';
import React, { FunctionComponent, useRef, useState } from 'react';

const useStyles = makeStyles((theme: Theme) =>
  createStyles({
    chevronRight: {
      marginLeft: 'auto',
    },
  })
);

type NestedMenuItemProps = {
  label: string;
  parentMenuOpen: boolean;
  left?: boolean;
};

const ITEM_HEIGHT = 48;

export const NestedMenuItem: FunctionComponent<NestedMenuItemProps> = (
  props
) => {
  const classes = useStyles();
  const [open, setOpen] = useState(false);
  const nestedMenuRef = useRef<any>(null);

  const handleMouseEnter = (event: any) => {
    event.stopPropagation();
    setOpen(true);
  };

  const handleMouseLeave = (event: any) => {
    setOpen(false);
  };

  const handleClick = (event: any) => {
    event.stopPropagation();
    setOpen((prevOpen) => !prevOpen);
  };

  return (
    <MenuItem
      ref={nestedMenuRef}
      onMouseEnter={handleMouseEnter}
      onMouseLeave={handleMouseLeave}
      onClick={handleClick}
    >
      {props.label}
      <ChevronRightIcon className={classes.chevronRight} />
      <Menu
        // set to pointerEvents to none to prevent menu from capturing
        // events meant for child elements
        style={{ pointerEvents: 'none' }}
        anchorEl={nestedMenuRef.current}
        anchorOrigin={{
          vertical: 'top',
          horizontal: props.left ? 'left' : 'right',
        }}
        transformOrigin={{
          vertical: 'top',
          horizontal: props.left ? 'right' : 'left',
        }}
        open={open && props.parentMenuOpen}
        onClose={() => {
          setOpen(false);
        }}
        PaperProps={{
          style: {
            maxHeight: ITEM_HEIGHT * 4.5,
            width: '20ch',
          },
        }}
      >
        <div style={{ pointerEvents: 'auto' }}>{props.children}</div>
      </Menu>
    </MenuItem>
  );
};
