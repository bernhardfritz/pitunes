import React from "react";
import { AppContext } from "./ResponsiveDrawer";

export class RootComponent extends React.Component {

    componentDidMount() {
        this.context.setTitle('piTunes');
    }
    
    render() {
        return (
            <div>{/* TODO */}</div>
        );
    }

}

RootComponent.contextType = AppContext;
