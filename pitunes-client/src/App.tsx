import React from 'react';
import './App.css';
import {
  BrowserRouter as Router,
  Switch,
  Route,
  Link
} from 'react-router-dom';
import GraphiQL from 'graphiql';

import 'graphiql/graphiql.min.css';

type AppState = {
  apiKey: string,
  responseText: string
}

class App extends React.Component<{}, AppState> {

  constructor(props: {}) {
    super(props);
    this.state = {
      apiKey: '',
      responseText: '',
    };

    this.handleChange = this.handleChange.bind(this);
    this.handleFiles = this.handleFiles.bind(this);
    this.graphQLFetcher = this.graphQLFetcher.bind(this);
  }

  handleChange(event: any) {
    this.setState({apiKey: event.target.value});
  }

  async handleFiles(event: any) {
    const formData = new FormData();
    for (const file of event.target.files) {
      formData.append("file", file);
    }
    const responseText = await fetch('https://localhost:8080/api/upload', {
      method: 'post',
      headers: {
        'Authorization': `Bearer ${this.state.apiKey}`,
      },
      body: formData
    }).then(response => response.text());
    this.setState({
      apiKey: this.state.apiKey,
      responseText
    });
  }

  graphQLFetcher(graphQLParams: any) {
    return fetch('https://localhost:8080/api/graphql', {
      method: 'post',
      headers: {
        'Content-Type': 'application/json',
        'Authorization': `Bearer ${this.state.apiKey}`
      },
      body: JSON.stringify(graphQLParams)
    }).then(response => response.json());
  }

  render() {
    return (
      <Router>
        <div className="App">
          <div className="App-header">
            <ul>
              <li>
                <Link to="/">Home</Link>
              </li>
              <li>
                <Link to="/graphiql">graphiql</Link>
              </li>
              <li>
                <Link to="/upload">upload</Link>
              </li>
            </ul>
            <label>API_KEY=
              <input type="password" value={this.state.apiKey} onChange={this.handleChange}></input>
            </label>
          </div>
          <div className="App-content">
            {/*
              A <Switch> looks through all its children <Route>
              elements and renders the first one whose path
              matches the current URL. Use a <Switch> any time
              you have multiple routes, but you want only one
              of them to render at a time
            */}
            <Switch>
              <Route exact path="/">
                <Home />
              </Route>
              <Route path="/graphiql">
                <GraphiQL fetcher={this.graphQLFetcher} />
              </Route>
              <Route path="/upload">
                <input type="file" onChange={this.handleFiles} multiple></input>
                <textarea value={this.state.responseText} readOnly></textarea>
              </Route>
            </Switch>
          </div>
        </div>
      </Router>
    );
  }

}

function Home() {
  return(
    <div>
      <h2>Home</h2>
    </div>
  );
}

export default App;