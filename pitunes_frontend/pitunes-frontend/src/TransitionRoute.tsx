import React from 'react';
import { Route } from 'react-router-dom';
import { CSSTransition } from 'react-transition-group';
import './TransitionRoute.css';

export const TransitionRoute = ({ children, ...rest }: any) => (
  <Route {...rest}>
    {({ match }) => (
      <CSSTransition
        in={match != null}
        timeout={300}
        classNames="transition"
        unmountOnExit
      >
        <div className="transition">{children}</div>
      </CSSTransition>
    )}
  </Route>
);
