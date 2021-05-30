import {
  Collapse,
  ListItem,
  ListItemIcon,
  ListItemText,
  useScrollTrigger,
} from '@material-ui/core';
import AppBar from '@material-ui/core/AppBar';
import CssBaseline from '@material-ui/core/CssBaseline';
import Divider from '@material-ui/core/Divider';
import Drawer from '@material-ui/core/Drawer';
import Hidden from '@material-ui/core/Hidden';
import IconButton from '@material-ui/core/IconButton';
import List from '@material-ui/core/List';
import {
  createStyles,
  makeStyles,
  Theme,
  useTheme,
} from '@material-ui/core/styles';
import Toolbar from '@material-ui/core/Toolbar';
import Typography from '@material-ui/core/Typography';
import Album from '@material-ui/icons/Album';
import Audiotrack from '@material-ui/icons/Audiotrack';
import Category from '@material-ui/icons/Category';
import ExpandLess from '@material-ui/icons/ExpandLess';
import ExpandMore from '@material-ui/icons/ExpandMore';
import LibraryMusicIcon from '@material-ui/icons/LibraryMusic';
import MenuIcon from '@material-ui/icons/Menu';
import Mic from '@material-ui/icons/Mic';
import PublishIcon from '@material-ui/icons/Publish';
import QueueMusic from '@material-ui/icons/QueueMusic';
import StorageIcon from '@material-ui/icons/Storage';
import React, { FunctionComponent, useRef } from 'react';
import { Link } from 'react-router-dom';
import { CSSTransition } from 'react-transition-group';
import { ListItemLink } from './ListItemLink';

export const drawerWidth = 240;

const useStyles = makeStyles((theme: Theme) =>
  createStyles({
    root: {
      display: 'flex',
      height: '100vh',
    },
    drawer: {
      [theme.breakpoints.up('sm')]: {
        width: drawerWidth,
        flexShrink: 0,
      },
    },
    appBar: {
      transform: 'translateY(0)',
      transition: 'transform 300ms',
      [theme.breakpoints.up('sm')]: {
        width: `calc(100% - ${drawerWidth}px)`,
        marginLeft: drawerWidth,
      },
      color: theme.palette.text.primary,
      backgroundColor: theme.palette.background.paper,
    },
    appBarEnterDone: {
      transform: `translateY(-${theme.mixins.toolbar.minHeight}px)`,
      [theme.breakpoints.up('sm')]: {
        transform: `translateY(-${
          (theme.mixins.toolbar[theme.breakpoints.up('sm')] as any).minHeight
        }px)`,
      },
    },
    menuButton: {
      marginRight: theme.spacing(2),
      [theme.breakpoints.up('sm')]: {
        display: 'none',
      },
    },
    // necessary for content to be below app bar
    toolbar: theme.mixins.toolbar,
    drawerPaper: {
      width: drawerWidth,
    },
    content: {
      flexGrow: 1,
      // padding: theme.spacing(3),
      display: 'flex',
      flexFlow: 'column',
      overflowX: 'hidden',
      overflowY: 'auto',
    },
    logoContainer: {
      display: 'flex',
      justifyContent: 'center',
      alignItems: 'center',
      minHeight: theme.mixins.toolbar.minHeight,
      [theme.breakpoints.up('sm')]: {
        minHeight: (theme.mixins.toolbar[theme.breakpoints.up('sm')] as any)
          .minHeight,
      },
    },
    logoLink: {
      position: 'relative',
      display: 'flex',
      justifyContent: 'center',
      alignItems: 'center',
      color: 'unset',
      textDecoration: 'unset',
    },
    logo: {
      height: '3rem',
    },
    logoText: {
      fontSize: '1.5rem',
    },
    tabs: {
      minHeight: 48,
    },
    nested: {
      paddingLeft: theme.spacing(4),
    },
  })
);

type ResponsiveDrawerProps = { title: string; tabs: any };

export const ResponsiveDrawer: FunctionComponent<ResponsiveDrawerProps> = (
  props
) => {
  const classes = useStyles();
  const theme = useTheme();
  const mainEl = useRef(null);
  const trigger = useScrollTrigger({ target: mainEl.current ?? undefined });
  const [mobileOpen, setMobileOpen] = React.useState(false);
  const [libraryOpen, setLibraryOpen] = React.useState(false);

  const handleDrawerToggle = () => {
    setMobileOpen(!mobileOpen);
  };

  const handleClick = () => {
    setMobileOpen(false);
  };

  const drawer = (
    <div>
      <div className={classes.toolbar}>
        <div className={classes.logoContainer}>
          <Link to="/" className={classes.logoLink} onClick={handleClick}>
            <img src="/logo192.png" className={classes.logo} />
            <span>
              <span className={classes.logoText}>piTunes</span>
              <sub>
                {(process.env.REACT_APP_VERSION &&
                  `v${process.env.REACT_APP_VERSION}`) ||
                  'dev'}
              </sub>
            </span>
          </Link>
        </div>
      </div>
      <Divider />
      <List>
        <ListItem button onClick={() => setLibraryOpen(!libraryOpen)}>
          <ListItemIcon>
            <LibraryMusicIcon />
          </ListItemIcon>
          <ListItemText primary="Library" />
          {libraryOpen ? <ExpandLess /> : <ExpandMore />}
        </ListItem>
        <Collapse in={libraryOpen} timeout="auto" unmountOnExit>
          <List component="div" disablePadding>
            <ListItemLink
              to="/playlists"
              primary="Playlists"
              icon={<QueueMusic />}
              onClick={handleClick}
              className={classes.nested}
            />
            <ListItemLink
              to="/artists"
              primary="Artists"
              icon={<Mic />}
              onClick={handleClick}
              className={classes.nested}
            />
            <ListItemLink
              to="/albums"
              primary="Albums"
              icon={<Album />}
              onClick={handleClick}
              className={classes.nested}
            />
            <ListItemLink
              to="/genres"
              primary="Genres"
              icon={<Category />}
              onClick={handleClick}
              className={classes.nested}
            />
            <ListItemLink
              to="/tracks"
              primary="Tracks"
              icon={<Audiotrack />}
              onClick={handleClick}
              className={classes.nested}
            />
          </List>
        </Collapse>
      </List>
      <Divider />
      <List>
        <ListItemLink
          to="/upload"
          primary="Upload"
          icon={<PublishIcon />}
          onClick={handleClick}
        ></ListItemLink>
        <ListItemLink
          to="/graphiql"
          primary="GraphiQL"
          icon={<StorageIcon />}
          onClick={handleClick}
        ></ListItemLink>
      </List>
    </div>
  );

  const container =
    window !== undefined ? () => window.document.body : undefined;

  return (
    <div className={classes.root}>
      <CssBaseline />
      <CSSTransition
        in={trigger}
        timeout={300}
        classNames={{
          enterDone: classes.appBarEnterDone,
        }}
      >
        <AppBar position="fixed" className={classes.appBar}>
          <Toolbar>
            <IconButton
              color="inherit"
              aria-label="open drawer"
              edge="start"
              onClick={handleDrawerToggle}
              className={classes.menuButton}
            >
              <MenuIcon />
            </IconButton>
            <Typography variant="h6" noWrap>
              {props.title}
            </Typography>
          </Toolbar>
          {props.tabs}
        </AppBar>
      </CSSTransition>
      <nav className={classes.drawer} aria-label="mailbox folders">
        {/* The implementation can be swapped with js to avoid SEO duplication of links. */}
        <Hidden smUp implementation="css">
          <Drawer
            container={container}
            variant="temporary"
            anchor={theme.direction === 'rtl' ? 'right' : 'left'}
            open={mobileOpen}
            onClose={handleDrawerToggle}
            classes={{
              paper: classes.drawerPaper,
            }}
            ModalProps={{
              keepMounted: true, // Better open performance on mobile.
            }}
          >
            {drawer}
          </Drawer>
        </Hidden>
        <Hidden xsDown implementation="css">
          <Drawer
            classes={{
              paper: classes.drawerPaper,
            }}
            variant="permanent"
            open
          >
            {drawer}
          </Drawer>
        </Hidden>
      </nav>
      <main className={classes.content} ref={mainEl}>
        <div className={classes.toolbar} />
        {!!props.tabs && <div className={classes.tabs} />}
        {props.children}
      </main>
    </div>
  );
};
