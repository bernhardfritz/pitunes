import { createStyles, makeStyles, Theme, Typography } from '@material-ui/core';
import React, { useRef, useState } from 'react';
import CloudUploadIcon from '@material-ui/icons/CloudUpload';

const useStyles = makeStyles((theme: Theme) =>
  createStyles({
    wrapper: (props: UploadDropZoneComponentProps) => ({
      display: 'flex',
      border: `${theme.spacing(1)}px dashed ${
        theme.palette.type === 'dark'
          ? theme.palette.grey[800]
          : theme.palette.grey[300]
      }`,
      padding: theme.spacing(3),
      height: `calc(100vh - ${
        (props.playerVisible ? 2 : 1) * Number(theme.mixins.toolbar.minHeight)
      }px)`,
      [theme.breakpoints.up('sm')]: {
        height: `calc(100vh - ${
          (props.playerVisible ? 2 : 1) *
          Number(
            (theme.mixins.toolbar[theme.breakpoints.up('sm')] as any).minHeight
          )
        }px)`,
      },
    }),
    solidBorder: {
      borderStyle: 'solid !important',
      '&container': {
        opacity: 0.5,
      },
    },
    container: {
      display: 'flex',
      flexDirection: 'column',
      alignItems: 'center',
      margin: 'auto',
    },
    semitransparent: {
      opacity: 0.5,
    },
    iconWrapper: {
      display: 'flex',
      width: 96,
      height: 96,
      borderRadius: 48,
      background:
        theme.palette.type === 'dark'
          ? theme.palette.grey[800]
          : theme.palette.grey[300],
      marginBottom: theme.spacing(3),
    },
    icon: {
      width: 48,
      height: 48,
      margin: 'auto',
      fill: theme.palette.common.white,
    },
  })
);

type UploadDropZoneComponentProps = {
  playerVisible: boolean;
  onUpload?: (fileList: any) => void;
  onClick?: React.MouseEventHandler<HTMLDivElement>;
  onDrop?: React.DragEventHandler<HTMLDivElement>;
  onDragOver?: React.DragEventHandler<HTMLDivElement>;
};

type UploadDropZoneComponentState = {
  inDropZone: boolean;
};

export const UploadDropZoneComponent = (
  props: UploadDropZoneComponentProps
) => {
  const inputEl = useRef<HTMLInputElement>(null);
  const classes = useStyles(props);
  const [state, setState] = useState<UploadDropZoneComponentState>({
    inDropZone: false,
  });

  const handleChange = (event: any) => {
    if (props.onUpload === undefined) {
      return;
    }
    props.onUpload(event.target.files);
  };

  const handleClick = (event: any) => {
    inputEl.current?.click();
  };

  const handleDrop = (event: any) => {
    event.preventDefault();
    event.stopPropagation();
    if (props.onUpload === undefined) {
      return;
    }
    props.onUpload(event.dataTransfer.files);
  };

  const handleDragOver = (event: any) => {
    event.preventDefault();
    event.stopPropagation();

    event.dataTransfer.dropEffect = 'copy';
    setState({ inDropZone: true });
  };

  const handleDragLeave = (event: any) => {
    event.preventDefault();
    event.stopPropagation();

    setState({ inDropZone: false });
  };

  return (
    <>
      <input
        type="file"
        ref={inputEl}
        onChange={handleChange}
        multiple
        hidden
      ></input>
      <div
        className={`${classes.wrapper} ${
          state.inDropZone ? classes.solidBorder : ''
        }`}
        onClick={handleClick}
        onDrop={handleDrop}
        onDragOver={handleDragOver}
        onDragLeave={handleDragLeave}
      >
        <div
          className={`${classes.container} ${
            state.inDropZone ? classes.semitransparent : ''
          }`}
        >
          <div className={classes.iconWrapper}>
            <CloudUploadIcon className={classes.icon}></CloudUploadIcon>
          </div>
          <Typography variant="subtitle1" color="textSecondary" component="div">
            Drop files here or click to upload.
          </Typography>
        </div>
      </div>
    </>
  );
};
