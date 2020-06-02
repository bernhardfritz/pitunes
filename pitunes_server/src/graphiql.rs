//! Utility module to generate a GraphiQL interface

/// Generate the HTML source to show a GraphiQL interface
pub fn graphiql_source(graphql_endpoint_url: &str) -> String {
    let stylesheet_source = r#"
    <style>
        html, body, #app {
            height: 100%;
            margin: 0;
            overflow: hidden;
            width: 100%;
        }
    </style>
    "#;
    let fetcher_source = r#"
    <script>
        function graphQLFetcher(params) {
            const headers = {
                'Accept': 'application/json',
                'Content-Type': 'application/json',
            };
            const apiKey = document.getElementById('api-key');
            if (apiKey) {
                headers['Authorization'] = `Bearer ${apiKey.value}`;
            }
            return fetch(GRAPHQL_URL, {
                method: 'POST',
                headers,
                credentials: 'include',
                body: JSON.stringify(params)
            }).then(function (response) {
                return response.text();
            }).then(function (body) {
                try {
                    return JSON.parse(body);
                } catch (error) {
                    return body;
                }
            });
        }

        function callback() {
            const topBar = document.querySelector('.topBar');
            const form = document.createElement('form');
            const label = document.createElement('label');
            label.setAttribute('for', 'api-key');
            label.textContent = 'API_KEY=';
            const input = document.createElement('input');
            input.setAttribute('id', 'api-key');
            input.setAttribute('name', 'api-key');
            input.setAttribute('type', 'password');
            const urlSearchParams = new URLSearchParams(window.location.search);
            const apiKey = urlSearchParams.get('api-key');
            if (apiKey) {
                input.value = apiKey;
            }
            urlSearchParams.delete('api-key');
            history.replaceState(null, '', window.location.href.split('?')[0] + urlSearchParams.toString())
            form.appendChild(label);
            form.appendChild(input);
            topBar.appendChild(form);
        }

        ReactDOM.render(
            React.createElement(GraphiQL, {
                fetcher: graphQLFetcher,
            }),
            document.querySelector('#app'),
            callback);
    </script>
    "#;

    format!(
        r#"
<!DOCTYPE html>
<html>
<head>
    <title>GraphQL</title>
    {stylesheet_source}
    <link rel="stylesheet" type="text/css" href="//cdn.jsdelivr.net/npm/graphiql@0.17.2/graphiql.min.css">
</head>
<body>
    <div id="app"></div>
    <script src="//cdnjs.cloudflare.com/ajax/libs/fetch/2.0.3/fetch.js"></script>
    <script src="//cdnjs.cloudflare.com/ajax/libs/react/16.10.2/umd/react.production.min.js"></script>
    <script src="//cdnjs.cloudflare.com/ajax/libs/react-dom/16.10.2/umd/react-dom.production.min.js"></script>
    <script src="//cdn.jsdelivr.net/npm/graphiql@0.17.2/graphiql.min.js"></script>
    <script>const GRAPHQL_URL = '{graphql_url}';</script>
    {fetcher_source}
</body>
</html>
"#,
        graphql_url = graphql_endpoint_url,
        stylesheet_source = stylesheet_source,
        fetcher_source = fetcher_source
    )
}
