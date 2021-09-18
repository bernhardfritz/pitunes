import {
  createStyles,
  fade,
  IconButton,
  InputAdornment,
  InputBase,
  makeStyles,
  Theme,
} from '@material-ui/core';
import ClearIcon from '@material-ui/icons/Clear';
import SearchIcon from '@material-ui/icons/Search';
import React, { useRef } from 'react';
import { useStateWithDispatchCallback } from './useStateWithDispatchCallback';

const useStyles = makeStyles((theme: Theme) =>
  createStyles({
    search: {
      position: 'relative',
      borderRadius: theme.shape.borderRadius,
      backgroundColor: fade(
        theme.palette.type === 'dark'
          ? theme.palette.common.white
          : theme.palette.grey[500],
        0.15
      ),
      '&:hover': {
        backgroundColor: fade(
          theme.palette.type === 'dark'
            ? theme.palette.common.white
            : theme.palette.grey[500],
          0.25
        ),
      },
      margin: theme.spacing(2),
      width: 'auto',
    },
    searchIcon: {
      padding: theme.spacing(0, 2),
      height: '100%',
      position: 'absolute',
      pointerEvents: 'none',
      display: 'flex',
      alignItems: 'center',
      justifyContent: 'center',
    },
    inputRoot: {
      color: 'inherit',
      width: '100%',
    },
    inputInput: {
      padding: theme.spacing(1, 1, 1, 0),
      // vertical padding + font size from searchIcon
      paddingLeft: `calc(1em + ${theme.spacing(4)}px)`,
      width: '100%',
    },
  })
);

type SearchComponentProps = {
  onSearch?: (pattern: string) => void;
};

export const SearchComponent = (props: SearchComponentProps) => {
  const classes = useStyles();
  const [pattern, setPattern] = useStateWithDispatchCallback('');
  const inputRef = useRef<HTMLInputElement>();
  const handleChange = (event: any) =>
    setPattern(event.target.value, (pattern) => {
      if (props.onSearch !== undefined) {
        props.onSearch(pattern);
      }
    });
  const handleClickClear = (event: any) =>
    setPattern('', (pattern) => {
      if (inputRef.current !== undefined) {
        inputRef.current.focus();
      }
      if (props.onSearch !== undefined) {
        props.onSearch(pattern);
      }
    });

  return (
    <div className={classes.search}>
      <div className={classes.searchIcon}>
        <SearchIcon />
      </div>
      <InputBase
        placeholder="Search"
        classes={{
          root: classes.inputRoot,
          input: classes.inputInput,
        }}
        inputProps={{ 'aria-label': 'search' }}
        value={pattern}
        onChange={handleChange}
        endAdornment={
          pattern.length > 0 && (
            <InputAdornment position="end">
              <IconButton aria-label="clear" onClick={handleClickClear}>
                <ClearIcon />
              </IconButton>
            </InputAdornment>
          )
        }
        inputRef={inputRef}
      />
    </div>
  );
};
