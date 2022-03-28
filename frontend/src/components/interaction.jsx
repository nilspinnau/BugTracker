import React from "react";

class UserInfo extends React.Component {
    constructor(props) {
        super(props);
        this.state = {
            id: 0
        };
    }

    render() {
        return (
            <div className="UserInfo">
                <div className="UserInfo-name">
                    {this.props.author}
                </div>
            </div>
        );
    }
}

class Comments extends React.Component {
    constructor(props) {
        super(props);
        this.state = {
            id: 0
        };
    }

    render() {
        return (
            <div className="Comment">
              <UserInfo user={this.props.author} /><div className="Comment-text">
                {this.props.text}
              </div>
              <div className="Comment-date">
              </div>
            </div>
          );
    }
}

class Chat extends React.Component {
    constructor(props) {
        super(props);
        this.state = {
            id: 0
        };
    }
    
    render() {
        return(
            <div></div>
        );
    }
}

export default Chat;