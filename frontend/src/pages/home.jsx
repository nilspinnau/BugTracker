import React from "react";
import Container from "react-bootstrap/esm/Container";

// This is our main page
class Home extends React.Component {

    __get() {
        let url = "/";
        let headers = {};
        let requestOptions = {
            method: "GET",
            headers: headers
        }
        return fetch(url, requestOptions)
            .then(response => console.log(response))
            .catch(error => console.log(error));
    }

    componentDidMount() {
        // GET request using fetch with error handling
        this.__get();
    }

    render() {
        return (
            <Container style={{ "textAlign": "center" }}>
                <h4>
                    This is our mainpage without any content
                </h4>
            </Container>
        );
    }
}

export default Home;
