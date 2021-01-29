import {
  Collapse,
  Slide,
  Tab,
  Tabs,
  useScrollTrigger,
} from '@material-ui/core';
import AppBar from '@material-ui/core/AppBar';
import Divider from '@material-ui/core/Divider';
import Drawer from '@material-ui/core/Drawer';
import Hidden from '@material-ui/core/Hidden';
import IconButton from '@material-ui/core/IconButton';
import List from '@material-ui/core/List';
import {
  createStyles,
  Theme,
  WithStyles,
  withStyles,
} from '@material-ui/core/styles';
import Toolbar from '@material-ui/core/Toolbar';
import Typography from '@material-ui/core/Typography';
import AlbumIcon from '@material-ui/icons/Album';
import CategoryIcon from '@material-ui/icons/Category';
import LibraryMusicIcon from '@material-ui/icons/LibraryMusic';
import MenuIcon from '@material-ui/icons/Menu';
import MicIcon from '@material-ui/icons/Mic';
import MusicNoteIcon from '@material-ui/icons/MusicNote';
import PublishIcon from '@material-ui/icons/Publish';
import QueueMusicIcon from '@material-ui/icons/QueueMusic';
import StorageIcon from '@material-ui/icons/Storage';
import React from 'react';
import { Link, RouteComponentProps, withRouter } from 'react-router-dom';
import { ListItemLink } from './ListItemLink';
import './ResponsiveDrawer.css';

function CollapseOnScroll(props: any) {
  const { children, passthrough } = props;
  const trigger = useScrollTrigger();

  return passthrough ? children : <Collapse in={!trigger}>{children}</Collapse>;
}

function SlideOnScroll(props: any) {
  const { children, passthrough } = props;
  const trigger = useScrollTrigger();

  return passthrough ? (
    children
  ) : (
    <Slide appear={false} direction="down" in={!trigger}>
      {children}
    </Slide>
  );
}

export const drawerWidth = 240;

const styles = (theme: Theme) =>
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
      [theme.breakpoints.up('sm')]: {
        width: `calc(100% - ${drawerWidth}px)`,
        marginLeft: drawerWidth,
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
    tabs: {
      minHeight: 48,
    },
    drawerPaper: {
      width: drawerWidth,
    },
    content: {
      flexGrow: 1,
    },
    audio: {
      display: 'block',
      position: 'fixed',
      bottom: 0,
      width: '100%',
      zIndex: 9001,
      [theme.breakpoints.up('sm')]: {
        width: `calc(100% - ${drawerWidth}px)`,
      },
    },
  });

type ResponsiveDrawerProps = { title: string } & RouteComponentProps &
  WithStyles<typeof styles, true>;

type ResponsiveDrawerState = {
  mobileOpen: boolean;
};

class ResponsiveDrawer extends React.Component<
  ResponsiveDrawerProps,
  ResponsiveDrawerState
> {
  readonly tabs = [
    {
      label: 'Playlists',
      value: '/playlists',
    },
    {
      label: 'Artists',
      value: '/artists',
    },
    {
      label: 'Albums',
      value: '/albums',
    },
  ];

  constructor(props: ResponsiveDrawerProps) {
    super(props);
    this.state = {
      mobileOpen: false,
    };
  }

  readonly handleDrawerToggle = () => {
    this.setState((state) => ({ mobileOpen: !state.mobileOpen }));
  };

  readonly handleClick = () => {
    this.setState({ mobileOpen: false });
  };

  readonly handleTabChange = (
    event: React.ChangeEvent<{}>,
    newValue: string
  ) => {
    this.props.history.push(newValue);
  };

  readonly drawer = (
    <div>
      <div className={this.props.classes.toolbar}>
        <div className="logo-container">
          <Link to="/" className="logo-link" onClick={this.handleClick}>
            <img src="/logo192.png" className="logo" />
            <span>
              <span className="logo-text">piTunes</span>
              <sub>v{process.env.REACT_APP_VERSION}</sub>
            </span>
          </Link>
        </div>
      </div>
      <Divider />
      <List>
        <ListItemLink
          to="/playlists"
          primary="Library"
          icon={<LibraryMusicIcon />}
          onClick={this.handleClick}
        ></ListItemLink>
        <ListItemLink
          to="/albums"
          primary="Albums"
          icon={<AlbumIcon />}
          onClick={this.handleClick}
        ></ListItemLink>
        <ListItemLink
          to="/artists"
          primary="Artists"
          icon={<MicIcon />}
          onClick={this.handleClick}
        ></ListItemLink>
        <ListItemLink
          to="/genres"
          primary="Genres"
          icon={<CategoryIcon />}
          onClick={this.handleClick}
        ></ListItemLink>
        <ListItemLink
          to="/playlists"
          primary="Playlists"
          icon={<QueueMusicIcon />}
          onClick={this.handleClick}
        ></ListItemLink>
        <ListItemLink
          to="/tracks"
          primary="Tracks"
          icon={<MusicNoteIcon />}
          onClick={this.handleClick}
        ></ListItemLink>
      </List>
      <Divider />
      <List>
        <ListItemLink
          to="/upload"
          primary="Upload"
          icon={<PublishIcon />}
          onClick={this.handleClick}
        ></ListItemLink>
        <ListItemLink
          to="/graphiql"
          primary="GraphiQL"
          icon={<StorageIcon />}
          onClick={this.handleClick}
        ></ListItemLink>
      </List>
    </div>
  );

  readonly container = window !== undefined ? window.document.body : undefined;

  render() {
    const index = this.props.location.pathname.indexOf('/', 1);
    let selectedTab: string | boolean =
      index > -1
        ? this.props.location.pathname.substring(0, index)
        : this.props.location.pathname;
    const renderTabs = selectedTab !== '/upload' && selectedTab !== '/graphiql';
    if (!this.tabs.some((tab) => tab.value === selectedTab)) {
      selectedTab = false;
    }
    return (
      <div className={this.props.classes.root}>
        <SlideOnScroll passthrough={renderTabs}>
          <AppBar position="fixed" className={this.props.classes.appBar}>
            <CollapseOnScroll passthrough={!renderTabs}>
              <Toolbar>
                <IconButton
                  color="inherit"
                  aria-label="open drawer"
                  edge="start"
                  onClick={this.handleDrawerToggle}
                  className={this.props.classes.menuButton}
                >
                  <MenuIcon />
                </IconButton>
                <Typography variant="h6" noWrap>
                  {this.props.title}
                </Typography>
              </Toolbar>
            </CollapseOnScroll>
            {renderTabs && (
              <Tabs
                value={selectedTab}
                onChange={this.handleTabChange}
                indicatorColor="secondary"
                textColor="secondary"
                variant="fullWidth"
              >
                {this.tabs.map((tab, index) => (
                  <Tab key={index} label={tab.label} value={tab.value} />
                ))}
              </Tabs>
            )}
          </AppBar>
        </SlideOnScroll>
        <nav className={this.props.classes.drawer} aria-label="mailbox folders">
          {/* The implementation can be swapped with js to avoid SEO duplication of links. */}
          <Hidden smUp implementation="css">
            <Drawer
              container={this.container}
              variant="temporary"
              anchor={this.props.theme.direction === 'rtl' ? 'right' : 'left'}
              open={this.state.mobileOpen}
              onClose={this.handleDrawerToggle}
              classes={{
                paper: this.props.classes.drawerPaper,
              }}
              ModalProps={{
                keepMounted: true, // Better open performance on mobile.
              }}
            >
              {this.drawer}
            </Drawer>
          </Hidden>
          <Hidden xsDown implementation="css">
            <Drawer
              classes={{
                paper: this.props.classes.drawerPaper,
              }}
              variant="permanent"
              open
            >
              {this.drawer}
            </Drawer>
          </Hidden>
        </nav>
        <main className={this.props.classes.content}>
          <div className={this.props.classes.toolbar} />
          {renderTabs && <div className={this.props.classes.tabs} />}
          {this.props.children}
        </main>
      </div>
    );
  }
}

export default withStyles(styles, { withTheme: true })(
  withRouter(ResponsiveDrawer)
);
