import { useScrollTrigger } from '@material-ui/core';
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
import LibraryMusicIcon from '@material-ui/icons/LibraryMusic';
import MenuIcon from '@material-ui/icons/Menu';
import PublishIcon from '@material-ui/icons/Publish';
import StorageIcon from '@material-ui/icons/Storage';
import React, { FunctionComponent } from 'react';
import { Link } from 'react-router-dom';
import { CSSTransition } from 'react-transition-group';
import { ListItemLink } from './ListItemLink';

export const drawerWidth = 240;

const useStyles = makeStyles((theme: Theme) =>
  createStyles({
    root: {
      display: 'flex',
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
  })
);

type ResponsiveDrawerProps = { title: string; tabs: any };

export const ResponsiveDrawer: FunctionComponent<ResponsiveDrawerProps> = (
  props
) => {
  const classes = useStyles();
  const theme = useTheme();
  const trigger = useScrollTrigger();
  const [mobileOpen, setMobileOpen] = React.useState(false);

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
        <ListItemLink
          to="/"
          primary="Library"
          icon={<LibraryMusicIcon />}
          onClick={handleClick}
        ></ListItemLink>
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
      <main className={classes.content}>
        <div className={classes.toolbar} />
        {!!props.tabs && <div className={classes.tabs} />}
        {props.children}
      </main>
    </div>
  );
};
